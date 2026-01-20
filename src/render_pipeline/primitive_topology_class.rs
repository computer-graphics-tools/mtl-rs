use objc2::{Encode, Encoding, RefEncode};

/// Primitive topology class (from `MTLPrimitiveTopologyClass`).
#[repr(u64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum MTLPrimitiveTopologyClass {
    Unspecified = 0,
    Point = 1,
    Line = 2,
    Triangle = 3,
}

unsafe impl Encode for MTLPrimitiveTopologyClass {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for MTLPrimitiveTopologyClass {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
