use objc2::{Encode, Encoding, RefEncode};

/// The error codes that Metal can raise when you create a tensor (from `MTLTensorError`).
#[repr(i64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum MTLTensorError {
    /// No error occurred.
    None = 0,
    /// An internal error occurred.
    InternalError = 1,
    /// The descriptor was invalid.
    InvalidDescriptor = 2,
}

unsafe impl Encode for MTLTensorError {
    const ENCODING: Encoding = i64::ENCODING;
}

unsafe impl RefEncode for MTLTensorError {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
