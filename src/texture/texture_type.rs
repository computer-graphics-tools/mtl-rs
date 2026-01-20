use objc2::{Encode, Encoding, RefEncode};

/// Describes texture dimensionality and arrangement (from `MTLTextureType`).
#[repr(u64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum MTLTextureType {
    Type1D = 0,
    Type1DArray = 1,
    Type2D = 2,
    Type2DArray = 3,
    Type2DMultisample = 4,
    Cube = 5,
    CubeArray = 6,
    Type3D = 7,
    Type2DMultisampleArray = 8,
    TextureBuffer = 9,
}

unsafe impl Encode for MTLTextureType {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for MTLTextureType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
