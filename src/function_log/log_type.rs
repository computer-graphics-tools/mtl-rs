use objc2::{Encode, Encoding, RefEncode};

/// Function log type (from `MTLFunctionLogType`).
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLFunctionLogType(pub usize);

impl MTLFunctionLogType {
    pub const VALIDATION: Self = Self(0);
}

unsafe impl Encode for MTLFunctionLogType {
    const ENCODING: Encoding = usize::ENCODING;
}

unsafe impl RefEncode for MTLFunctionLogType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
