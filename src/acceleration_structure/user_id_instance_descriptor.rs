use objc2::{Encode, Encoding, RefEncode};

use crate::{MTLAccelerationStructureInstanceOptions, MTLPackedFloat4x3};

/// Instance descriptor with a user-assigned ID to help identify the instance (from
/// `MTLAccelerationStructureUserIDInstanceDescriptor`).
///
/// Availability: API_AVAILABLE(macos(12.0), ios(15.0), tvos(16.0))
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MTLAccelerationStructureUserIDInstanceDescriptor {
    /// Transformation matrix describing how to transform the bottom-level acceleration structure.
    pub transformation_matrix: MTLPackedFloat4x3,
    /// Instance options.
    pub options: MTLAccelerationStructureInstanceOptions,
    /// Instance mask used to ignore geometry during ray tracing.
    pub mask: u32,
    /// Used to index into intersection function tables.
    pub intersection_function_table_offset: u32,
    /// Acceleration structure index to use for this instance.
    pub acceleration_structure_index: u32,
    /// User-assigned instance ID to help identify this instance.
    pub user_id: u32,
}

unsafe impl Encode for MTLAccelerationStructureUserIDInstanceDescriptor {
    const ENCODING: Encoding = Encoding::Struct(
        "{MTLAccelerationStructureUserIDInstanceDescriptor=}",
        &[
            <MTLPackedFloat4x3>::ENCODING,
            <MTLAccelerationStructureInstanceOptions>::ENCODING,
            <u32>::ENCODING,
            <u32>::ENCODING,
            <u32>::ENCODING,
            <u32>::ENCODING,
        ],
    );
}

unsafe impl RefEncode for MTLAccelerationStructureUserIDInstanceDescriptor {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
