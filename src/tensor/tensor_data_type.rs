use objc2::{Encode, Encoding, RefEncode};

use crate::MTLDataType;

/// The possible data types for the elements of a tensor (from `MTLTensorDataType`).
#[repr(i64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum MTLTensorDataType {
    /// None
    None = MTLDataType::None as i64,
    /// 32-bit float
    Float32 = MTLDataType::Float as i64,
    /// 16-bit float
    Float16 = MTLDataType::Half as i64,
    /// 16-bit bfloat
    BFloat16 = MTLDataType::BFloat as i64,
    /// 8-bit signed integer
    Int8 = MTLDataType::Char as i64,
    /// 8-bit unsigned integer
    UInt8 = MTLDataType::UChar as i64,
    /// 16-bit signed integer
    Int16 = MTLDataType::Short as i64,
    /// 16-bit unsigned integer
    UInt16 = MTLDataType::UShort as i64,
    /// 32-bit signed integer
    Int32 = MTLDataType::Int as i64,
    /// 32-bit unsigned integer
    UInt32 = MTLDataType::UInt as i64,
}

unsafe impl Encode for MTLTensorDataType {
    const ENCODING: Encoding = i64::ENCODING;
}

unsafe impl RefEncode for MTLTensorDataType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
