use objc2::{Encode, Encoding, RefEncode};

#[repr(i64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MTLLibraryOptimizationLevel {
    Default = 0,
    Size = 1,
}

unsafe impl Encode for MTLLibraryOptimizationLevel {
    const ENCODING: Encoding = i64::ENCODING;
}
unsafe impl RefEncode for MTLLibraryOptimizationLevel {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}


