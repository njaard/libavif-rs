mod borrowed;
mod chroma_position;
mod format;
mod owned;

pub use borrowed::BorrowedAvifImage;
pub use chroma_position::ChromaSamplePosition;
pub use format::YuvFormat;
pub use owned::AvifImage;

use libavif_sys as sys;

pub trait AvifImageRef {
    /// # Safety
    /// * The returned image must be a valid reference
    /// * The YUV and A planes must be null pointers _or_ point
    /// to non-freed memory with enough capacity `(plane_row_bytes * height)`
    unsafe fn image(&self) -> &sys::avifImage;
}
