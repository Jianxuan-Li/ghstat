use magick_rust::{magick_wand_genesis, DrawingWand, MagickWand, PixelWand};
use std::sync::Once;

static START: Once = Once::new();

pub fn draw_png(num: i32) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // Initialize MagickWand
    START.call_once(|| {
        magick_wand_genesis();
    });
    let mut wand = MagickWand::new();
    let mut pw = PixelWand::new();

    // Create MagickWand
    wand.new_image(100, 16, &pw).unwrap();

    // Create DrawingWand
    let mut draw_wand = DrawingWand::new();

    // set fill color to 226, 226, 226
    pw.set_color("#E2E2E2").unwrap();

    // Draw a rectangle (100x16) with the fill color (white)
    draw_wand.set_fill_color(&pw);
    draw_wand.draw_rectangle(0.0, 0.0, 100.0, 16.0);

    // Set the font and font size
    draw_wand.set_font("Arial").unwrap();
    draw_wand.set_font_size(12.0);

    // Set the text color to black
    pw.set_color("#000").unwrap();
    draw_wand.set_fill_color(&pw);

    // Add text "Hello World" at position (0, 0)
    draw_wand.draw_annotation(12.0, 12.0, &format!("Visits: {}", num)).unwrap();

    // Draw on the MagickWand
    let _ = wand.draw_image(&draw_wand);

    // Save the image to a buffer
    let buffer = wand.write_image_blob("PNG").unwrap();

    Ok(buffer)
}
