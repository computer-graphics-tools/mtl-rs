use objc2::{Encode, Encoding, RefEncode};

#[repr(i64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MTLLibraryType {
    Executable = 0,
    Dynamic = 1,
}

unsafe impl Encode for MTLLibraryType {
    const ENCODING: Encoding = i64::ENCODING;
}
unsafe impl RefEncode for MTLLibraryType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
