use objc2::{Encode, Encoding, RefEncode};

/// An enumeration of the different data types in Metal (from `MTLDataType`).
#[repr(u64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum MTLDataType {
    /// Represents no data type.
    None = 0,

    /// Represents a struct data type.
    Struct = 1,
    /// Represents an array data type.
    Array = 2,

    /// A single floating-point value.
    Float = 3,
    /// A vector of two floating-point values.
    Float2 = 4,
    /// A vector of three floating-point values.
    Float3 = 5,
    /// A vector of four floating-point values.
    Float4 = 6,

    /// A 2x2 floating-point matrix.
    Float2x2 = 7,
    /// A 2x3 floating-point matrix.
    Float2x3 = 8,
    /// A 2x4 floating-point matrix.
    Float2x4 = 9,

    /// A 3x2 floating-point matrix.
    Float3x2 = 10,
    /// A 3x3 floating-point matrix.
    Float3x3 = 11,
    /// A 3x4 floating-point matrix.
    Float3x4 = 12,

    /// A 4x2 floating-point matrix.
    Float4x2 = 13,
    /// A 4x3 floating-point matrix.
    Float4x3 = 14,
    /// A 4x4 floating-point matrix.
    Float4x4 = 15,

    /// A single half-precision floating-point value.
    Half = 16,
    /// A vector of two half-precision floating-point values.
    Half2 = 17,
    /// A vector of three half-precision floating-point values.
    Half3 = 18,
    /// A vector of four half-precision floating-point values.
    Half4 = 19,

    /// A 2x2 half-precision floating-point matrix.
    Half2x2 = 20,
    /// A 2x3 half-precision floating-point matrix.
    Half2x3 = 21,
    /// A 2x4 half-precision floating-point matrix.
    Half2x4 = 22,

    /// A 3x2 half-precision floating-point matrix.
    Half3x2 = 23,
    /// A 3x3 half-precision floating-point matrix.
    Half3x3 = 24,
    /// A 3x4 half-precision floating-point matrix.
    Half3x4 = 25,

    /// A 4x2 half-precision floating-point matrix.
    Half4x2 = 26,
    /// A 4x3 half-precision floating-point matrix.
    Half4x3 = 27,
    /// A 4x4 half-precision floating-point matrix.
    Half4x4 = 28,

    /// A single signed integer value.
    Int = 29,
    /// A vector of two signed integer values.
    Int2 = 30,
    /// A vector of three signed integer values.
    Int3 = 31,
    /// A vector of four signed integer values.
    Int4 = 32,

    /// A single unsigned integer value.
    UInt = 33,
    /// A vector of two unsigned integer values.
    UInt2 = 34,
    /// A vector of three unsigned integer values.
    UInt3 = 35,
    /// A vector of four unsigned integer values.
    UInt4 = 36,

    /// A single 16-bit signed integer value.
    Short = 37,
    /// A vector of two 16-bit signed integer values.
    Short2 = 38,
    /// A vector of three 16-bit signed integer values.
    Short3 = 39,
    /// A vector of four 16-bit signed integer values.
    Short4 = 40,

    /// A single 16-bit unsigned integer value.
    UShort = 41,
    /// A vector of two 16-bit unsigned integer values.
    UShort2 = 42,
    /// A vector of three 16-bit unsigned integer values.
    UShort3 = 43,
    /// A vector of four 16-bit unsigned integer values.
    UShort4 = 44,

    /// A single signed character value.
    Char = 45,
    /// A vector of two signed character values.
    Char2 = 46,
    /// A vector of three signed character values.
    Char3 = 47,
    /// A vector of four signed character values.
    Char4 = 48,

    /// A single unsigned character value.
    UChar = 49,
    /// A vector of two unsigned character values.
    UChar2 = 50,
    /// A vector of three unsigned character values.
    UChar3 = 51,
    /// A vector of four unsigned character values.
    UChar4 = 52,

    /// A single boolean value.
    Bool = 53,
    /// A vector of two boolean values.
    Bool2 = 54,
    /// A vector of three boolean values.
    Bool3 = 55,
    /// A vector of four boolean values.
    Bool4 = 56,

    /// A data type corresponding to a texture object.
    Texture = 58,
    /// A data type corresponding to a sampler state object.
    Sampler = 59,
    /// A data type corresponding to a pointer.
    Pointer = 60,

    /// Image block: unsigned 8-bit red channel normalized to [0-1].
    R8Unorm = 62,
    /// Image block: signed 8-bit red channel normalized to [0-1].
    R8Snorm = 63,
    /// Image block: unsigned 16-bit red channel normalized to [0-1].
    R16Unorm = 64,
    /// Image block: signed 16-bit red channel normalized to [0-1].
    R16Snorm = 65,
    /// Image block: unsigned 8-bit RG channels normalized to [0-1].
    Rg8Unorm = 66,
    /// Image block: signed 8-bit RG channels normalized to [0-1].
    Rg8Snorm = 67,
    /// Image block: unsigned 16-bit RG channels normalized to [0-1].
    Rg16Unorm = 68,
    /// Image block: signed 16-bit RG channels normalized to [0-1].
    Rg16Snorm = 69,
    /// Image block: four unsigned 8-bit channels normalized to [0-1].
    Rgba8Unorm = 70,
    /// Image block: RGBA8Unorm with sRGB.
    Rgba8UnormSrgb = 71,
    /// Image block: four signed 8-bit channels normalized to [0-1].
    Rgba8Snorm = 72,
    /// Image block: four unsigned 16-bit channels normalized to [0-1].
    Rgba16Unorm = 73,
    /// Image block: four signed 16-bit channels normalized to [0-1].
    Rgba16Snorm = 74,
    /// Image block: three 10-bit unsigned channels and one 2-bit unsigned alpha channel.
    Rgb10a2Unorm = 75,
    /// Image block: two 11-bit float channels, one 10-bit float blue channel.
    Rg11b10Float = 76,
    /// Image block: three 9-bit float channels, one 5-bit float exponent.
    Rgb9e5Float = 77,

    /// Render pipeline state object.
    RenderPipeline = 78,
    /// Compute pipeline state object.
    ComputePipeline = 79,
    /// Indirect command buffer object.
    IndirectCommandBuffer = 80,

    /// A signed long integer value.
    Long = 81,
    /// A vector of two signed long integer values.
    Long2 = 82,
    /// A vector of three signed long integer values.
    Long3 = 83,
    /// A vector of four signed long integer values.
    Long4 = 84,

    /// An unsigned long integer value.
    ULong = 85,
    /// A vector of two unsigned long integer values.
    ULong2 = 86,
    /// A vector of three unsigned long integer values.
    ULong3 = 87,
    /// A vector of four unsigned long integer values.
    ULong4 = 88,

    /// A visible function table object.
    VisibleFunctionTable = 115,
    /// An intersection function table object.
    IntersectionFunctionTable = 116,
    /// A primitive acceleration structure.
    PrimitiveAccelerationStructure = 117,
    /// An instance acceleration structure.
    InstanceAccelerationStructure = 118,

    /// A single BFloat value.
    BFloat = 121,
    /// A vector of two BFloat values.
    BFloat2 = 122,
    /// A vector of three BFloat values.
    BFloat3 = 123,
    /// A vector of four BFloat values.
    BFloat4 = 124,

    /// A depth-stencil state object.
    DepthStencilState = 139,

    /// A machine learning tensor.
    Tensor = 140,
}

unsafe impl Encode for MTLDataType {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for MTLDataType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
