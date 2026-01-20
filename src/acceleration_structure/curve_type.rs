use objc2::{Encode, Encoding, RefEncode};

/// Curve types (from `MTLCurveType`).
///
/// Availability: API_AVAILABLE(macos(14.0), ios(17.0))
#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MTLCurveType {
    /// Curve with a circular cross-section. These curves have the
    /// advantage of having a real 3D shape consistent across different ray
    /// directions, well-defined surface normals, etc. However, they may be
    /// slower to intersect. These curves are ideal for viewing close-up.
    Round = 0,
    /// Curve with a flat cross-section aligned with the ray direction.
    /// These curves may be faster to intersect but do not have a consistent
    /// 3D structure across different rays. These curves are ideal for viewing
    /// at a distance or curves with a small radius such as hair and fur.
    Flat = 1,
}

unsafe impl Encode for MTLCurveType {
    const ENCODING: Encoding = u64::ENCODING;
}
unsafe impl RefEncode for MTLCurveType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
