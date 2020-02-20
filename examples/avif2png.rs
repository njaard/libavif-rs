use std::env;
use std::fs::File;
use std::io::Read;

use image::{DynamicImage, ImageOutputFormat, RgbImage};

fn main() {
    let input = env::args().nth(1).expect("input filename");
    let mut avifdata = vec![];
    let mut f = File::open(&input).expect("opening");
    f.read_to_end(&mut avifdata).expect("reading");

    let pixels = libavif::decode_rgb(&avifdata).expect("decoding");
    eprintln!("w={}, h={}", pixels.width(), pixels.height());

    let mut im = RgbImage::new(pixels.width(), pixels.height());

    for y in 0..im.height() {
        for x in 0..im.width() {
            let (r, g, b, _a) = pixels.pixel(x, y);
            im.put_pixel(x, y, [r, g, b].into());
        }
    }

    let im = DynamicImage::ImageRgb8(im);

    im.write_to(&mut std::io::stdout(), ImageOutputFormat::Png)
        .expect("out");
}
