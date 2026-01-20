use objc2::{Encode, Encoding, RefEncode};

/// The type used for mesh/vertex indices.
///
/// Availability: macOS 10.11+, iOS 8.0+
#[repr(u64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum MTLIndexType {
    /// 16-bit unsigned integer indices.
    UInt16 = 0,
    /// 32-bit unsigned integer indices.
    UInt32 = 1,
}

unsafe impl Encode for MTLIndexType {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for MTLIndexType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
