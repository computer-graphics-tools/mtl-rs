use objc2::{Encode, Encoding, RefEncode};

/// Type of instance descriptor layout used in acceleration structures (from
/// `MTLAccelerationStructureInstanceDescriptorType`).
///
/// Availability: API_AVAILABLE(macos(12.0), ios(15.0), tvos(16.0))
#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MTLAccelerationStructureInstanceDescriptorType {
    /// Default instance descriptor: `MTLAccelerationStructureInstanceDescriptor`.
    Default = 0,
    /// Instance descriptor with an added user-ID.
    UserID = 1,
    /// Instance descriptor with support for motion.
    Motion = 2,
    /// Instance descriptor with a resource handle for the instanced acceleration structure.
    ///
    /// Availability: API_AVAILABLE(macos(14.0), ios(17.0))
    Indirect = 3,
    /// Motion instance descriptor with a resource handle for the instanced acceleration structure.
    ///
    /// Availability: API_AVAILABLE(macos(14.0), ios(17.0))
    IndirectMotion = 4,
}
unsafe impl Encode for MTLAccelerationStructureInstanceDescriptorType {
    const ENCODING: Encoding = u64::ENCODING;
}
unsafe impl RefEncode for MTLAccelerationStructureInstanceDescriptorType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
