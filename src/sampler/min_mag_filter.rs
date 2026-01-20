use objc2::{Encode, Encoding, RefEncode};

/// Options for filtering texels within a mip level (from `MTLSamplerMinMagFilter`).
#[repr(u64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum MTLSamplerMinMagFilter {
    Nearest = 0,
    Linear = 1,
}

unsafe impl Encode for MTLSamplerMinMagFilter {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for MTLSamplerMinMagFilter {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
