use objc2::{Encode, Encoding, RefEncode};

/// Support level for argument buffers (ported from `MTLArgumentBuffersTier`).
#[repr(u64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum MTLArgumentBuffersTier {
    Tier1 = 0,
    Tier2 = 1,
}

unsafe impl Encode for MTLArgumentBuffersTier {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for MTLArgumentBuffersTier {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
