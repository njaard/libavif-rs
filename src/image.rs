use std::slice;

use libavif_sys as sys;

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

    pub(crate) unsafe fn from_raw(image: *mut sys::avifImage) -> Self {
        debug_assert!(!image.is_null());

        Self { image }
    }

    pub(crate) fn inner(&self) -> *const sys::avifImage {
        self.image
    }
}
