use std::marker::PhantomData;
use std::ptr;

use crate::{ChromaSamplePosition, YuvFormat};
use libavif_sys as sys;
use libavif_sys::avifImage;

/// A readonly image that borrows its YUV+A planes' data.
///
/// **Safety:** This struct must never return a `*mut sys::avifImage`
/// to make sure the plane data remains immutable.
pub struct BorrowedAvifImage<'y, 'u, 'v, 'a> {
    image: ptr::NonNull<sys::avifImage>,
    _y_marker: PhantomData<&'y [u8]>,
    _u_marker: PhantomData<&'u [u8]>,
    _v_marker: PhantomData<&'v [u8]>,
    _a_marker: PhantomData<&'a [u8]>,
}

impl<'y, 'u, 'v, 'a> BorrowedAvifImage<'y, 'u, 'v, 'a> {
    pub fn new(width: i32, height: i32, depth: i32, format: YuvFormat) -> Option<Self> {
        unsafe {
            let mut image =
                ptr::NonNull::new(sys::avifImageCreate(width, height, depth, format as u32))?;
            image.as_mut().imageOwnsYUVPlanes = sys::AVIF_FALSE as sys::avifBool;
            image.as_mut().imageOwnsAlphaPlane = sys::AVIF_FALSE as sys::avifBool;

            Some(Self {
                image,
                _y_marker: Default::default(),
                _u_marker: Default::default(),
                _v_marker: Default::default(),
                _a_marker: Default::default(),
            })
        }
    }

    /// Safety: Here, we turn an immutable reference into a mutable reference.
    /// This is safe as long as we don't give out `*mut sys::avifImage`.
    pub fn set_y(&mut self, data: &'y [u8]) -> &mut Self {
        unsafe { self.image.as_mut().yuvPlanes[0] = data.as_ptr() as *mut _ }
        self
    }

    /// Safety: See [set_y](Self::set_y).
    pub fn set_u(&mut self, data: &'u [u8]) -> &mut Self {
        unsafe { self.image.as_mut().yuvPlanes[1] = data.as_ptr() as *mut _ }
        self
    }

    /// Safety: See [set_y](Self::set_y).
    pub fn set_v(&mut self, data: &'v [u8]) -> &mut Self {
        unsafe { self.image.as_mut().yuvPlanes[2] = data.as_ptr() as *mut _ }
        self
    }

    /// Safety: See [set_y](Self::set_y).
    pub fn set_a(&mut self, data: &'a [u8]) -> &mut Self {
        unsafe { self.image.as_mut().alphaPlane = data.as_ptr() as *mut _ }
        self
    }

    /// Set the bytes of this image for a row in the Y-plane (stride).
    /// This _does not_ need to be equal to the width.
    pub fn set_y_row_bytes(&mut self, row_bytes: u32) -> &mut Self {
        unsafe { self.image.as_mut().yuvRowBytes[0] = row_bytes }
        self
    }

    /// Set the bytes of this image for a row in the U-plane (stride).
    /// This _does not_ need to be equal to the width.
    pub fn set_u_row_bytes(&mut self, row_bytes: u32) -> &mut Self {
        unsafe { self.image.as_mut().yuvRowBytes[1] = row_bytes }
        self
    }

    /// Set the bytes of this image for a row in the V-plane (stride).
    /// This _does not_ need to be equal to the width.
    pub fn set_v_row_bytes(&mut self, row_bytes: u32) -> &mut Self {
        unsafe { self.image.as_mut().yuvRowBytes[2] = row_bytes }
        self
    }

    /// Set the bytes of this image for a row in the A-plane (stride).
    /// This _does not_ need to be equal to the width.
    pub fn set_a_row_bytes(&mut self, row_bytes: u32) -> &mut Self {
        unsafe { self.image.as_mut().alphaRowBytes = row_bytes }
        self
    }

    /// Set the chroma sample position
    pub fn set_chroma_sample_position(&mut self, pos: ChromaSamplePosition) -> &mut Self {
        unsafe { self.image.as_mut().yuvChromaSamplePosition = pos as _ }
        self
    }
}

impl Drop for BorrowedAvifImage<'_, '_, '_, '_> {
    fn drop(&mut self) {
        unsafe { sys::avifImageDestroy(self.image.as_ptr()) }
    }
}

impl super::AvifImageRef for BorrowedAvifImage<'_, '_, '_, '_> {
    unsafe fn image(&self) -> &avifImage {
        self.image.as_ref()
    }
}
