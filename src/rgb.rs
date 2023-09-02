use std::marker::PhantomData;
use std::ops::Deref;
use std::slice;

use libavif_sys as sys;

use crate::{AvifImage, Error, YuvFormat};

pub struct RgbPixels<'a> {
    owned: bool,
    inner: sys::avifRGBImage,

    phantom: PhantomData<&'a [u8]>,
}

impl<'a> RgbPixels<'a> {
    pub fn new(width: u32, height: u32, rgb: &'a [u8]) -> Result<Self, Error> {
        let (stride, format) = if (width * height * 3) as usize == rgb.len() {
            // RGB
            (3, sys::AVIF_RGB_FORMAT_RGB)
        } else if (width * height * 4) as usize == rgb.len() {
            // RGBA
            (4, sys::AVIF_RGB_FORMAT_RGBA)
        } else {
            return Err(Error::UnsupportedImageType);
        };

        Ok(Self {
            owned: true,
            inner: sys::avifRGBImage {
                width,
                height,
                depth: 8,
                format,
                chromaUpsampling: sys::AVIF_CHROMA_UPSAMPLING_BILINEAR,
                ignoreAlpha: 0,
                alphaPremultiplied: 0,
                isFloat: 0,
                maxThreads: 1,
                pixels: rgb.as_ptr() as *mut u8,
                rowBytes: stride * width,
                avoidLibYUV: sys::AVIF_FALSE as i32,
                chromaDownsampling: sys::AVIF_CHROMA_DOWNSAMPLING_BEST_QUALITY,
            },
            phantom: PhantomData,
        })
    }

    /// Safety: `rgb` must be a valid value obtained from libavif
    /// which must have not been freed yet.
    pub(crate) unsafe fn from_raw(rgb: sys::avifRGBImage) -> Self {
        Self {
            owned: false,
            inner: rgb,
            phantom: PhantomData,
        }
    }

    /// width of the image in pixels
    pub fn width(&self) -> u32 {
        self.inner.width
    }

    /// height of the image in pixels
    pub fn height(&self) -> u32 {
        self.inner.height
    }

    pub fn pixel(&self, x: u32, y: u32) -> (u8, u8, u8, u8) {
        let stride = if self.inner.format == sys::AVIF_RGB_FORMAT_RGBA {
            4
        } else {
            3
        };

        let row_bytes = self.inner.rowBytes as usize;
        let i = (stride * x as usize) + (row_bytes * y as usize);

        let slice = self.as_slice();
        let slice = &slice[i..][..stride];
        (
            slice[0],
            slice[1],
            slice[2],
            if stride == 4 { slice[3] } else { 255 },
        )
    }

    /// Extracts a slice containg all of the pixels without doing clones or allocation.
    pub fn as_slice(&'a self) -> &'a [u8] {
        let size = self.inner.rowBytes * self.height();
        unsafe { slice::from_raw_parts(self.inner.pixels, size as usize) }
    }

    /// Converts `self` into a new vector by cloning all of the pixels.
    pub fn to_vec(&self) -> Vec<u8> {
        self.as_slice().to_vec()
    }

    pub fn to_image(&self, yuv_format: YuvFormat) -> AvifImage {
        unsafe {
            let image =
                sys::avifImageCreate(self.width() as _, self.height() as _, 8, yuv_format as u32);
            sys::avifImageAllocatePlanes(image, sys::AVIF_PLANES_YUV as _);

            sys::avifImageRGBToYUV(image, &self.inner as *const sys::avifRGBImage);
            AvifImage::from_raw(image)
        }
    }
}

impl<'a> From<AvifImage> for RgbPixels<'a> {
    fn from(image: AvifImage) -> Self {
        Self::from(&image)
    }
}

impl<'a> From<&AvifImage> for RgbPixels<'a> {
    fn from(image: &AvifImage) -> Self {
        unsafe {
            let mut rgb = sys::avifRGBImage::default();
            let raw_rgb = &mut rgb as *mut sys::avifRGBImage;
            sys::avifRGBImageSetDefaults(raw_rgb, image.inner());
            rgb.format = sys::AVIF_RGB_FORMAT_RGBA;
            rgb.depth = 8;

            sys::avifRGBImageAllocatePixels(raw_rgb);
            sys::avifImageYUVToRGB(image.inner(), raw_rgb);

            RgbPixels::from_raw(rgb)
        }
    }
}

impl<'a> Deref for RgbPixels<'a> {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl Drop for RgbPixels<'_> {
    fn drop(&mut self) {
        if !self.owned {
            // pixels were allocated by libavif
            unsafe {
                sys::avifRGBImageFreePixels(&mut self.inner as *mut sys::avifRGBImage);
            }
        }
    }
}
