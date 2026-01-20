use objc2::{Encode, Encoding, RefEncode};

/// Errors emitted by binary archive operations.
///
/// Availability: macOS 11.0+, iOS 14.0+
#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MTLBinaryArchiveError {
    /// No error occurred.
    None = 0,
    /// The file provided is invalid.
    InvalidFile = 1,
    /// An unexpected element was encountered.
    UnexpectedElement = 2,
    /// The compilation of pipeline state or functions failed.
    CompilationFailure = 3,
    /// An internal error occurred.
    ///
    /// Availability: macOS 13.0+, iOS 16.0+
    InternalError = 4,
}

unsafe impl Encode for MTLBinaryArchiveError {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for MTLBinaryArchiveError {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
