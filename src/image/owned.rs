use std::ptr;

use libavif_sys as sys;
use libavif_sys::avifImage;

use crate::{Error, YuvFormat};

/// YUV image
pub struct AvifImage {
    image: ptr::NonNull<sys::avifImage>,
}

impl AvifImage {
    pub fn from_luma8(width: u32, height: u32, pixels: &[u8]) -> Result<Self, Error> {
        if (width * height) as usize != pixels.len() {
            return Err(Error::UnsupportedImageType);
        }

        let mut image = Self::new(
            width.try_into().unwrap(),
            height.try_into().unwrap(),
            8,
            YuvFormat::Yuv400,
        );
        unsafe {
            image.set_y(pixels);
        }
        Ok(image)
    }

    pub fn width(&self) -> u32 {
        unsafe { self.image.as_ref().width }
    }

    pub fn height(&self) -> u32 {
        unsafe { self.image.as_ref().height }
    }

    pub(crate) fn empty() -> Self {
        unsafe {
            let image = sys::avifImageCreateEmpty();
            Self::from_raw(image)
        }
    }

    pub(crate) fn new(width: i32, height: i32, depth: i32, format: YuvFormat) -> Self {
        unsafe {
            let image = sys::avifImageCreate(width, height, depth, format as u32);
            sys::avifImageAllocatePlanes(image, sys::AVIF_PLANES_YUV);
            Self::from_raw(image)
        }
    }

    pub(crate) unsafe fn set_y(&mut self, y: &[u8]) {
        debug_assert!(!self.image.as_ref().yuvPlanes[0].is_null());

        ptr::copy_nonoverlapping(y.as_ptr(), self.image.as_mut().yuvPlanes[0], y.len());
    }

    /// Safety: `image` must be a valid value obtained from libavif
    /// which must have not been freed yet.
    pub(crate) unsafe fn from_raw(image: *mut sys::avifImage) -> Self {
        // unwrap used for compatibility
        Self { image: ptr::NonNull::new(image).unwrap() }
    }


    pub(crate) fn inner_mut(&mut self) -> *mut sys::avifImage {
        self.image.as_ptr()
    }
}

impl Drop for AvifImage {
    fn drop(&mut self) {
        unsafe {
            sys::avifImageDestroy(self.image.as_ptr());
        }
    }
}

impl super::AvifImageRef for AvifImage {
    unsafe fn image(&self) -> &avifImage {
        self.image.as_ref()
    }
}
