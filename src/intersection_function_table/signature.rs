use objc2::{Encode, Encoding, RefEncode};

/// Signature defining what data is provided to an intersection function (from `MTLIntersectionFunctionSignature`).
///
/// Availability: macOS 11.0+, iOS 14.0+, tvOS 16.0+
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLIntersectionFunctionSignature(pub u64);
bitflags::bitflags! {
    impl MTLIntersectionFunctionSignature: u64 {
        /// Availability: macOS 11.0+, iOS 14.0+, tvOS 16.0+
        const None = 0;
        /// Intersection functions can read the built-in `instance_id`.
        /// Availability: macOS 11.0+, iOS 14.0+, tvOS 16.0+
        const Instancing = 1<<0;
        /// Triangle intersection functions can read `barycentric_coord` and `front_facing`.
        /// Availability: macOS 11.0+, iOS 14.0+, tvOS 16.0+
        const TriangleData = 1<<1;
        /// Intersection functions can query `world_space_origin` and `world_space_direction`.
        /// Availability: macOS 11.0+, iOS 14.0+, tvOS 16.0+
        const WorldSpaceData = 1<<2;
        /// May be called from intersectors using the `instance_motion` tag.
        /// Availability: macOS 12.0+, iOS 15.0+, tvOS 16.0+
        const InstanceMotion = 1<<3;
        /// Can query `time`, `motion_start_time`, `motion_end_time`, `key_frame_count`.
        /// Availability: macOS 12.0+, iOS 15.0+, tvOS 16.0+
        const PrimitiveMotion = 1<<4;
        /// May be called from intersectors using the `extended_limits` tag.
        /// Availability: macOS 12.0+, iOS 15.0+, tvOS 16.0+
        const ExtendedLimits = 1<<5;
        /// May be called from intersectors using the `max_levels` tag.
        /// Availability: macOS 14.0+, iOS 17.0+
        const MaxLevels = 1<<6;
        /// Curve intersection functions can read `curve_parameter`.
        /// Availability: macOS 14.0+, iOS 17.0+
        const CurveData = 1<<7;
        /// Intersection function will be used with intersection function buffers.
        /// Availability: macOS 26.0+, iOS 26.0+
        const IntersectionFunctionBuffer = 1<<8;
        /// Intersection function uses the intersection function buffer `user_data` pointer.
        /// Availability: macOS 26.0+, iOS 26.0+
        const UserData = 1<<9;
    }
}

unsafe impl Encode for MTLIntersectionFunctionSignature {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for MTLIntersectionFunctionSignature {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
