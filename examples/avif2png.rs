use std::env;
use std::fs;

use image::{DynamicImage, ImageBuffer, ImageOutputFormat};

fn main() {
    let input = env::args().nth(1).expect("input filename");
    let buf = fs::read(&input).expect("reading");

    let pixels = libavif::decode_rgb(&buf).expect("decoding");
    eprintln!("w={}, h={}", pixels.width(), pixels.height());

    let buffer = ImageBuffer::from_vec(pixels.width(), pixels.height(), pixels.to_vec())
        .expect("pixels doesn't fit image::ImageBuffer");

    let img = DynamicImage::ImageRgba8(buffer);
    img.write_to(&mut std::io::stdout(), ImageOutputFormat::Png)
        .expect("out");
}
