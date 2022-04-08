//! Load and Save AVIF from [image](https://crates.io/crates/image)
//! types.
//!
//! Converts to and from YUV (`image` only does RGB).

use image::{buffer::ConvertBuffer, DynamicImage, ImageBuffer, Rgb};

use libavif::AvifData;
pub use libavif::{is_avif, Error};

/// Read data that is in an AVIF file and load it into an image
pub fn read(buf: &[u8]) -> Result<DynamicImage, Error> {
    let pixels = libavif::decode_rgb(buf)?;
    let buffer = ImageBuffer::from_vec(pixels.width(), pixels.height(), pixels.to_vec())
        .expect("pixels doesn't fit image::ImageBuffer");

    Ok(DynamicImage::ImageRgba8(buffer))
}

/// Save an image into an AVIF file
pub fn save(img: &DynamicImage) -> Result<AvifData, Error> {
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
        _ => return Err(Error::UnsupportedImageType),
    };

    Ok(data)
}
