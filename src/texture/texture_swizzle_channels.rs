use objc2::{Encode, Encoding, RefEncode};

use crate::texture::MTLTextureSwizzle;

/// Channel swizzle to use when reading or sampling from the texture (from `MTLTextureSwizzleChannels`).
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct MTLTextureSwizzleChannels {
    pub red: MTLTextureSwizzle,
    pub green: MTLTextureSwizzle,
    pub blue: MTLTextureSwizzle,
    pub alpha: MTLTextureSwizzle,
}

unsafe impl Encode for MTLTextureSwizzleChannels {
    const ENCODING: Encoding = Encoding::Struct(
        "{MTLTextureSwizzleChannels=CCCC}",
        &[
            MTLTextureSwizzle::ENCODING,
            MTLTextureSwizzle::ENCODING,
            MTLTextureSwizzle::ENCODING,
            MTLTextureSwizzle::ENCODING,
        ],
    );
}

unsafe impl RefEncode for MTLTextureSwizzleChannels {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

impl MTLTextureSwizzleChannels {
    pub fn new(
        red: MTLTextureSwizzle,
        green: MTLTextureSwizzle,
        blue: MTLTextureSwizzle,
        alpha: MTLTextureSwizzle,
    ) -> Self {
        Self {
            red,
            green,
            blue,
            alpha,
        }
    }
}

impl Default for MTLTextureSwizzleChannels {
    fn default() -> Self {
        Self {
            red: MTLTextureSwizzle::Red,
            green: MTLTextureSwizzle::Green,
            blue: MTLTextureSwizzle::Blue,
            alpha: MTLTextureSwizzle::Alpha,
        }
    }
}
