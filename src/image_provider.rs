use crate::config::Config;
use crate::error::{QrImageError, Result};
use image::{DynamicImage, ImageFormat};
use log::{debug, info, warn};
use serde::Deserialize;

const UNSPLASH_API_URL: &str = "https://api.unsplash.com/photos/random";
const FALLBACK_IMAGE_URL: &str = "https://source.unsplash.com/random";

#[derive(Debug, Deserialize)]
struct UnsplashResponse {
    urls: UnsplashUrls,
    description: Option<String>,
    alt_description: Option<String>,
}

#[derive(Debug, Deserialize)]
struct UnsplashUrls {
    raw: String,
    full: String,
    regular: String,
    small: String,
}

pub struct ImageProvider {
    config: Config,
    client: reqwest::blocking::Client,
}

impl ImageProvider {
    pub fn new(config: Config) -> Self {
        let client = reqwest::blocking::Client::builder()
            .user_agent("QR-Image-Generator/1.0")
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        Self { config, client }
    }

    /// Fetch an image based on a keyword
    pub fn fetch_image(&self, keyword: &str) -> Result<DynamicImage> {
        info!("Fetching image for keyword: {}", keyword);

        // Try Unsplash API first if key is available
        if let Some(api_key) = &self.config.unsplash_api_key {
            match self.fetch_from_unsplash(keyword, api_key) {
                Ok(img) => {
                    info!("Successfully fetched image from Unsplash");
                    return Ok(img);
                }
                Err(e) => {
                    warn!("Unsplash API failed: {}, trying fallback", e);
                }
            }
        } else {
            warn!("No Unsplash API key provided, using fallback");
        }

        // Fallback to public Unsplash source
        self.fetch_fallback_image(keyword)
    }

    fn fetch_from_unsplash(&self, keyword: &str, api_key: &str) -> Result<DynamicImage> {
        debug!("Requesting from Unsplash API with keyword: {}", keyword);

        let response = self
            .client
            .get(UNSPLASH_API_URL)
            .query(&[
                ("query", keyword),
                ("orientation", "landscape"),
                ("content_filter", "high"),
            ])
            .header("Authorization", format!("Client-ID {}", api_key))
            .send()?;

        if !response.status().is_success() {
            return Err(QrImageError::ApiError(format!(
                "Unsplash API returned status: {}",
                response.status()
            )));
        }

        let unsplash_data: UnsplashResponse = response.json()?;
        debug!("Image description: {:?}", unsplash_data.description);

        // Use 'regular' size URL with custom dimensions
        let image_url = format!(
            "{}&w={}&h={}&fit=crop",
            unsplash_data.urls.raw, self.config.image_width, self.config.image_height
        );

        self.download_image(&image_url)
    }

    fn fetch_fallback_image(&self, keyword: &str) -> Result<DynamicImage> {
        info!("Using fallback image source");

        let image_url = format!(
            "{}/?{}",
            FALLBACK_IMAGE_URL,
            keyword.replace(' ', "+")
        );

        self.download_image(&image_url)
            .or_else(|_| {
                warn!("Fallback failed, generating solid color image");
                self.generate_placeholder_image(keyword)
            })
    }

    fn download_image(&self, url: &str) -> Result<DynamicImage> {
        debug!("Downloading image from: {}", url);

        let response = self.client.get(url).send()?;

        if !response.status().is_success() {
            return Err(QrImageError::ApiError(format!(
                "Image download failed with status: {}",
                response.status()
            )));
        }

        let bytes = response.bytes()?;
        let img = image::load_from_memory(&bytes)
            .or_else(|_| {
                // Try to parse as specific format
                image::load_from_memory_with_format(&bytes, ImageFormat::Jpeg)
                    .or_else(|_| image::load_from_memory_with_format(&bytes, ImageFormat::Png))
            })
            .map_err(|e| QrImageError::ProviderError(format!("Failed to decode image: {}", e)))?;

        // Resize to target dimensions if needed
        let resized = img.resize_exact(
            self.config.image_width,
            self.config.image_height,
            image::imageops::FilterType::Lanczos3,
        );

        Ok(resized)
    }

    fn generate_placeholder_image(&self, keyword: &str) -> Result<DynamicImage> {
        info!("Generating placeholder image for: {}", keyword);

        // Generate a color based on keyword hash
        let hash = keyword.bytes().fold(0u32, |acc, b| acc.wrapping_add(b as u32));
        let r = ((hash * 137) % 256) as u8;
        let g = ((hash * 193) % 256) as u8;
        let b = ((hash * 241) % 256) as u8;

        let mut img = image::RgbImage::new(self.config.image_width, self.config.image_height);

        // Create gradient effect
        for (x, _y, pixel) in img.enumerate_pixels_mut() {
            let factor = (x as f32 / self.config.image_width as f32) * 0.3 + 0.7;
            *pixel = image::Rgb([
                (r as f32 * factor) as u8,
                (g as f32 * factor) as u8,
                (b as f32 * factor) as u8,
            ]);
        }

        Ok(DynamicImage::ImageRgb8(img))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_placeholder_generation() {
        let config = Config::default();
        let provider = ImageProvider::new(config);
        let img = provider.generate_placeholder_image("test").unwrap();
        assert_eq!(img.width(), 1920);
        assert_eq!(img.height(), 1080);
    }
}
