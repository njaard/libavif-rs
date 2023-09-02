use crate::{AddImageFlags, AvifData, AvifImage, Error};
use libavif_sys as sys;

/// AVIF image encoder
///
/// ## Encoding a single image
///
/// ```no_run
/// # use std::fs;
/// # use libavif::{AvifImage, Encoder};
/// #
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let luma_data = fs::read("luma.raw")?;
/// let image = AvifImage::from_luma8(128, 128, &luma_data)?;
/// let encoder = Encoder::new();
/// let data = encoder.encode(&image)?;
/// fs::write("luma.avif", &*data)?;
/// # Ok(())
/// # }
/// ```
///
/// ## Encoding an image sequence
///
/// ```no_run
/// # use std::fs;
/// # use libavif::{AvifImage, Encoder};
/// #
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let mut encoder = Encoder::new();
/// // Set timescale to 60Hz...
/// encoder.set_timescale(60);
/// for i in 0..60 {
///     let luma_data = fs::read(&format!("luma{}.raw", i))?;
///     let image = AvifImage::from_luma8(128, 128, &luma_data)?;
///     // ...so we can use 1 as the duration.
///     // The duration of a single frame is now 1/60 s.
///     // So the framerate is 60fps
///     encoder.add_image(&image, 1, Default::default())?;
/// }
/// let data = encoder.finish()?;
/// fs::write("luma_animation.avif", &*data)?;
/// # Ok(())
/// # }
/// ```
pub struct Encoder {
    encoder: *mut sys::avifEncoder,
}

impl Encoder {
    /// Create a new encoder with default settings
    ///
    /// # Defaults
    ///
    /// * `max_threads` -> `1`
    /// * `quantizer` -> `0`
    /// * `quantizer_alpha` -> `0`
    /// * `speed` -> `10`
    pub fn new() -> Self {
        let encoder = unsafe { sys::avifEncoderCreate() };
        Self { encoder }
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

    /// Get quality of the YUV channels
    pub fn quality(&self) -> u8 {
        unsafe { (*self.encoder).quality as u8 }
    }

    /// Set the quality for the YUV channels
    ///
    /// Must be between 0 and 100.
    ///
    /// * `100` - _lossless quality_
    /// * `0` - _lowest quality_
    pub fn set_quality(&mut self, quality: u8) -> &mut Self {
        let quality = quality.min(100);
        unsafe {
            (*self.encoder).quality = quality.into();
        }
        self
    }

    /// Get quality of the alpha channel
    pub fn alpha_quality(&self) -> u8 {
        unsafe { (*self.encoder).qualityAlpha as u8 }
    }

    /// Set the quality for the alpha channel
    ///
    /// Must be between 0 and 100.
    ///
    /// * `100` - _lossless quality_
    /// * `0` - _lowest quality_
    pub fn set_alpha_quality(&mut self, alpha_quality: u8) -> &mut Self {
        let alpha_quality = alpha_quality.min(100);
        unsafe {
            (*self.encoder).qualityAlpha = alpha_quality.into();
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

    /// Get the timescale of this `Encoder` in Hz (1/s)
    pub fn timescale(&self) -> u64 {
        unsafe { (*self.encoder).timescale }
    }

    /// Set the timescale of this `Encoder` in Hz (1/s)
    ///
    /// The duration of an image in seconds is `duration_in_timescales / timescale [s]`.
    ///
    /// For example, an image with a duration of `1` added to an `Encoder` with a timescale of `60 Hz`
    /// would mean the image has a duration of `1/60 s`.
    pub fn set_timescale(&mut self, timescale: u64) -> &mut Self {
        unsafe { (*self.encoder).timescale = timescale }
        self
    }

    /// Encode an `AvifImage` using the settings from this `Encoder`
    ///
    /// Calling this function is the same as calling [`add_image`](Self::add_image) with
    /// [`AddImageFlags::SINGLE`](crate::AddImageFlags::SINGLE) and calling [`finish`](Self::finish).
    pub fn encode(&self, image: &AvifImage) -> Result<AvifData<'static>, Error> {
        let mut data = Default::default();
        unsafe {
            Error::code(sys::avifEncoderWrite(
                self.encoder,
                image.inner(),
                &mut data,
            ))?;
            Ok(AvifData::from_raw(data))
        }
    }

    /// Add an `AvifImage` to this `Encoder`.
    pub fn add_image(
        &self,
        image: &AvifImage,
        duration_in_timescales: u64,
        flags: AddImageFlags,
    ) -> Result<(), super::Error> {
        unsafe {
            Error::code(sys::avifEncoderAddImage(
                self.encoder,
                image.inner(),
                duration_in_timescales,
                flags.into(),
            ))
        }
    }

    /// Finish encoding an image or an animation.
    ///
    /// You only need to call this function if you used [`add_image`](Self::add_image).
    pub fn finish(&self) -> Result<AvifData<'static>, super::Error> {
        unsafe {
            let mut data = Default::default();
            Error::code(sys::avifEncoderFinish(self.encoder, &mut data))?;
            Ok(AvifData::from_raw(data))
        }
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
