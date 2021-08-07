use image::{Pixel, Rgba};
use std::cmp::Ordering;
use std::collections::HashSet;
use std::env;

fn main() {
    let path = env::args().nth(1).expect("Target image is required.");
    let image = image::open(&path)
        .expect("Failed to open the image.")
        .to_rgba8();
    let mut pixels = image
        .pixels()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    pixels.sort_by(|a, b| cmp_rgba(a, b));

    for pixel in pixels {
        println!("({}, {}, {}, {})", pixel[0], pixel[1], pixel[2], pixel[3]);
    }
}

fn cmp_rgba(a: &Rgba<u8>, b: &Rgba<u8>) -> Ordering {
    let a_sum = a.channels().iter().map(|v| u16::from(*v)).sum::<u16>();
    let b_sum = b.channels().iter().map(|v| u16::from(*v)).sum::<u16>();
    if a_sum == b_sum {
        return Ordering::Equal;
    }
    return if a_sum > b_sum {
        Ordering::Greater
    } else {
        Ordering::Less
    };
}
