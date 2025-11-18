use thiserror::Error;

#[derive(Error, Debug)]
pub enum QrImageError {
    #[error("Image processing error: {0}")]
    ImageError(#[from] image::ImageError),

    #[error("QR code generation error: {0}")]
    QrCodeError(#[from] qrcode::types::QrError),

    #[error("HTTP request error: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("QR code validation failed: {0}")]
    ValidationError(String),

    #[error("Image provider error: {0}")]
    ProviderError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Invalid image format or dimensions")]
    InvalidImage,

    #[error("QR code not readable after embedding")]
    QrNotReadable,

    #[error("API error: {0}")]
    ApiError(String),
}

pub type Result<T> = std::result::Result<T, QrImageError>;
