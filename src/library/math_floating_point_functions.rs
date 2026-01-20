use objc2::{Encode, Encoding, RefEncode};

#[repr(i64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MTLMathFloatingPointFunctions {
    Fast = 0,
    Precise = 1,
}

unsafe impl Encode for MTLMathFloatingPointFunctions {
    const ENCODING: Encoding = i64::ENCODING;
}
unsafe impl RefEncode for MTLMathFloatingPointFunctions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}


