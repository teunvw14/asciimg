extern crate image;

use image::{GenericImageView, imageops};
use std::env;

fn main() {
    
    if env::args().len() < 4 {
        panic!("Too few arguments! Usage: asciimg [path-to-image] [width] [height]");
    }

    // Parse the arguments passed to the program:
    let image_path = env::args().nth(1).unwrap();
    let img = image::open(image_path).expect("Expected valid path.");
    let w: u32 = env::args().nth(2).unwrap().parse()
                .expect(&format!("Was expecting an integer width, got {:?}", env::args().nth(2)));
    let h: u32 = env::args().nth(3).unwrap().parse()
                .expect(&format!("Was expecting an integer height, got {:?}", env::args().nth(3)));
    
    // Resize the image to the requested size.
    let img_resized = img.resize_exact(w, h, imageops::FilterType::Nearest);
    
    // Build up the string by calculating the brightness for each pixel and adding
    // ascii characters that correspond to that brightness. Brighter pixels are mapped
    // to more screen-filling characters to give the illusion of brightness, while darker
    // pixels are mapped to less screen-filling pixels to give the illusion of darkness.
    let mut result: String = "".to_string();
    for y in 0..img_resized.height() {
        for x in 0..img_resized.width() {
            let pixel_values = img_resized.get_pixel(x, y).0;
            if pixel_values[3] != 255 {
                println!("pixel is {:?}", pixel_values);
            }
            let brightness: u32 = pixel_values[0] as u32 + pixel_values[1] as u32 + pixel_values[2] as u32;
            result = result + match brightness {
                32..=191  => "`",
                192..=287 => "-",
                288..=383 => "=",
                384..=479 => "+",
                480..=575 => "$",
                576..=671 => "#",
                672..=765 => "@",
                _ => " "
            }
        }
        result = result + "\n";
    }
    println!("\n{}\n", result);
}
