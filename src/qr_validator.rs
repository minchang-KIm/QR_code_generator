use crate::error::{QrImageError, Result};
use image::{DynamicImage, GrayImage};
use log::{debug, info, warn};
use rqrr::PreparedImage;

pub struct QrValidator {
    max_attempts: u32,
}

impl QrValidator {
    pub fn new(max_attempts: u32) -> Self {
        Self { max_attempts }
    }

    /// Validate that QR code in image is readable and matches expected data
    pub fn validate(&self, image: &DynamicImage, expected_data: &str) -> Result<bool> {
        info!("Starting QR code validation");

        for attempt in 1..=self.max_attempts {
            debug!("Validation attempt {}/{}", attempt, self.max_attempts);

            match self.try_decode(image, attempt) {
                Ok(decoded_data) => {
                    info!("QR code decoded successfully");
                    debug!("Decoded data length: {}", decoded_data.len());

                    if decoded_data == expected_data {
                        info!("QR code validation successful - data matches");
                        return Ok(true);
                    } else {
                        warn!("QR code decoded but data mismatch");
                        debug!("Expected: {}", expected_data);
                        debug!("Got: {}", decoded_data);
                        return Err(QrImageError::ValidationError(
                            "Decoded data does not match expected data".to_string(),
                        ));
                    }
                }
                Err(e) => {
                    warn!("Attempt {} failed: {}", attempt, e);
                    if attempt == self.max_attempts {
                        return Err(QrImageError::ValidationError(format!(
                            "Failed to decode QR code after {} attempts",
                            self.max_attempts
                        )));
                    }
                }
            }
        }

        Err(QrImageError::ValidationError(
            "QR code validation failed".to_string(),
        ))
    }

    fn try_decode(&self, image: &DynamicImage, attempt: u32) -> Result<String> {
        // Convert to grayscale for better QR detection
        let gray_image = self.preprocess_image(image, attempt)?;

        // Prepare image for QR detection
        let mut prepared = PreparedImage::prepare(gray_image);

        // Find QR codes in image
        let grids = prepared.detect_grids();

        if grids.is_empty() {
            return Err(QrImageError::ValidationError(
                "No QR code detected in image".to_string(),
            ));
        }

        debug!("Detected {} QR code grid(s)", grids.len());

        // Try to decode each detected grid
        for (i, grid) in grids.iter().enumerate() {
            debug!("Attempting to decode grid {}", i + 1);

            match grid.decode() {
                Ok((meta, content)) => {
                    debug!("QR code decoded: version={:?}", meta.version);
                    // content is already a String in rqrr 0.7
                    return Ok(content);
                }
                Err(e) => {
                    debug!("Grid {} decode failed: {:?}", i + 1, e);
                }
            }
        }

        Err(QrImageError::ValidationError(
            "QR codes detected but none could be decoded".to_string(),
        ))
    }

    fn preprocess_image(&self, image: &DynamicImage, attempt: u32) -> Result<GrayImage> {
        let mut gray = image.to_luma8();

        // Apply different preprocessing strategies based on attempt
        match attempt {
            1 => {
                // First attempt: use original image
                debug!("Using original image");
            }
            2 => {
                // Second attempt: increase contrast
                debug!("Applying contrast enhancement");
                self.enhance_contrast(&mut gray);
            }
            3 => {
                // Third attempt: apply adaptive threshold
                debug!("Applying adaptive thresholding");
                self.adaptive_threshold(&mut gray);
            }
            _ => {
                // Additional attempts: try brightness adjustment
                debug!("Applying brightness adjustment");
                self.adjust_brightness(&mut gray, attempt as i32);
            }
        }

        Ok(gray)
    }

    fn enhance_contrast(&self, image: &mut GrayImage) {
        // Simple contrast stretching
        let (width, height) = image.dimensions();
        let mut min_val = 255u8;
        let mut max_val = 0u8;

        // Find min and max
        for y in 0..height {
            for x in 0..width {
                let pixel = image.get_pixel(x, y)[0];
                min_val = min_val.min(pixel);
                max_val = max_val.max(pixel);
            }
        }

        if max_val > min_val {
            let range = max_val - min_val;
            for y in 0..height {
                for x in 0..width {
                    let pixel = image.get_pixel(x, y)[0];
                    let normalized = ((pixel - min_val) as f32 / range as f32 * 255.0) as u8;
                    image.put_pixel(x, y, image::Luma([normalized]));
                }
            }
        }
    }

    fn adaptive_threshold(&self, image: &mut GrayImage) {
        let (width, height) = image.dimensions();
        let mut result = image.clone();

        // Simple adaptive thresholding
        let window_size = 15u32;
        let c = 10i32; // Constant subtracted from mean

        for y in 0..height {
            for x in 0..width {
                let mut sum = 0u32;
                let mut count = 0u32;

                // Calculate local mean
                for dy in 0..window_size {
                    for dx in 0..window_size {
                        let px = (x + dx).saturating_sub(window_size / 2);
                        let py = (y + dy).saturating_sub(window_size / 2);

                        if px < width && py < height {
                            sum += image.get_pixel(px, py)[0] as u32;
                            count += 1;
                        }
                    }
                }

                let mean = (sum / count.max(1)) as i32;
                let pixel = image.get_pixel(x, y)[0] as i32;
                let threshold = mean - c;

                let new_val = if pixel > threshold { 255 } else { 0 };
                result.put_pixel(x, y, image::Luma([new_val]));
            }
        }

        *image = result;
    }

    fn adjust_brightness(&self, image: &mut GrayImage, factor: i32) {
        let adjustment = (factor - 3) * 20; // -40, -20, 0, 20, 40, ...

        for pixel in image.pixels_mut() {
            let val = pixel[0] as i32 + adjustment;
            pixel[0] = val.clamp(0, 255) as u8;
        }
    }

    /// Quick check if image likely contains a readable QR code
    pub fn quick_check(&self, image: &DynamicImage) -> bool {
        let gray = image.to_luma8();
        let mut prepared = PreparedImage::prepare(gray);
        let grids = prepared.detect_grids();

        !grids.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use qrcode::QrCode;

    #[test]
    fn test_validate_simple_qr() {
        // Create a simple QR code
        let data = "https://example.com";
        let code = QrCode::new(data.as_bytes()).unwrap();
        let image = code.render::<image::Luma<u8>>().build();
        let dynamic = DynamicImage::ImageLuma8(image);

        let validator = QrValidator::new(3);
        let result = validator.validate(&dynamic, data);

        assert!(result.is_ok());
        assert!(result.unwrap());
    }
}
