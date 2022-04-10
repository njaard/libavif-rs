mod owned;
mod borrowed;
mod format;
mod chroma_position;

pub use owned::AvifImage;
pub use borrowed::BorrowedAvifImage;
pub use format::YuvFormat;
pub use chroma_position::ChromaSamplePosition;

use libavif_sys as sys;

pub trait AvifImageRef {
    /// # Safety
    /// * The returned image must be a valid reference
    /// * The YUV and A planes must be null pointers _or_ point
    /// to non-freed memory with enough capacity `(plane_row_bytes * height)`
    unsafe fn image(&self) -> &sys::avifImage;
}
