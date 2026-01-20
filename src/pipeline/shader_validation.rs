use objc2::{Encode, Encoding, RefEncode};

/// Shader validation mode (from `MTLShaderValidation`).
///
/// Availability: macOS 15.0+, iOS 18.0+
#[repr(i64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum MTLShaderValidation {
    Default = 0,
    Enabled = 1,
    Disabled = 2,
}

unsafe impl Encode for MTLShaderValidation {
    const ENCODING: Encoding = i64::ENCODING;
}

unsafe impl RefEncode for MTLShaderValidation {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
