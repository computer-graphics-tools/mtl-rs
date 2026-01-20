use objc2::{Encode, Encoding, RefEncode};

/// Addressing mode for out-of-bounds texture fetches (from `MTLSamplerAddressMode`).
#[repr(u64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum MTLSamplerAddressMode {
    ClampToEdge = 0,
    MirrorClampToEdge = 1,
    Repeat = 2,
    MirrorRepeat = 3,
    ClampToZero = 4,
    ClampToBorderColor = 5,
}

unsafe impl Encode for MTLSamplerAddressMode {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for MTLSamplerAddressMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
