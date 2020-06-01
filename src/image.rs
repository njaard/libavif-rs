use std::marker::PhantomData;
use std::slice;

use libavif_sys as sys;

pub struct RgbPixels<'a> {
    owned: bool,
    inner: sys::avifRGBImage,

    phantom: PhantomData<&'a [u8]>,
}

impl<'a> RgbPixels<'a> {
    pub fn new(width: u32, height: u32, rgb: &'a [u8]) -> Self {
        let (stride, format) = if (width * height * 3) as usize == rgb.len() {
            // RGB
            (3, sys::AVIF_RGB_FORMAT_RGB)
        } else if (width * height * 4) as usize == rgb.len() {
            // RGBA
            (4, sys::AVIF_RGB_FORMAT_RGBA)
        } else {
            panic!("invalid rgb len")
        };

        Self {
            owned: true,
            inner: sys::avifRGBImage {
                width,
                height,
                depth: 8,
                format,
                chromaUpsampling: sys::AVIF_CHROMA_UPSAMPLING_BILINEAR,
                pixels: rgb.as_ptr() as *mut u8,
                rowBytes: stride * width,
            },
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
        assert!(x < self.width());
        assert!(y < self.height());

        unsafe {
            let pixels = self.inner.pixels;
            let row_bytes = self.inner.rowBytes as usize;
            let rgb = pixels.add((4 * x as usize) + (row_bytes * y as usize));
            let r = *rgb.add(0);
            let g = *rgb.add(1);
            let b = *rgb.add(2);
            let a = *rgb.add(3);
            (r, g, b, a)
        }
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

    pub(crate) unsafe fn inner_mut(&mut self) -> &mut sys::avifRGBImage {
        &mut self.inner
    }
}

impl<'a> From<sys::avifRGBImage> for RgbPixels<'a> {
    fn from(rgb: sys::avifRGBImage) -> Self {
        Self {
            owned: false,
            inner: rgb,
            phantom: PhantomData,
        }
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
