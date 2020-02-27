use std::env;
use std::fs;

use image::{DynamicImage, ImageOutputFormat, RgbImage};

fn main() {
    let input = env::args().nth(1).expect("input filename");
    let buf = fs::read(&input).expect("reading");

    let pixels = libavif::decode_rgb(&buf).expect("decoding");
    eprintln!("w={}, h={}", pixels.width(), pixels.height());

    let mut img = RgbImage::new(pixels.width(), pixels.height());

    for y in 0..img.height() {
        for x in 0..img.width() {
            let (r, g, b, _a) = pixels.pixel(x, y);
            img.put_pixel(x, y, [r, g, b].into());
        }
    }

    let img = DynamicImage::ImageRgb8(img);
    img.write_to(&mut std::io::stdout(), ImageOutputFormat::Png)
        .expect("out");
}
