use image::GenericImageView;
use thiserror::Error;

/// This program loads an image, converts it to ASCII art, and prints it to the terminal.

const ASCII_SPACE: char = ' ';
const ASCII_DOT: char = '.';
const ASCII_STAR: char = '*';
const ASCII_O: char = 'O';
const ASCII_HASH: char = '#';

#[derive(Error, Debug)]
enum ImageError {
    #[error("Failed to load image: {0}")]
    LoadError(#[from] image::ImageError),
}

fn load_image(path: &str) -> Result<image::DynamicImage, ImageError> {
    let img = image::open(path)?;
    Ok(img)
}

fn convert_to_ascii(img: &image::DynamicImage) {
    let (width, height) = img.dimensions();

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            let brightness = (0.3 * pixel[0] as f32 + 0.59 * pixel[1] as f32 + 0.11 * pixel[2] as f32) / 255.0;
            let ascii = match brightness {
                0.0..=0.2 => ASCII_SPACE,
                0.2..=0.4 => ASCII_DOT,
                0.4..=0.6 => ASCII_STAR,
                0.6..=0.8 => ASCII_O,
                _ => ASCII_HASH,
            };
            print!("{}", ascii);
        }
        println!();
    }
}

fn main() -> Result<(), ImageError> {
    let img = load_image("assets/your_image.png")?;
    convert_to_ascii(&img);
    Ok(())
}
