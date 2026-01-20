use objc2::{Encode, Encoding, RefEncode};

/// Comparison function for depth/stencil tests.
///
/// Availability: macOS 10.11+, iOS 8.0+
#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MTLCompareFunction {
    Never = 0,
    Less = 1,
    Equal = 2,
    LessEqual = 3,
    Greater = 4,
    NotEqual = 5,
    GreaterEqual = 6,
    Always = 7,
}

unsafe impl Encode for MTLCompareFunction {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for MTLCompareFunction {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
