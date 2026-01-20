use objc2::{Encode, Encoding, RefEncode};

use crate::{MTLPackedFloat3, MTLPackedFloatQuaternion};

/// A transformation represented by individual components such as translation and
/// rotation. The rotation is represented by a quaternion, allowing for correct
/// motion interpolation.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MTLComponentTransform {
    /// The scale of the instance applied before rotation alongside shear and pivot.
    pub scale: MTLPackedFloat3,
    /// The shear of the instance applied before rotation alongside scale and pivot.
    pub shear: MTLPackedFloat3,
    /// Translation applied before rotation alongside scale and shear.
    /// Allows rotation to pivot around a point.
    pub pivot: MTLPackedFloat3,
    /// The rotation of the instance as a normalized quaternion.
    /// Applied after scale, shear, and pivot and before translation.
    pub rotation: MTLPackedFloatQuaternion,
    /// The translation of the instance. Applied after rotation.
    /// Typically contains the composition of object translation and the inverse of the pivot translation.
    pub translation: MTLPackedFloat3,
}

unsafe impl Encode for MTLComponentTransform {
    const ENCODING: Encoding = Encoding::Struct(
        "MTLComponentTransform",
        &[
            <MTLPackedFloat3>::ENCODING,
            <MTLPackedFloat3>::ENCODING,
            <MTLPackedFloat3>::ENCODING,
            <MTLPackedFloatQuaternion>::ENCODING,
            <MTLPackedFloat3>::ENCODING,
        ],
    );
}

unsafe impl RefEncode for MTLComponentTransform {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
