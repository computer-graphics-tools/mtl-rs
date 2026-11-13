use objc2::{Encode, Encoding, RefEncode};

use crate::MTLDataType;

/// The possible data types for the elements of a tensor (from `MTLTensorDataType`).
#[repr(isize)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum MTLTensorDataType {
    /// None
    None = MTLDataType::None as isize,
    /// 32-bit float
    Float32 = MTLDataType::Float as isize,
    /// 16-bit float
    Float16 = MTLDataType::Half as isize,
    /// 16-bit bfloat
    BFloat16 = MTLDataType::BFloat as isize,
    /// 8-bit signed integer
    Int8 = MTLDataType::Char as isize,
    /// 8-bit unsigned integer
    UInt8 = MTLDataType::UChar as isize,
    /// 16-bit signed integer
    Int16 = MTLDataType::Short as isize,
    /// 16-bit unsigned integer
    UInt16 = MTLDataType::UShort as isize,
    /// 32-bit signed integer
    Int32 = MTLDataType::Int as isize,
    /// 32-bit unsigned integer
    UInt32 = MTLDataType::UInt as isize,
    /// A 4-bit signed integer.
    Int4 = 143,
    /// A 4-bit unsigned integer.
    UInt4 = 144,
    /// An 8-bit floating-point value with 8 exponent bits, no mantissa, and no sign bit.
    MetalFloat8UE8M0 = 145,
    /// A 2-bit unsigned integer.
    UInt2 = 149,
    /// A 2-bit signed integer.
    Int2 = 150,
    /// An 8-bit floating-point value with 5 exponent bits, 2 mantissa bits, and 1 sign bit.
    MetalFloat8E5M2 = 141,
    /// An 8-bit floating-point value with 4 exponent bits, 3 mantissa bits, and 1 sign bit.
    MetalFloat8E4M3 = 142,
    /// A 4-bit floating-point value with 2 exponent bits, 1 mantissa bit, and 1 sign bit.
    MetalFloat4E2M1 = 148,
}

unsafe impl Encode for MTLTensorDataType {
    const ENCODING: Encoding = isize::ENCODING;
}

unsafe impl RefEncode for MTLTensorDataType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
