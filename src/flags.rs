use libavif_sys as sys;
use std::fmt::{Binary, Formatter, LowerHex, Octal, UpperHex};
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
/// Flags when adding an image to the encoder using [`Encoder::add_image`](crate::Encoder::add_image).
pub struct AddImageFlags(sys::avifAddImageFlags);

impl AddImageFlags {
    /// Default flags.
    pub const NONE: Self = Self(sys::AVIF_ADD_IMAGE_FLAG_NONE);
    /// Force this frame to be a keyframe (sync frame).
    pub const KEYFRAME: Self = Self(sys::AVIF_ADD_IMAGE_FLAG_FORCE_KEYFRAME);
    /// Use this flag when encoding a single image. Signals "still_picture" to AV1 encoders, which
    /// tweaks various compression rules. This is enabled automatically when using the
    /// [`Encoder::encode`](crate::Encoder::encode) single-image encode path.
    pub const SINGLE: Self = Self(sys::AVIF_ADD_IMAGE_FLAG_SINGLE);
}

impl Binary for AddImageFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Binary::fmt(&self.0, f)
    }
}

impl BitAnd<AddImageFlags> for AddImageFlags {
    type Output = Self;

    fn bitand(self, rhs: AddImageFlags) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitAndAssign<AddImageFlags> for AddImageFlags {
    fn bitand_assign(&mut self, rhs: AddImageFlags) {
        self.0 &= rhs.0
    }
}

impl BitOr<AddImageFlags> for AddImageFlags {
    type Output = Self;

    fn bitor(self, rhs: AddImageFlags) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign<AddImageFlags> for AddImageFlags {
    fn bitor_assign(&mut self, rhs: AddImageFlags) {
        self.0 |= rhs.0
    }
}

impl BitXor<AddImageFlags> for AddImageFlags {
    type Output = Self;

    fn bitxor(self, rhs: AddImageFlags) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl BitXorAssign<AddImageFlags> for AddImageFlags {
    fn bitxor_assign(&mut self, rhs: AddImageFlags) {
        self.0 ^= rhs.0
    }
}

impl Default for AddImageFlags {
    fn default() -> Self {
        Self::NONE
    }
}

impl From<AddImageFlags> for sys::avifAddImageFlags {
    fn from(flags: AddImageFlags) -> Self {
        flags.0
    }
}

impl LowerHex for AddImageFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        LowerHex::fmt(&self.0, f)
    }
}

impl UpperHex for AddImageFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        UpperHex::fmt(&self.0, f)
    }
}

impl Octal for AddImageFlags {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Octal::fmt(&self.0, f)
    }
}

impl Not for AddImageFlags {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(Not::not(self.0))
    }
}
