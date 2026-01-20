use objc2::{Encode, Encoding, RefEncode};

/// Errors when creating a log state (from `MTLLogStateError`).
///
/// Availability: macOS 15.0+, iOS 18.0+
#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MTLLogStateError {
    InvalidSize = 1,
    Invalid = 2,
}

unsafe impl Encode for MTLLogStateError {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for MTLLogStateError {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
