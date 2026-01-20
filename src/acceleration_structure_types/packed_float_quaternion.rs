use core::ffi::c_float;

use objc2::{Encode, Encoding, RefEncode};

/// Quaternion of 4 f32 values matching `MTLPackedFloatQuaternion` from Metal.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MTLPackedFloatQuaternion {
    pub x: c_float,
    pub y: c_float,
    pub z: c_float,
    pub w: c_float,
}

unsafe impl Encode for MTLPackedFloatQuaternion {
    const ENCODING: Encoding = Encoding::Struct(
        "MTLPackedFloatQuaternion",
        &[
            <c_float>::ENCODING,
            <c_float>::ENCODING,
            <c_float>::ENCODING,
            <c_float>::ENCODING,
        ],
    );
}

unsafe impl RefEncode for MTLPackedFloatQuaternion {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
