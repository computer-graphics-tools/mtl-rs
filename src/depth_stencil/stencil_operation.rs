use objc2::{Encode, Encoding, RefEncode};

/// Stencil buffer update operation for various test outcomes.
///
/// Availability: macOS 10.11+, iOS 8.0+
#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MTLStencilOperation {
    Keep = 0,
    Zero = 1,
    Replace = 2,
    IncrementClamp = 3,
    DecrementClamp = 4,
    Invert = 5,
    IncrementWrap = 6,
    DecrementWrap = 7,
}

unsafe impl Encode for MTLStencilOperation {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for MTLStencilOperation {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
