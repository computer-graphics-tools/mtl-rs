use objc2::{Encode, Encoding, RefEncode};

/// Tessellation factor format (from `MTLTessellationFactorFormat`).
#[repr(u64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum MTLTessellationFactorFormat {
    Half = 0,
}

unsafe impl Encode for MTLTessellationFactorFormat {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for MTLTessellationFactorFormat {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
