use objc2::{Encode, Encoding, RefEncode};

/// Blend factor (from `MTLBlendFactor`).
#[repr(u64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum MTLBlendFactor {
    Zero = 0,
    One = 1,
    SourceColor = 2,
    OneMinusSourceColor = 3,
    SourceAlpha = 4,
    OneMinusSourceAlpha = 5,
    DestinationColor = 6,
    OneMinusDestinationColor = 7,
    DestinationAlpha = 8,
    OneMinusDestinationAlpha = 9,
    SourceAlphaSaturated = 10,
    BlendColor = 11,
    OneMinusBlendColor = 12,
    BlendAlpha = 13,
    OneMinusBlendAlpha = 14,
    Source1Color = 15,
    OneMinusSource1Color = 16,
    Source1Alpha = 17,
    OneMinusSource1Alpha = 18,
    /// Defers assigning the blend factor. See Apple docs.
    Unspecialized = 19,
}

unsafe impl Encode for MTLBlendFactor {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for MTLBlendFactor {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
