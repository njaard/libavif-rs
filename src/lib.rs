#![allow(clippy::many_single_char_names)]

pub use self::data::AvifData;
pub use self::encoder::Encoder;
pub use self::error::Error;
pub use self::flags::AddImageFlags;
pub use self::format::YuvFormat;
pub use self::image::AvifImage;
pub use self::rgb::RgbPixels;
use libavif_sys as sys;

mod data;
mod encoder;
mod error;
mod flags;
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

pub fn decode(avif_bytes: &[u8]) -> Result<AvifImage, Error> {
    let mut image = AvifImage::empty();
    unsafe {
        let decoder = sys::avifDecoderCreate();
        let result = sys::avifDecoderReadMemory(
            decoder,
            image.inner_mut(),
            avif_bytes.as_ptr(),
            avif_bytes.len(),
        );
        sys::avifDecoderDestroy(decoder);
        Error::code(result)?;

        Ok(image)
    }
}

/// Decode into RGB pixels
pub fn decode_rgb(avif_bytes: &[u8]) -> Result<RgbPixels, Error> {
    decode(avif_bytes).map(Into::into)
}

/// Encode an 8 bit per channel RGB, RGBA or Luma8 image
pub fn encode_rgb8(width: u32, height: u32, rgb: &[u8]) -> Result<AvifData<'static>, Error> {
    let image = if (width * height) as usize == rgb.len() {
        AvifImage::from_luma8(width, height, rgb)?
    } else {
        let rgb = RgbPixels::new(width, height, rgb)?;
        rgb.to_image(YuvFormat::Yuv444)
    };

    let encoder = Encoder::new();
    encoder.encode(&image)
}
