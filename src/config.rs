use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Unsplash API access key (get from https://unsplash.com/developers)
    pub unsplash_api_key: Option<String>,

    /// Default image width
    pub image_width: u32,

    /// Default image height
    pub image_height: u32,

    /// QR code size as percentage of image size (0.0 to 1.0)
    pub qr_size_ratio: f32,

    /// QR code position: TopLeft, TopRight, BottomLeft, BottomRight, Center
    pub qr_position: QrPosition,

    /// Maximum validation attempts
    pub max_validation_attempts: u32,

    /// QR code background opacity (0-255)
    pub qr_background_opacity: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum QrPosition {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Center,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            unsplash_api_key: env::var("UNSPLASH_API_KEY").ok(),
            image_width: 1920,
            image_height: 1080,
            qr_size_ratio: 0.25,
            qr_position: QrPosition::BottomRight,
            max_validation_attempts: 3,
            qr_background_opacity: 230,
        }
    }
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_api_key(mut self, key: String) -> Self {
        self.unsplash_api_key = Some(key);
        self
    }

    pub fn with_dimensions(mut self, width: u32, height: u32) -> Self {
        self.image_width = width;
        self.image_height = height;
        self
    }

    pub fn with_qr_size_ratio(mut self, ratio: f32) -> Self {
        self.qr_size_ratio = ratio.clamp(0.1, 0.5);
        self
    }

    pub fn with_qr_position(mut self, position: QrPosition) -> Self {
        self.qr_position = position;
        self
    }
}
