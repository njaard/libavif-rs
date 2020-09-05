use libavif_sys as sys;

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
