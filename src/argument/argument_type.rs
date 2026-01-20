use objc2::{Encode, Encoding, RefEncode};

/// The type for an input to a `MTLRenderPipelineState` or a `MTLComputePipelineState`.
///
/// Availability: macOS 10.11+, iOS 8.0+
///
/// Deprecated: Use `MTLBindingType` instead (macOS 13.0+, iOS 16.0+).
#[repr(u64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum MTLArgumentType {
    /// This input is a `MTLBuffer`.
    Buffer = 0,
    /// This input is a pointer to threadgroup memory.
    ThreadgroupMemory = 1,
    /// This input is a `MTLTexture`.
    Texture = 2,
    /// This input is a sampler.
    Sampler = 3,

    /// Imageblock data.
    ///
    /// Availability: macOS 11.0+, macCatalyst 14.0+, iOS 11.0+, tvOS 14.5+
    ImageblockData = 16,
    /// Imageblock.
    ///
    /// Availability: macOS 11.0+, macCatalyst 14.0+, iOS 11.0+, tvOS 14.5+
    Imageblock = 17,
    /// Visible function table.
    ///
    /// Availability: macOS 11.0+, iOS 14.0+, tvOS 16.0+
    VisibleFunctionTable = 24,
    /// Primitive acceleration structure.
    ///
    /// Availability: macOS 11.0+, iOS 14.0+, tvOS 16.0+
    PrimitiveAccelerationStructure = 25,
    /// Instance acceleration structure.
    ///
    /// Availability: macOS 11.0+, iOS 14.0+, tvOS 16.0+
    InstanceAccelerationStructure = 26,
    /// Intersection function table.
    ///
    /// Availability: macOS 11.0+, iOS 14.0+, tvOS 16.0+
    IntersectionFunctionTable = 27,
}

unsafe impl Encode for MTLArgumentType {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for MTLArgumentType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
