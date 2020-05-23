use std::marker::PhantomData;
use std::slice;

use libavif_sys as sys;

pub struct AvifData<'a> {
    owned: bool,
    inner: sys::avifRWData,

    phantom: PhantomData<&'a [u8]>,
}

impl<'a> AvifData<'a> {
    pub fn new(b: &'a [u8]) -> Self {
        Self {
            owned: true,
            inner: sys::avifRWData {
                data: b.as_ptr() as *mut u8,
                size: b.len(),
            },
            phantom: PhantomData,
        }
    }

    /// Extracts a slice containg the entire data without doing clones or allocation.
    pub fn as_slice(&'a self) -> &'a [u8] {
        unsafe { slice::from_raw_parts(self.inner.data, self.inner.size) }
    }

    /// Converts `self` into a new vector by cloning the entire data.
    pub fn to_vec(&self) -> Vec<u8> {
        self.as_slice().to_vec()
    }
}

impl<'a> From<sys::avifRWData> for AvifData<'a> {
    fn from(data: sys::avifRWData) -> Self {
        Self {
            owned: false,
            inner: data,
            phantom: PhantomData,
        }
    }
}

impl Drop for AvifData<'_> {
    fn drop(&mut self) {
        if !self.owned {
            // pixels were allocated by libavif
            unsafe {
                sys::avifRWDataFree(&mut self.inner);
            }
        }
    }
}
