use image::GenericImageView;

fn main() {
    let img = image::open("assets/your_image.png").unwrap(); // Update the path to your actual image file
    let (width, height) = img.dimensions();

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            let brightness = (0.3 * pixel[0] as f32 + 0.59 * pixel[1] as f32 + 0.11 * pixel[2] as f32) / 255.0;
            let ascii = match brightness {
                0.0..=0.2 => ' ',
                0.2..=0.4 => '.',
                0.4..=0.6 => '*',
                0.6..=0.8 => 'O',
                _ => '#',
            };
            print!("{}", ascii);
        }
        println!();
    }
}
