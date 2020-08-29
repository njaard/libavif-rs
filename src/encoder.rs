use std::io;

use crate::{AvifData, AvifImage};
use libavif_sys as sys;

/// AVIF image encoder
pub struct Encoder {
    encoder: *mut sys::avifEncoder,
}

impl Encoder {
    /// Create a new encoder with default settings
    pub fn new() -> Self {
        let encoder = unsafe { sys::avifEncoderCreate() };
        let mut encoder = Self { encoder };
        encoder.set_speed(10);
        encoder
    }

    /// Get the maximum allowed number of threads this `Encoder` can use
    pub fn max_threads(&self) -> usize {
        unsafe { (*self.encoder).maxThreads as usize }
    }

    /// Set the maximum allowed number of threads this `Encoder` can use
    pub fn set_max_threads(&mut self, max_threads: usize) -> &mut Self {
        unsafe { (*self.encoder).maxThreads = max_threads.max(1) as i32 }
        self
    }

    /// Get quantizer value for the YUV channels
    pub fn quantizer(&self) -> u8 {
        unsafe { (*self.encoder).minQuantizer as u8 }
    }

    /// Set the quantizer value for the YUV channels
    ///
    /// Must be between 0 and 63.
    ///
    /// * `0` - _lossless_
    /// * `63` - _lowest quality_
    pub fn set_quantizer(&mut self, quantizer: u8) -> &mut Self {
        let quantizer = quantizer.min(63) as i32;
        unsafe {
            (*self.encoder).minQuantizer = quantizer;
            (*self.encoder).maxQuantizer = quantizer;
        }
        self
    }

    /// Get quantizer value for the alpha channel
    pub fn quantizer_alpha(&self) -> u8 {
        unsafe { (*self.encoder).minQuantizerAlpha as u8 }
    }

    /// Set the quantizer value for the alpha channel
    ///
    /// Must be between 0 and 63.
    ///
    /// * `0` - _lossless_
    /// * `63` - _lowest quality_
    pub fn set_quantizer_alpha(&mut self, quantizer_alpha: u8) -> &mut Self {
        let quantizer_alpha = quantizer_alpha.min(63) as i32;
        unsafe {
            (*self.encoder).minQuantizerAlpha = quantizer_alpha;
            (*self.encoder).maxQuantizerAlpha = quantizer_alpha;
        }
        self
    }

    /// Get the speed of this `Encoder`
    pub fn speed(&self) -> u8 {
        unsafe { (*self.encoder).speed as u8 }
    }

    /// Set the speed of this `Encoder`
    ///
    /// Must be between 0 and 10.
    ///
    /// * `10` - _fastest_
    /// * `0` - _slowest_
    pub fn set_speed(&mut self, speed: u8) -> &mut Self {
        unsafe { (*self.encoder).speed = speed.min(10) as i32 }
        self
    }

    /// Encode an `AvifImage` using the settings from this `Encoder`
    pub fn encode(&self, image: &AvifImage) -> io::Result<AvifData<'static>> {
        let mut data = Default::default();
        let result = unsafe { sys::avifEncoderWrite(self.encoder, image.inner(), &mut data) };
        if result != sys::AVIF_RESULT_OK {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("result={}", result),
            ));
        }
        Ok(AvifData::from(data))
    }
}

impl Drop for Encoder {
    fn drop(&mut self) {
        unsafe {
            sys::avifEncoderDestroy(self.encoder);
        }
    }
}

impl Default for Encoder {
    fn default() -> Self {
        Self::new()
    }
}
