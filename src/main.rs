use std::collections::HashSet;
use std::env;

fn main() {
    let path = env::args().nth(1).expect("Target image is required.");
    let image = image::open(&path)
        .expect("Failed to open the image.")
        .to_rgba8();
    let pixels = image.pixels().collect::<HashSet<_>>();

    for pixel in pixels {
        println!("({}, {}, {}, {})", pixel[0], pixel[1], pixel[2], pixel[3]);
    }
}
