use objc2::{Encode, Encoding, RefEncode};

/// Border color used when clamping (from `MTLSamplerBorderColor`).
#[repr(u64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum MTLSamplerBorderColor {
    TransparentBlack = 0,
    OpaqueBlack = 1,
    OpaqueWhite = 2,
}

unsafe impl Encode for MTLSamplerBorderColor {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for MTLSamplerBorderColor {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
