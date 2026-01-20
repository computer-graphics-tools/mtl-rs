use objc2::{Encode, Encoding, RefEncode};

/// Controls whether visibility results accumulate between encoders (from `MTLVisibilityResultType`).
#[repr(i64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MTLVisibilityResultType {
    Reset = 0,
    Accumulate = 1,
}

unsafe impl Encode for MTLVisibilityResultType {
    const ENCODING: Encoding = i64::ENCODING;
}

unsafe impl RefEncode for MTLVisibilityResultType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
