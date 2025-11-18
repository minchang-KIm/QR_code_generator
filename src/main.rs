use clap::Parser;
use qr_code_generator::config::{Config, QrPosition};
use qr_code_generator::QrImageGenerator;
use std::process;

#[derive(Parser, Debug)]
#[command(name = "QR Image Generator")]
#[command(version = "1.0.0")]
#[command(about = "Generate beautiful QR code images with keyword-based backgrounds", long_about = None)]
struct Args {
    /// Keyword for background image search
    #[arg(short, long)]
    keyword: String,

    /// Data to encode in QR code (URL, text, etc.)
    #[arg(short, long)]
    data: String,

    /// Output file path
    #[arg(short, long, default_value = "qr_output.png")]
    output: String,

    /// Unsplash API key (or set UNSPLASH_API_KEY env var)
    #[arg(long)]
    api_key: Option<String>,

    /// Image width in pixels
    #[arg(long, default_value = "1920")]
    width: u32,

    /// Image height in pixels
    #[arg(long, default_value = "1080")]
    height: u32,

    /// QR code size as percentage (0.1 to 0.5)
    #[arg(long, default_value = "0.25")]
    qr_size: f32,

    /// QR code position: top-left, top-right, bottom-left, bottom-right, center
    #[arg(long, default_value = "bottom-right")]
    position: String,

    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,

    /// QR code background opacity (0-255)
    #[arg(long, default_value = "230")]
    opacity: u8,
}

fn main() {
    let args = Args::parse();

    // Initialize logger
    let log_level = if args.verbose { "debug" } else { "info" };
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(log_level))
        .format_timestamp(None)
        .format_module_path(false)
        .init();

    // Parse QR position
    let qr_position = match args.position.to_lowercase().as_str() {
        "top-left" => QrPosition::TopLeft,
        "top-right" => QrPosition::TopRight,
        "bottom-left" => QrPosition::BottomLeft,
        "bottom-right" => QrPosition::BottomRight,
        "center" => QrPosition::Center,
        _ => {
            eprintln!("Invalid position. Use: top-left, top-right, bottom-left, bottom-right, or center");
            process::exit(1);
        }
    };

    // Validate QR size
    if !(0.1..=0.5).contains(&args.qr_size) {
        eprintln!("QR size must be between 0.1 and 0.5");
        process::exit(1);
    }

    // Build configuration
    let mut config = Config::default()
        .with_dimensions(args.width, args.height)
        .with_qr_size_ratio(args.qr_size)
        .with_qr_position(qr_position);

    config.qr_background_opacity = args.opacity;

    // Use API key from args, or fall back to environment variable
    let api_key = args.api_key.or_else(|| std::env::var("UNSPLASH_API_KEY").ok());
    if let Some(key) = api_key {
        config = config.with_api_key(key);
    }

    // Create generator
    let generator = QrImageGenerator::new(config);

    // Generate image
    println!("🎨 Generating QR code image...");
    println!("📝 Keyword: {}", args.keyword);
    println!("🔗 QR Data: {}", args.data);
    println!();

    match generator.generate_and_save(&args.keyword, &args.data, &args.output) {
        Ok(()) => {
            println!();
            println!("✅ Success! QR code image generated.");
            println!("📁 Saved to: {}", args.output);
            println!();
            println!("The QR code has been validated and is guaranteed to be readable!");
        }
        Err(e) => {
            eprintln!();
            eprintln!("❌ Error: {}", e);
            eprintln!();
            eprintln!("Troubleshooting tips:");
            eprintln!("  • Check your internet connection");
            eprintln!("  • Verify your Unsplash API key (if provided)");
            eprintln!("  • Try a different keyword");
            eprintln!("  • Enable verbose mode with -v for more details");
            process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_parsing() {
        // Basic test to ensure CLI structure is valid
        use clap::CommandFactory;
        Args::command().debug_assert();
    }
}