use crate::config::{Config, QrPosition};
use crate::error::Result;
use image::{DynamicImage, Rgba, RgbaImage};
use log::{debug, info};
use qrcode::QrCode;

pub struct QrEmbedder {
    config: Config,
}

impl QrEmbedder {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// Generate QR code and embed it into the background image
    pub fn embed_qr_code(&self, background: DynamicImage, data: &str) -> Result<DynamicImage> {
        info!("Embedding QR code with data length: {}", data.len());

        // Generate QR code
        let qr_code = QrCode::new(data.as_bytes())?;
        debug!("QR code generated successfully");

        // Calculate QR code size
        let qr_size = self.calculate_qr_size(&background);
        debug!("QR code size: {}x{}", qr_size, qr_size);

        // Render QR code to image with padding and background
        let qr_image = self.render_qr_code(&qr_code, qr_size)?;
        debug!("QR code rendered to image");

        // Calculate position
        let (x, y) = self.calculate_position(&background, &qr_image);
        debug!("QR code position: ({}, {})", x, y);

        // Overlay QR code onto background
        let result = self.overlay_qr_code(background, qr_image, x, y)?;
        info!("QR code embedded successfully");

        Ok(result)
    }

    fn calculate_qr_size(&self, background: &DynamicImage) -> u32 {
        let min_dimension = background.width().min(background.height());
        let size = (min_dimension as f32 * self.config.qr_size_ratio) as u32;

        // Ensure minimum size for readability
        size.max(200).min(800)
    }

    fn render_qr_code(&self, qr_code: &QrCode, target_size: u32) -> Result<RgbaImage> {
        // Render QR code as a simple black and white image
        let qr_raw = qr_code.render::<image::Luma<u8>>().build();

        // Calculate padding (10% of target size)
        let padding = (target_size as f32 * 0.1) as u32;
        let qr_content_size = target_size - (padding * 2);

        // Resize QR code to fit within padded area
        let qr_resized = image::imageops::resize(
            &qr_raw,
            qr_content_size,
            qr_content_size,
            image::imageops::FilterType::Nearest, // Use Nearest for crisp QR codes
        );

        // Create final image with white background and padding
        let mut qr_with_bg = RgbaImage::new(target_size, target_size);

        // Fill with semi-transparent white background
        for pixel in qr_with_bg.pixels_mut() {
            *pixel = Rgba([255, 255, 255, self.config.qr_background_opacity]);
        }

        // Copy QR code to center with padding
        for (x, y, pixel) in qr_resized.enumerate_pixels() {
            let luminance = pixel[0];
            let alpha = if luminance < 128 { 255 } else { self.config.qr_background_opacity };
            let color = if luminance < 128 { 0 } else { 255 };

            qr_with_bg.put_pixel(
                x + padding,
                y + padding,
                Rgba([color, color, color, alpha]),
            );
        }

        // Add a thin border for better visibility
        self.add_border(&mut qr_with_bg, padding / 2);

        Ok(qr_with_bg)
    }

    fn add_border(&self, image: &mut RgbaImage, border_width: u32) {
        let (width, height) = image.dimensions();
        let border_color = Rgba([200, 200, 200, self.config.qr_background_opacity]);

        // Top and bottom borders
        for x in 0..width {
            for i in 0..border_width {
                if i < height {
                    image.put_pixel(x, i, border_color);
                    if height > i {
                        image.put_pixel(x, height - 1 - i, border_color);
                    }
                }
            }
        }

        // Left and right borders
        for y in 0..height {
            for i in 0..border_width {
                if i < width {
                    image.put_pixel(i, y, border_color);
                    if width > i {
                        image.put_pixel(width - 1 - i, y, border_color);
                    }
                }
            }
        }
    }

    fn calculate_position(&self, background: &DynamicImage, qr_image: &RgbaImage) -> (u32, u32) {
        let bg_width = background.width();
        let bg_height = background.height();
        let qr_width = qr_image.width();
        let qr_height = qr_image.height();

        let margin = 30u32; // Margin from edges

        match self.config.qr_position {
            QrPosition::TopLeft => (margin, margin),
            QrPosition::TopRight => (bg_width.saturating_sub(qr_width + margin), margin),
            QrPosition::BottomLeft => (margin, bg_height.saturating_sub(qr_height + margin)),
            QrPosition::BottomRight => (
                bg_width.saturating_sub(qr_width + margin),
                bg_height.saturating_sub(qr_height + margin),
            ),
            QrPosition::Center => (
                (bg_width.saturating_sub(qr_width)) / 2,
                (bg_height.saturating_sub(qr_height)) / 2,
            ),
        }
    }

    fn overlay_qr_code(
        &self,
        background: DynamicImage,
        qr_image: RgbaImage,
        x: u32,
        y: u32,
    ) -> Result<DynamicImage> {
        let mut bg_rgba = background.to_rgba8();

        // Alpha blending
        for (qr_x, qr_y, qr_pixel) in qr_image.enumerate_pixels() {
            let bg_x = x + qr_x;
            let bg_y = y + qr_y;

            if bg_x < bg_rgba.width() && bg_y < bg_rgba.height() {
                let bg_pixel = bg_rgba.get_pixel(bg_x, bg_y);
                let blended = alpha_blend(*bg_pixel, *qr_pixel);
                bg_rgba.put_pixel(bg_x, bg_y, blended);
            }
        }

        Ok(DynamicImage::ImageRgba8(bg_rgba))
    }
}

fn alpha_blend(bg: Rgba<u8>, fg: Rgba<u8>) -> Rgba<u8> {
    let alpha_fg = fg[3] as f32 / 255.0;
    let alpha_bg = bg[3] as f32 / 255.0;

    // Alpha compositing formula
    let alpha_out = alpha_fg + alpha_bg * (1.0 - alpha_fg);

    if alpha_out == 0.0 {
        return Rgba([0, 0, 0, 0]);
    }

    let r = ((fg[0] as f32 * alpha_fg + bg[0] as f32 * alpha_bg * (1.0 - alpha_fg)) / alpha_out) as u8;
    let g = ((fg[1] as f32 * alpha_fg + bg[1] as f32 * alpha_bg * (1.0 - alpha_fg)) / alpha_out) as u8;
    let b = ((fg[2] as f32 * alpha_fg + bg[2] as f32 * alpha_bg * (1.0 - alpha_fg)) / alpha_out) as u8;
    let a = (alpha_out * 255.0) as u8;

    Rgba([r, g, b, a])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alpha_blend() {
        let bg = Rgba([100, 100, 100, 255]);
        let fg = Rgba([255, 255, 255, 128]);
        let result = alpha_blend(bg, fg);

        // Result should be between bg and fg
        assert!(result[0] > bg[0] && result[0] < fg[0]);
    }

    #[test]
    fn test_calculate_qr_size() {
        let config = Config::default();
        let embedder = QrEmbedder::new(config);
        let img = DynamicImage::new_rgb8(1920, 1080);
        let size = embedder.calculate_qr_size(&img);

        assert!(size >= 200 && size <= 800);
    }
}
