use objc2::{Encode, Encoding, RefEncode};

/// The execution state of a Metal command encoder after command-buffer execution.
///
/// Availability: macOS 11.0+, iOS 14.0+
#[repr(isize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MTLCommandEncoderErrorState {
    Unknown = 0,
    Completed = 1,
    Affected = 2,
    Pending = 3,
    Faulted = 4,
}

unsafe impl Encode for MTLCommandEncoderErrorState {
    const ENCODING: Encoding = isize::ENCODING;
}

unsafe impl RefEncode for MTLCommandEncoderErrorState {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
