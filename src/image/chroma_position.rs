use libavif_sys as sys;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(i32)]
pub enum ChromaSamplePosition {
    Unknown = sys::AVIF_CHROMA_SAMPLE_POSITION_UNKNOWN as _,
    Vertical = sys::AVIF_CHROMA_SAMPLE_POSITION_VERTICAL as _,
    Colocated = sys::AVIF_CHROMA_SAMPLE_POSITION_COLOCATED as _,
}

impl Default for ChromaSamplePosition {
    fn default() -> Self {
        Self::Unknown
    }
}
