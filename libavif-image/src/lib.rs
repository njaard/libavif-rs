//! Load and Save AVIF from [image](https://crates.io/crates/image)
//! types.
//!
//! Converts to and from YUV (`image` only does RGB).

use image::{DynamicImage, ImageBuffer};

pub use libavif::is_avif;
use libavif::AvifData;

/// Read data that is in an AVIF file and load it into an image
pub fn read(buf: &[u8]) -> Result<DynamicImage, String> {
    let pixels = libavif::decode_rgb(buf).map_err(|e| format!("decoding AVIF: {}", e))?;
    let buffer = ImageBuffer::from_vec(pixels.width(), pixels.height(), pixels.to_vec())
        .expect("pixels doesn't fit image::ImageBuffer");

    Ok(DynamicImage::ImageRgba8(buffer))
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
