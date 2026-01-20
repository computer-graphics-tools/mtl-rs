use objc2::{Encode, Encoding, RefEncode};

/// Per-instance options.
///
/// Availability: API_AVAILABLE(macos(11.0), ios(14.0), tvos(16.0))
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLAccelerationStructureInstanceOptions(pub u32);

bitflags::bitflags! {
    impl MTLAccelerationStructureInstanceOptions: u32 {
        /// No options.
        const None = 0;

        /// Disable triangle back or front face culling.
        const DisableTriangleCulling = 1<<0;

        /// Override triangle front-facing winding. By default, the winding is assumed to be clockwise unless
        /// overridden by the intersector object. This overrides the intersector's winding order.
        const TriangleFrontFacingWindingCounterClockwise = 1<<1;

        /// Geometry is opaque.
        const Opaque = 1<<2;

        /// Geometry is non-opaque.
        const NonOpaque = 1<<3;
    }
}

unsafe impl Encode for MTLAccelerationStructureInstanceOptions {
    const ENCODING: Encoding = u32::ENCODING;
}

unsafe impl RefEncode for MTLAccelerationStructureInstanceOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
