use std::error::Error as StdError;
use std::fmt::{self, Display, Formatter};

use libavif_sys as sys;

/// The errors that may occur while processing an image
#[derive(Debug, Clone)]
pub enum Error {
    /// libavif operation failed with result `code`
    Code(u32),
    /// The image pixel format isn't supported or the specified `width` and `height` don't
    /// match the pixel buffer size
    UnsupportedImageType,
}

impl Error {
    pub(crate) fn code(code: ::std::os::raw::c_uint) -> Result<(), Error> {
        if code == sys::AVIF_RESULT_OK {
            Ok(())
        } else {
            #[allow(clippy::unnecessary_cast)]
            Err(Error::Code(code as u32))
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::Code(code) => write!(f, "libavif error: {}", code),
            Error::UnsupportedImageType => f.write_str("unsupported image type"),
        }
    }
}

impl StdError for Error {}
