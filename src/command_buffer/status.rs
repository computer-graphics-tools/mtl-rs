use objc2::{Encode, Encoding, RefEncode};

/// Reports the current stage in the lifetime of a command buffer.
///
/// Availability: macOS 10.11+, iOS 8.0+
#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MTLCommandBufferStatus {
    NotEnqueued = 0,
    Enqueued = 1,
    Committed = 2,
    Scheduled = 3,
    Completed = 4,
    Error = 5,
}

unsafe impl Encode for MTLCommandBufferStatus {
    const ENCODING: Encoding = u64::ENCODING;
}
unsafe impl RefEncode for MTLCommandBufferStatus {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
