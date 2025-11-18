pub mod config;
pub mod error;
pub mod image_provider;
pub mod qr_embedder;
pub mod qr_validator;

use config::Config;
use error::Result;
use image::DynamicImage;
use image_provider::ImageProvider;
use log::{error, info};
use qr_embedder::QrEmbedder;
use qr_validator::QrValidator;

/// Main orchestrator for QR code image generation
pub struct QrImageGenerator {
    config: Config,
    provider: ImageProvider,
    embedder: QrEmbedder,
    validator: QrValidator,
}

impl QrImageGenerator {
    /// Create a new QR image generator with the given configuration
    pub fn new(config: Config) -> Self {
        let provider = ImageProvider::new(config.clone());
        let embedder = QrEmbedder::new(config.clone());
        let validator = QrValidator::new(config.max_validation_attempts);

        Self {
            config,
            provider,
            embedder,
            validator,
        }
    }

    /// Create a QR code image from a keyword and data
    ///
    /// # Arguments
    /// * `keyword` - Search keyword for background image
    /// * `qr_data` - Data to encode in QR code (URL, text, etc.)
    ///
    /// # Returns
    /// * `Result<DynamicImage>` - Generated image with embedded QR code
    ///
    /// # Process
    /// 1. Fetch/generate background image based on keyword
    /// 2. Generate and embed QR code
    /// 3. Validate QR code is readable
    /// 4. Return validated image
    pub fn generate(&self, keyword: &str, qr_data: &str) -> Result<DynamicImage> {
        info!("Starting QR image generation");
        info!("Keyword: {}", keyword);
        info!("QR data length: {}", qr_data.len());

        // Step 1: Fetch background image
        info!("Fetching background image...");
        let background = self.provider.fetch_image(keyword)?;
        info!(
            "Background image fetched: {}x{}",
            background.width(),
            background.height()
        );

        // Step 2: Embed QR code
        info!("Embedding QR code...");
        let image_with_qr = self.embedder.embed_qr_code(background, qr_data)?;
        info!("QR code embedded successfully");

        // Step 3: Validate QR code
        info!("Validating QR code readability...");
        match self.validator.validate(&image_with_qr, qr_data) {
            Ok(true) => {
                info!("✓ QR code validation successful");
                Ok(image_with_qr)
            }
            Ok(false) => {
                error!("✗ QR code validation failed - readable but data mismatch");
                Err(error::QrImageError::QrNotReadable)
            }
            Err(e) => {
                error!("✗ QR code validation failed: {}", e);
                Err(error::QrImageError::QrNotReadable)
            }
        }
    }

    /// Generate and save QR code image to file
    ///
    /// # Arguments
    /// * `keyword` - Search keyword for background image
    /// * `qr_data` - Data to encode in QR code
    /// * `output_path` - Path to save the generated image
    ///
    /// # Returns
    /// * `Result<()>` - Success or error
    pub fn generate_and_save(
        &self,
        keyword: &str,
        qr_data: &str,
        output_path: &str,
    ) -> Result<()> {
        let image = self.generate(keyword, qr_data)?;

        info!("Saving image to: {}", output_path);
        image.save(output_path)?;
        info!("✓ Image saved successfully");

        Ok(())
    }

    /// Quick validation check without full generation
    pub fn quick_validate(&self, image: &DynamicImage) -> bool {
        self.validator.quick_check(image)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generator_creation() {
        let config = Config::default();
        let generator = QrImageGenerator::new(config);
        assert!(true); // Basic instantiation test
    }
}
