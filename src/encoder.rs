use std::io;

use crate::{AvifData, AvifImage};
use libavif_sys as sys;

pub struct Encoder {
    encoder: *mut sys::avifEncoder,
}

impl Encoder {
    pub fn new() -> Self {
        let encoder = unsafe { sys::avifEncoderCreate() };
        let mut encoder = Self { encoder };
        encoder.set_speed(10);
        encoder
    }

    pub fn max_threads(&self) -> usize {
        unsafe { (*self.encoder).maxThreads as usize }
    }

    pub fn set_max_threads(&mut self, max_threads: usize) -> &mut Self {
        assert!(max_threads >= 1, "max_threads must be >= 1");

        unsafe { (*self.encoder).maxThreads = max_threads as i32 }
        self
    }

    pub fn min_quantizer(&self) -> u8 {
        unsafe { (*self.encoder).minQuantizer as u8 }
    }

    pub fn set_min_quantizer(&mut self, min_quantizer: u8) -> &mut Self {
        assert!(min_quantizer <= 63, "min_quantizer must be <= 63");

        unsafe { (*self.encoder).minQuantizer = min_quantizer as i32 }
        self
    }

    pub fn max_quantizer(&self) -> u8 {
        unsafe { (*self.encoder).maxQuantizer as u8 }
    }

    pub fn set_max_quantizer(&mut self, max_quantizer: u8) -> &mut Self {
        assert!(max_quantizer <= 63, "max_quantizer must be <= 63");

        unsafe { (*self.encoder).maxQuantizer = max_quantizer as i32 }
        self
    }

    pub fn min_quantizer_alpha(&self) -> u8 {
        unsafe { (*self.encoder).minQuantizerAlpha as u8 }
    }

    pub fn set_min_quantizer_alpha(&mut self, min_quantizer_alpha: u8) -> &mut Self {
        assert!(
            min_quantizer_alpha <= 63,
            "min_quantizer_alpha must be <= 63"
        );

        unsafe { (*self.encoder).minQuantizerAlpha = min_quantizer_alpha as i32 }
        self
    }

    pub fn max_quantizer_alpha(&self) -> u8 {
        unsafe { (*self.encoder).maxQuantizerAlpha as u8 }
    }

    pub fn set_max_quantizer_alpha(&mut self, max_quantizer_alpha: u8) -> &mut Self {
        assert!(
            max_quantizer_alpha <= 63,
            "max_quantizer_alpha must be <= 63"
        );

        unsafe { (*self.encoder).maxQuantizerAlpha = max_quantizer_alpha as i32 }
        self
    }

    pub fn speed(&self) -> u8 {
        unsafe { (*self.encoder).speed as u8 }
    }

    pub fn set_speed(&mut self, speed: u8) -> &mut Self {
        assert!(speed <= 10, "speed must be <= 10");

        unsafe { (*self.encoder).speed = speed as i32 }
        self
    }

    pub fn encode(&self, image: AvifImage) -> io::Result<AvifData<'static>> {
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
