use std::{env, fs};

use image::{codecs::png::PngEncoder, ColorType, ImageEncoder};

fn main() {
    let input = env::args().nth(1).expect("input filename");
    let buf = fs::read(&input).expect("reading");

    let pixels = libavif::decode_rgb(&buf).expect("decoding");
    eprintln!("w={}, h={}", pixels.width(), pixels.height());

    let encoder = PngEncoder::new(std::io::stdout());
    encoder
        .write_image(&pixels, pixels.width(), pixels.height(), ColorType::Rgba8)
        .expect("out");
}
