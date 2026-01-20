use objc2::{Encode, Encoding, RefEncode};

/// Dispatch type of the compute command encoder (from `MTLDispatchType`).
///
/// Availability: macOS 10.14+, iOS 12.0+
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLDispatchType(pub u64);

unsafe impl Encode for MTLDispatchType {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for MTLDispatchType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
