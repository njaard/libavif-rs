use std::ptr;

use libavif_sys as sys;

use crate::YuvFormat;

/// YUV image
pub struct AvifImage {
    image: *mut sys::avifImage,
}

impl AvifImage {
    pub fn width(&self) -> u32 {
        unsafe { (*self.image).width }
    }

    pub fn height(&self) -> u32 {
        unsafe { (*self.image).height }
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

    pub(crate) fn set_y(&mut self, y: &[u8]) {
        // TODO: unsound
        unsafe {
            ptr::copy_nonoverlapping(y.as_ptr(), (*self.image).yuvPlanes[0], y.len());
        }
    }

    /// Safety: `image` must be a valid value obtained from libavif
    /// which must have not been freed yet.
    pub(crate) unsafe fn from_raw(image: *mut sys::avifImage) -> Self {
        debug_assert!(!image.is_null());

        Self { image }
    }

    pub(crate) fn inner(&self) -> *const sys::avifImage {
        self.image
    }

    pub(crate) fn inner_mut(&mut self) -> *mut sys::avifImage {
        self.image
    }
}

impl Drop for AvifImage {
    fn drop(&mut self) {
        unsafe {
            sys::avifImageDestroy(self.image);
        }
    }
}
