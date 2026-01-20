use objc2::{Encode, Encoding, RefEncode};

use crate::MTLPackedFloat3;

/// 4x3 matrix of packed float3 columns matching `MTLPackedFloat4x3` from Metal.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MTLPackedFloat4x3 {
    pub columns: [MTLPackedFloat3; 4],
}

unsafe impl Encode for MTLPackedFloat4x3 {
    const ENCODING: Encoding =
        Encoding::Struct("MTLPackedFloat4x3", &[<[MTLPackedFloat3; 4]>::ENCODING]);
}

unsafe impl RefEncode for MTLPackedFloat4x3 {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
