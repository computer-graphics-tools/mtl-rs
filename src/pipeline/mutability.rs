use objc2::{Encode, Encoding, RefEncode};

/// Specifies whether a buffer will be modified between binding and pipeline execution (from `MTLMutability`).
///
/// Availability: macOS 10.13+, iOS 11.0+
#[repr(u64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum MTLMutability {
    Default = 0,
    Mutable = 1,
    Immutable = 2,
}

unsafe impl Encode for MTLMutability {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for MTLMutability {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
