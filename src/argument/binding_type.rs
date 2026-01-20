use objc2::{Encode, Encoding, RefEncode};

/// The type of a resource binding.
///
/// Availability: macOS 11.0+, iOS 14.0+
#[repr(i64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MTLBindingType {
    /// This binding represents a buffer.
    Buffer = 0,
    /// This binding represents threadgroup memory.
    ThreadgroupMemory = 1,
    /// This binding represents a texture.
    Texture = 2,
    /// This binding represents a sampler.
    Sampler = 3,
    /// This binding represents an image block data.
    ImageblockData = 16,
    /// This binding represents an image block.
    Imageblock = 17,
    /// This binding represents a visible function table object.
    VisibleFunctionTable = 24,
    /// This binding represents a primitive acceleration structure object.
    PrimitiveAccelerationStructure = 25,
    /// This binding represents an instance acceleration structure object.
    InstanceAccelerationStructure = 26,
    /// This binding represents an intersection function table object.
    IntersectionFunctionTable = 27,
    /// This binding represents an object payload.
    ObjectPayload = 34,
    /// This binding represents a tensor object.
    ///
    /// Availability: macOS 26.0+, iOS 26.0+
    Tensor = 37,
}

unsafe impl Encode for MTLBindingType {
    const ENCODING: Encoding = i64::ENCODING;
}

unsafe impl RefEncode for MTLBindingType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
