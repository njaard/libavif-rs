use std::env;

use image::{buffer::ConvertBuffer, DynamicImage, ImageBuffer, Rgb};

fn main() -> Result<(), libavif::Error> {
    let input = env::args().nth(1).expect("input png");
    let img = image::open(&input).expect("opening");

    let data = match img {
        DynamicImage::ImageRgb8(img) => {
            let rgb = img.as_flat_samples();
            libavif::encode_rgb8(img.width(), img.height(), rgb.as_slice())?
        }
        DynamicImage::ImageRgba8(img) => {
            let rgb = img.as_flat_samples();
            libavif::encode_rgb8(img.width(), img.height(), rgb.as_slice())?
        }
        DynamicImage::ImageLuma8(img) => {
            let image: ImageBuffer<Rgb<u8>, _> = img.convert();
            let rgb = image.as_flat_samples();
            libavif::encode_rgb8(img.width(), img.height(), rgb.as_slice())?
        }
        DynamicImage::ImageLumaA8(img) => {
            let image: ImageBuffer<Rgb<u8>, _> = img.convert();
            let rgb = image.as_flat_samples();
            libavif::encode_rgb8(img.width(), img.height(), rgb.as_slice())?
        }
        DynamicImage::ImageLuma16(img) => {
            let image: ImageBuffer<Rgb<u8>, _> = img.convert();
            let rgb = image.as_flat_samples();
            libavif::encode_rgb8(img.width(), img.height(), rgb.as_slice())?
        }
        DynamicImage::ImageLumaA16(img) => {
            let image: ImageBuffer<Rgb<u8>, _> = img.convert();
            let rgb = image.as_flat_samples();
            libavif::encode_rgb8(img.width(), img.height(), rgb.as_slice())?
        }
        DynamicImage::ImageRgb16(img) => {
            let image: ImageBuffer<Rgb<u8>, _> = img.convert();
            let rgb = image.as_flat_samples();
            libavif::encode_rgb8(img.width(), img.height(), rgb.as_slice())?
        }
        DynamicImage::ImageRgba16(img) => {
            let image: ImageBuffer<Rgb<u8>, _> = img.convert();
            let rgb = image.as_flat_samples();
            libavif::encode_rgb8(img.width(), img.height(), rgb.as_slice())?
        }
        DynamicImage::ImageRgb32F(img) => {
            let image: ImageBuffer<Rgb<u8>, _> = img.convert();
            let rgb = image.as_flat_samples();
            libavif::encode_rgb8(img.width(), img.height(), rgb.as_slice())?
        }
        DynamicImage::ImageRgba32F(img) => {
            let image: ImageBuffer<Rgb<u8>, _> = img.convert();
            let rgb = image.as_flat_samples();
            libavif::encode_rgb8(img.width(), img.height(), rgb.as_slice())?
        }
        _ => panic!("Unsupported Image Type"),
    };

    std::fs::write("out.avif", data.as_slice()).expect("output avif");
    Ok(())
}
