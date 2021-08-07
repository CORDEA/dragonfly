use clap::Clap;
use image::{Pixel, Rgba};
use std::cmp::Ordering;
use std::collections::HashSet;

#[derive(Clap)]
struct Opts {
    image: String,
    #[clap(long)]
    hex: bool,
}

fn main() {
    let opts: Opts = Opts::parse();

    let image = image::open(&opts.image)
        .expect("Failed to open the image.")
        .to_rgba8();
    let mut pixels = image
        .pixels()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    pixels.sort_by(|a, b| cmp_rgba(a, b));

    for pixel in pixels {
        println!(
            "({})",
            pixel
                .channels()
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        );
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
