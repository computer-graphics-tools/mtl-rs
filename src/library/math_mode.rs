use objc2::{Encode, Encoding, RefEncode};

#[repr(i64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MTLMathMode {
    Safe = 0,
    Relaxed = 1,
    Fast = 2,
}

unsafe impl Encode for MTLMathMode {
    const ENCODING: Encoding = i64::ENCODING;
}
unsafe impl RefEncode for MTLMathMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
