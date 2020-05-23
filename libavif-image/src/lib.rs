//! Load and Save AVIF from [image](https://crates.io/crates/image)
//! types.
//!
//! Converts to and from YUV (`image` only does RGB).

use image::{DynamicImage, RgbImage};

pub use libavif::is_avif;
use libavif::AvifData;

/// Read data that is in an AVIF file and load it into an image
pub fn read(buf: &[u8]) -> Result<DynamicImage, String> {
    let pixels = libavif::decode_rgb(buf).map_err(|e| format!("decoding AVIF: {}", e))?;
    let mut img = RgbImage::new(pixels.width(), pixels.height());

    for y in 0..img.height() {
        for x in 0..img.width() {
            let (r, g, b, _a) = pixels.pixel(x, y);
            img.put_pixel(x, y, [r, g, b].into());
        }
    }

    Ok(DynamicImage::ImageRgb8(img))
}

/// Save an image into an AVIF file
pub fn save(img: &DynamicImage) -> Result<AvifData, String> {
    let data = match img {
        DynamicImage::ImageRgb8(img) => {
            let rgb = img.as_flat_samples();
            libavif::encode_rgb8(img.width(), img.height(), rgb.as_slice())
                .map_err(|e| format!("encoding AVIF: {:?}", e))?
        }
        _ => return Err("image type not supported".into()),
    };

    Ok(data)
}
