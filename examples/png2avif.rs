use std::env;
use std::io::{self, Write};

fn main() {
    let input = env::args().nth(1).expect("input png");

    let png = match image::open(&input).expect("opening") {
        image::DynamicImage::ImageRgb8(image) => image,
        _ => panic!("image type not supported"),
    };

    let rows = png
        .rows()
        .map(|row| row.map(|c| (c[0], c[1], c[2])).collect());

    let data = libavif::encode_rgb(png.width(), png.height(), rows, 0).expect("encoding avif");

    io::stdout().write_all(&data).expect("output avif");
}
