use core::ffi::c_float;

use objc2::{Encode, Encoding, RefEncode};

/// Packed 3D float vector matching `MTLPackedFloat3` from Metal.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MTLPackedFloat3 {
    pub x: c_float,
    pub y: c_float,
    pub z: c_float,
}

unsafe impl Encode for MTLPackedFloat3 {
    const ENCODING: Encoding = Encoding::Struct(
        "MTLPackedFloat3",
        &[
            <c_float>::ENCODING,
            <c_float>::ENCODING,
            <c_float>::ENCODING,
        ],
    );
}

unsafe impl RefEncode for MTLPackedFloat3 {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
