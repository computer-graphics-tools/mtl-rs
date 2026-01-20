use objc2::{Encode, Encoding, RefEncode};

/// IO command queue type (ported from `MTLIOCommandQueueType`).
///
/// Availability: macOS 13.0+, iOS 16.0+
#[repr(i64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum MTLIOCommandQueueType {
    Concurrent = 0,
    Serial = 1,
}

unsafe impl Encode for MTLIOCommandQueueType {
    const ENCODING: Encoding = i64::ENCODING;
}

unsafe impl RefEncode for MTLIOCommandQueueType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
