#![allow(clippy::many_single_char_names)]

use std::io;

pub use self::data::AvifData;
pub use self::image::RgbPixels;
use libavif_sys as sys;

mod data;
mod image;

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

/// Decode into RGB pixels
pub fn decode_rgb(avif_bytes: &[u8]) -> io::Result<RgbPixels> {
    unsafe {
        let mut raw = sys::avifROData {
            data: avif_bytes.as_ptr(),
            size: avif_bytes.len(),
        };

        let image = sys::avifImageCreateEmpty();
        let decoder = sys::avifDecoderCreate();
        let result = sys::avifDecoderRead(decoder, image, &mut raw);
        sys::avifDecoderDestroy(decoder);
        if result != sys::AVIF_RESULT_OK {
            sys::avifImageDestroy(image);
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("result={}", result),
            ));
        }

        let mut rgb = sys::avifRGBImage::default();
        let raw_rgb = &mut rgb as *mut sys::avifRGBImage;
        sys::avifRGBImageSetDefaults(raw_rgb, image);
        rgb.format = sys::AVIF_RGB_FORMAT_RGBA;
        rgb.depth = 8;

        sys::avifRGBImageAllocatePixels(raw_rgb);
        sys::avifImageYUVToRGB(image, raw_rgb);
        sys::avifImageDestroy(image);

        Ok(RgbPixels { rgb })
    }
}

/// Encode an 8 bit per channel RGB or RGBA image
pub fn encode_rgb8(width: u32, height: u32, rgb: &[u8]) -> io::Result<AvifData<'static>> {
    let (stride, format) = if (width * height * 3) as usize == rgb.len() {
        // RGB
        (3, sys::AVIF_RGB_FORMAT_RGB)
    } else if (width * height * 4) as usize == rgb.len() {
        // RGBA
        (4, sys::AVIF_RGB_FORMAT_RGBA)
    } else {
        panic!("invalid rgb len")
    };

    unsafe {
        let image = sys::avifImageCreate(width as _, height as _, 8, sys::AVIF_PIXEL_FORMAT_YUV444);
        sys::avifImageAllocatePlanes(image, sys::AVIF_PLANES_YUV as _);

        let mut rgb = sys::avifRGBImage {
            width,
            height,
            depth: 8,
            format,
            chromaUpsampling: sys::AVIF_CHROMA_UPSAMPLING_BILINEAR,
            pixels: rgb.as_ptr() as *mut u8,
            rowBytes: stride * width,
        };

        sys::avifImageRGBToYUV(image, &mut rgb);

        let mut encoder = sys::avifEncoderCreate();
        (*encoder).maxThreads = 1;
        (*encoder).minQuantizer = 5;
        (*encoder).maxQuantizer = 40;
        let mut raw = Default::default();
        let result = sys::avifEncoderWrite(encoder, image, &mut raw);
        sys::avifEncoderDestroy(encoder);
        sys::avifImageDestroy(image);
        if result != sys::AVIF_RESULT_OK {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("result={}", result),
            ));
        }

        Ok(AvifData::from(raw))
    }
}
