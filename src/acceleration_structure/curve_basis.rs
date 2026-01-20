use objc2::{Encode, Encoding, RefEncode};

/// Basis function to use to interpolate curve control points (from `MTLCurveBasis`).
///
/// Availability: API_AVAILABLE(macos(14.0), ios(17.0))
#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MTLCurveBasis {
    /// B-Spline basis. Each curve segment must have 3 or 4 control
    /// points. Curve segments join with C^(N - 2) continuity, where N is
    /// the number of control points. The curve does not necessarily pass
    /// through the control points without additional control points at the
    /// beginning and end of the curve. Each curve segment can overlap
    /// N-1 control points.
    BSpline = 0,
    /// Catmull-Rom basis. Curves represented in this basis can also be
    /// easily converted to and from the Bézier basis. Each curve segment must
    /// have 4 control points. Each index in the control point index buffer
    /// points to the first of 4 consecutive control points in the control point
    /// buffer.
    ///
    /// The tangent at each control point is given by
    /// (P_(i+1) - P_(i-1)) / 2. Therefore, the curve does not pass through the
    /// first and last control point of each connected sequence of curve
    /// segments. Instead, the first and last control point are used to control
    /// the tangent vector at the beginning and end of the curve.
    ///
    /// Curve segments join with C^1 continuity and the
    /// curve passes through the control points. Each curve segment can overlap
    /// 3 control points.
    CatmullRom = 1,
    /// Linear basis. The curve is made of a sequence of connected line
    /// segments each with 2 control points.
    Linear = 2,
    /// Bezier basis
    Bezier = 3,
}

unsafe impl Encode for MTLCurveBasis {
    const ENCODING: Encoding = u64::ENCODING;
}
unsafe impl RefEncode for MTLCurveBasis {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
