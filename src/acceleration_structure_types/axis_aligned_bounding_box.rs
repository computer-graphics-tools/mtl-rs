use objc2::{Encode, Encoding, RefEncode};

use crate::MTLPackedFloat3;

/// An axis aligned bounding box with a minimum and maximum point.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MTLAxisAlignedBoundingBox {
    /// Minimum point.
    pub min: MTLPackedFloat3,
    /// Maximum point.
    pub max: MTLPackedFloat3,
}

unsafe impl Encode for MTLAxisAlignedBoundingBox {
    const ENCODING: Encoding = Encoding::Struct(
        "MTLAxisAlignedBoundingBox",
        &[<MTLPackedFloat3>::ENCODING, <MTLPackedFloat3>::ENCODING],
    );
}

unsafe impl RefEncode for MTLAxisAlignedBoundingBox {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
