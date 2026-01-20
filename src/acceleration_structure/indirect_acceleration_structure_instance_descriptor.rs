use objc2::{Encode, Encoding, RefEncode};

use crate::{MTLAccelerationStructureInstanceOptions, MTLPackedFloat4x3, MTLResourceID};

/// Indirect instance descriptor with a resource handle for the instanced acceleration structure
/// (from `MTLIndirectAccelerationStructureInstanceDescriptor`).
///
/// Availability: API_AVAILABLE(macos(14.0), ios(17.0))
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MTLIndirectAccelerationStructureInstanceDescriptor {
    /// Transformation matrix describing how to transform the bottom-level acceleration structure.
    pub transformation_matrix: MTLPackedFloat4x3,
    /// Instance options.
    pub options: MTLAccelerationStructureInstanceOptions,
    /// Instance mask used to ignore geometry during ray tracing.
    pub mask: u32,
    /// Used to index into intersection function tables.
    pub intersection_function_table_offset: u32,
    /// User-assigned instance ID to help identify this instance.
    pub user_id: u32,
    /// Acceleration structure resource handle to use for this instance.
    pub acceleration_structure_id: MTLResourceID,
}

unsafe impl Encode for MTLIndirectAccelerationStructureInstanceDescriptor {
    const ENCODING: Encoding = Encoding::Struct(
        "{MTLIndirectAccelerationStructureInstanceDescriptor=}",
        &[
            <MTLPackedFloat4x3>::ENCODING,
            <MTLAccelerationStructureInstanceOptions>::ENCODING,
            <u32>::ENCODING,
            <u32>::ENCODING,
            <u32>::ENCODING,
            <MTLResourceID>::ENCODING,
        ],
    );
}

unsafe impl RefEncode for MTLIndirectAccelerationStructureInstanceDescriptor {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
