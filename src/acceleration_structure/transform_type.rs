use objc2::{Encode, Encoding, RefEncode};

/// Transform type (from `MTLTransformType`).
///
/// Availability: API_AVAILABLE(macos(15.0), ios(18.0), tvos(18.1), visionos(2.1))
#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MTLTransformType {
    /// A tightly packed matrix with 4 columns and 3 rows. The full transform is assumed
    /// to be a 4x4 matrix with the last row being (0, 0, 0, 1).
    PackedFloat4x3 = 0,
    /// A transformation represented by individual components such as translation and
    /// rotation. The rotation is represented by a quaternion, allowing for correct motion
    /// interpolation.
    Component = 1,
}

unsafe impl Encode for MTLTransformType {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for MTLTransformType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
