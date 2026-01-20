use objc2::{Encode, Encoding, RefEncode};

/// Represents a memory size and alignment in bytes (from `MTLSizeAndAlign`).
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct MTLSizeAndAlign {
    /// Size in bytes
    pub size: usize,
    /// Alignment in bytes
    pub align: usize,
}

unsafe impl Encode for MTLSizeAndAlign {
    const ENCODING: Encoding =
        Encoding::Struct("{MTLSizeAndAlign=QQ}", &[usize::ENCODING, usize::ENCODING]);
}

unsafe impl RefEncode for MTLSizeAndAlign {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
