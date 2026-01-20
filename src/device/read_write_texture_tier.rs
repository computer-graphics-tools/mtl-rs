use objc2::{Encode, Encoding, RefEncode};

/// Support level for read-write texture formats (ported from `MTLReadWriteTextureTier`).
#[repr(u64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum MTLReadWriteTextureTier {
    None = 0,
    Tier1 = 1,
    Tier2 = 2,
}

unsafe impl Encode for MTLReadWriteTextureTier {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for MTLReadWriteTextureTier {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
