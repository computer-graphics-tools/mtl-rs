use objc2::{Encode, Encoding, RefEncode};

#[repr(i64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MTLCompileSymbolVisibility {
    Default = 0,
    Hidden = 1,
}

unsafe impl Encode for MTLCompileSymbolVisibility {
    const ENCODING: Encoding = i64::ENCODING;
}
unsafe impl RefEncode for MTLCompileSymbolVisibility {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
