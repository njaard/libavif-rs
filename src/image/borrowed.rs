use std::marker::PhantomData;
use std::ptr;

use crate::{ChromaSamplePosition, YuvFormat};
use libavif_sys as sys;
use libavif_sys::avifImage;

/// A readonly image that borrows its YUV+A planes' data.
///
/// **Safety:** This struct must never return a `*mut sys::avifImage`
/// to make sure the plane data remains immutable.
pub struct BorrowedAvifImage<'data> {
    image: ptr::NonNull<sys::avifImage>,
    _marker: PhantomData<&'data [u8]>,
}

impl<'data> BorrowedAvifImage<'data> {
    pub fn new(width: i32, height: i32, depth: i32, format: YuvFormat) -> Option<Self> {
        unsafe {
            let mut image =
                ptr::NonNull::new(sys::avifImageCreate(width, height, depth, format as u32))?;
            image.as_mut().imageOwnsYUVPlanes = sys::AVIF_FALSE as sys::avifBool;
            image.as_mut().imageOwnsAlphaPlane = sys::AVIF_FALSE as sys::avifBool;

            Some(Self {
                image,
                _marker: Default::default(),
            })
        }
    }

    /// `row_bytes` is the number of bytes for one row in the image.
    /// This _doesn't_ need to be equal to the width of this image.
    ///
    /// **Panics** If the length of `data` is less than `row_bytes * height`.
    ///
    /// **Safety:** Here, we turn an immutable reference into a mutable reference.
    /// This is safe as long as we don't give out `*mut sys::avifImage`.
    pub fn set_y(&mut self, data: &'data [u8], row_bytes: u32) -> &mut Self {
        unsafe {
            // Y-Plane must always have full width and height
            assert!(data.len() as u32 >= row_bytes * self.image.as_ref().height);
            self.image.as_mut().yuvRowBytes[0] = row_bytes;
            self.image.as_mut().yuvPlanes[0] = data.as_ptr() as *mut _;
        }
        self
    }

    /// `row_bytes` is the number of bytes for one row in the image.
    /// This _doesn't_ need to be equal to the width of this image.
    ///
    /// **Panics** If `data` is too short.
    /// For YUV420, the height only has to be half of the luma's height,
    //  else it has to be as high as the luma plane.
    ///
    /// **Safety:** See [set_y](Self::set_y).
    pub fn set_u(&mut self, data: &'data [u8], row_bytes: u32) -> &mut Self {
        unsafe {
            assert!(data.len() as u32 / row_bytes >= self.expected_chroma_height());
            self.image.as_mut().yuvRowBytes[1] = row_bytes;
            self.image.as_mut().yuvPlanes[1] = data.as_ptr() as *mut _;
        }
        self
    }

    /// `row_bytes` is the number of bytes for one row in the image.
    /// This _doesn't_ need to be equal to the width of this image.
    ///
    /// **Panics** If `data` is too short.
    /// For YUV420, the height only has to be half of the luma's height,
    /// else it has to be as high as the luma plane.
    ///
    /// **Safety:** See [set_y](Self::set_y).
    pub fn set_v(&mut self, data: &'data [u8], row_bytes: u32) -> &mut Self {
        unsafe {
            assert!(data.len() as u32 / row_bytes >= self.expected_chroma_height());
            self.image.as_mut().yuvRowBytes[2] = row_bytes;
            self.image.as_mut().yuvPlanes[2] = data.as_ptr() as *mut _;
        }
        self
    }

    /// `row_bytes` is the number of bytes for one row in the image.
    /// This _doesn't_ need to be equal to the width of this image.
    ///
    /// **Panics** If the length of `data` is less than `row_bytes * height`.
    ///
    /// **Safety:** See [set_y](Self::set_y).
    pub fn set_a(&mut self, data: &'data [u8], row_bytes: u32) -> &mut Self {
        unsafe {
            // alpha plane has the same height as the luma plane
            assert!(data.len() as u32 / row_bytes >= self.image.as_ref().height);
            self.image.as_mut().alphaRowBytes = row_bytes;
            self.image.as_mut().alphaPlane = data.as_ptr() as *mut _;
        }
        self
    }

    /// Set the chroma sample position
    pub fn set_chroma_sample_position(&mut self, pos: ChromaSamplePosition) -> &mut Self {
        unsafe { self.image.as_mut().yuvChromaSamplePosition = pos as _ }
        self
    }

    /// Returns the expected height of the u and v planes for this image.
    fn expected_chroma_height(&self) -> u32 {
        unsafe {
            if self.image.as_ref().yuvFormat == sys::AVIF_PIXEL_FORMAT_YUV420 {
                // only in yuv420: the chroma size is half of the luma
                (self.image.as_ref().height + 1) / 2
            } else {
                self.image.as_ref().height
            }
        }
    }
}

impl Drop for BorrowedAvifImage<'_> {
    fn drop(&mut self) {
        unsafe { sys::avifImageDestroy(self.image.as_ptr()) }
    }
}

impl super::AvifImageRef for BorrowedAvifImage<'_> {
    unsafe fn image(&self) -> &avifImage {
        self.image.as_ref()
    }
}
