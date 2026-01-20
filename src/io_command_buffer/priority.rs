use objc2::{Encode, Encoding, RefEncode};

/// IO priority for queues (ported from `MTLIOPriority`).
///
/// Availability: macOS 13.0+, iOS 16.0+
#[repr(i64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum MTLIOPriority {
    High = 0,
    Normal = 1,
    Low = 2,
}

unsafe impl Encode for MTLIOPriority {
    const ENCODING: Encoding = i64::ENCODING;
}

unsafe impl RefEncode for MTLIOPriority {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
