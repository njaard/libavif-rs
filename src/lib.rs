#![allow(clippy::many_single_char_names)]

use std::io;

pub use self::data::AvifData;
pub use self::encoder::Encoder;
pub use self::format::YuvFormat;
pub use self::image::AvifImage;
pub use self::rgb::RgbPixels;
use libavif_sys as sys;

mod data;
mod encoder;
mod format;
mod image;
mod rgb;

/// Very efficiently detects AVIF files
///
/// returns true if the file header matches the AVIF type
/// Does not necessarily confirm that the file can actually
/// be decoded.
///
/// Generally requires no more than 64 bytes to make
/// this determination.
pub fn is_avif(avif_bytes: &[u8]) -> bool {
    let raw = sys::avifROData {
        data: avif_bytes.as_ptr(),
        size: avif_bytes.len(),
    };
    unsafe { sys::avifPeekCompatibleFileType(&raw) == 1 }
}

pub fn decode(avif_bytes: &[u8]) -> io::Result<AvifImage> {
    unsafe {
        let raw = sys::avifROData {
            data: avif_bytes.as_ptr(),
            size: avif_bytes.len(),
        };

        let image = sys::avifImageCreateEmpty();
        let decoder = sys::avifDecoderCreate();
        let result = sys::avifDecoderRead(decoder, image, &raw);
        sys::avifDecoderDestroy(decoder);
        if result != sys::AVIF_RESULT_OK {
            sys::avifImageDestroy(image);
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("result={}", result),
            ));
        }
        Ok(AvifImage::from_raw(image))
    }
}

/// Decode into RGB pixels
pub fn decode_rgb(avif_bytes: &[u8]) -> io::Result<RgbPixels> {
    decode(avif_bytes).map(Into::into)
}

/// Encode an 8 bit per channel RGB or RGBA image
pub fn encode_rgb8(width: u32, height: u32, rgb: &[u8]) -> io::Result<AvifData<'static>> {
    let rgb = RgbPixels::new(width, height, rgb);

    let mut encoder = Encoder::new();
    encoder.set_max_threads(1);
    encoder.set_min_quantizer(5);
    encoder.set_max_quantizer(40);
    encoder.encode(rgb.to_image(YuvFormat::Yuv444))
}
