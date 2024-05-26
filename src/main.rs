use qrcode::QrCode;
use qrcode::render::svg;
use image::Luma;

fn main() {
    // Define the data for the QR code
    let data = "https://snowball-tree.tistory.com/";

    // Create the QR code
    let code = QrCode::new(data).unwrap();

    // Render the QR code into an image buffer
    let image = code.render::<Luma<u8>>().build();

    // Save the image to a file
    image.save("qrcode.png").unwrap();

    println!("QR code generated and saved as qrcode.png");
}