use objc2::{Encode, Encoding, RefEncode};

/// Status of an IO command buffer (ported from `MTLIOStatus`).
///
/// Availability: macOS 13.0+, iOS 16.0+
#[repr(i64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum MTLIOStatus {
    Pending = 0,
    Cancelled = 1,
    Error = 2,
    Complete = 3,
}

unsafe impl Encode for MTLIOStatus {
    const ENCODING: Encoding = i64::ENCODING;
}

unsafe impl RefEncode for MTLIOStatus {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
