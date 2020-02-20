use std::io::Read;

fn main() {
    let input = std::env::args().nth(1).expect("input filename");
    let mut avifdata = vec![];
    let mut f = std::fs::File::open(&input).expect("opening");
    f.read_to_end(&mut avifdata).expect("reading");

    let pixels = libavif::decode_rgb(&avifdata).expect("decoding");
    eprintln!("w={}, h={}", pixels.width(), pixels.height());

    let mut im = image::RgbImage::new(pixels.width(), pixels.height());

    for y in 0..im.height() {
        for x in 0..im.width() {
            let (r, g, b, _a) = pixels.pixel(x, y);
            im.put_pixel(x, y, [r, g, b].into());
        }
    }

    let im = image::DynamicImage::ImageRgb8(im);

    im.write_to(&mut std::io::stdout(), image::ImageOutputFormat::Png)
        .expect("out");
}
