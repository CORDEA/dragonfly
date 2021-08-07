use clap::Clap;
use image::Pixel;
use std::cmp::Ordering;
use std::collections::HashSet;

#[derive(Clap)]
struct Opts {
    image: String,
    #[clap(long)]
    hex: bool,
    #[clap(long)]
    alpha: bool,
}

fn main() {
    let opts: Opts = Opts::parse();

    let raw = image::open(&opts.image).expect("Failed to open the image.");

    let rgba = raw.to_rgba8();
    let rgb = raw.to_rgb8();
    let image = if opts.alpha {
        rgba.pixels().map(|v| v.channels()).collect::<Vec<_>>()
    } else {
        rgb.pixels().map(|v| v.channels()).collect::<Vec<_>>()
    };

    let mut pixels = image
        .into_iter()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    pixels.sort_by(|a, b| cmp_rgba(a, b));

    if opts.hex {
        pixels
            .iter()
            .map(|p| {
                p.iter()
                    .map(|v| format!("{:02X}", v))
                    .collect::<Vec<_>>()
                    .join("")
            })
            .for_each(|p| println!("#{}", p));
        return;
    }

    pixels
        .iter()
        .map(|p| {
            p.iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        })
        .for_each(|p| println!("({})", p));
}

fn cmp_rgba(a: &[u8], b: &[u8]) -> Ordering {
    let a_sum = a.iter().map(|v| u16::from(*v)).sum::<u16>();
    let b_sum = b.iter().map(|v| u16::from(*v)).sum::<u16>();
    if a_sum == b_sum {
        return Ordering::Equal;
    }
    return if a_sum > b_sum {
        Ordering::Greater
    } else {
        Ordering::Less
    };
}
