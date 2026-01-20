use objc2::{Encode, Encoding, RefEncode};

#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MTLLibraryError {
    Unsupported = 1,
    Internal = 2,
    CompileFailure = 3,
    CompileWarning = 4,
    FunctionNotFound = 5,
    FileNotFound = 6,
}

unsafe impl Encode for MTLLibraryError {
    const ENCODING: Encoding = u64::ENCODING;
}
unsafe impl RefEncode for MTLLibraryError {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
