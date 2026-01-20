use objc2::{Encode, Encoding, RefEncode};

/// Texture channel swizzle (from `MTLTextureSwizzle`).
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum MTLTextureSwizzle {
    Zero = 0,
    One = 1,
    Red = 2,
    Green = 3,
    Blue = 4,
    Alpha = 5,
}

unsafe impl Encode for MTLTextureSwizzle {
    const ENCODING: Encoding = u8::ENCODING;
}

unsafe impl RefEncode for MTLTextureSwizzle {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
