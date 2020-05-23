use std::env;
use std::io::{self, Write};

use image::DynamicImage;

fn main() {
    let input = env::args().nth(1).expect("input png");
    let img = image::open(&input).expect("opening");

    let data = match img {
        DynamicImage::ImageRgb8(img) => {
            let rgb = img.as_flat_samples();
            libavif::encode_rgb8(img.width(), img.height(), rgb.as_slice()).expect("encoding avif")
        }
        _ => panic!("image type not supported"),
    };

    io::stdout()
        .write_all(data.as_slice())
        .expect("output avif");
}
