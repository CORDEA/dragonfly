use clap::Clap;
use image::Pixel;
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
        .map(|v| v.channels())
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
