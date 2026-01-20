use objc2::{Encode, Encoding, RefEncode};

/// Describes how vertex data is shared among vertices in a vertex buffer (from `MTLVertexStepFunction`).
///
/// Available since macOS 10.11, iOS 8.0.
#[repr(u64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum MTLVertexStepFunction {
    /// The vertex data is shared among all vertices in a primitive.
    ///
    /// Available since macOS 10.11, iOS 8.0.
    Constant = 0,

    /// The vertex data is shared among all vertices in a primitive.
    ///
    /// Available since macOS 10.11, iOS 8.0.
    PerVertex = 1,

    /// The vertex data is shared among all vertices in an instance.
    ///
    /// Available since macOS 10.11, iOS 8.0.
    PerInstance = 2,

    /// The vertex data is shared among all vertices in a patch.
    ///
    /// Available since macOS 10.12, iOS 10.0.
    PerPatch = 3,

    /// The vertex data is shared among all vertices in a patch control point.
    ///
    /// Available since macOS 10.12, iOS 10.0.
    PerPatchControlPoint = 4,
}

unsafe impl Encode for MTLVertexStepFunction {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for MTLVertexStepFunction {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
