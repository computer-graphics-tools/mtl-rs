use objc2::{Encode, Encoding, RefEncode};

/// Type of end cap to insert at the beginning and end of each connected
/// sequence of curve segments (from `MTLCurveEndCaps`).
///
/// Availability: API_AVAILABLE(macos(14.0), ios(17.0))
#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MTLCurveEndCaps {
    /// No end caps
    None = 0,
    /// Disk end caps
    Disk = 1,
    /// Spherical end caps
    Sphere = 2,
}

unsafe impl Encode for MTLCurveEndCaps {
    const ENCODING: Encoding = u64::ENCODING;
}
unsafe impl RefEncode for MTLCurveEndCaps {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
