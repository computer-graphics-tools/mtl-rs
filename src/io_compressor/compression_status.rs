use objc2::{Encode, Encoding, RefEncode};

/// Result status from flushing and destroying a Metal I/O compression context
/// (ported from `MTLIOCompressionStatus`).
///
/// Availability: macOS 13.0+, iOS 16.0+
#[repr(i64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum MTLCompressionStatus {
    /// Compression completed successfully.
    Complete = 0,
    /// Compression encountered an error.
    Error = 1,
}

unsafe impl Encode for MTLCompressionStatus {
    const ENCODING: Encoding = i64::ENCODING;
}

unsafe impl RefEncode for MTLCompressionStatus {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
