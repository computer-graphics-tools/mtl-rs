use objc2::{Encode, Encoding, RefEncode};

use crate::{MTLAccelerationStructureInstanceOptions, MTLPackedFloat4x3};

/// Instance descriptor describing how to transform and reference a bottom-level acceleration structure
/// (from `MTLAccelerationStructureInstanceDescriptor`).
///
/// Availability: API_AVAILABLE(macos(11.0), ios(14.0))
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MTLAccelerationStructureInstanceDescriptor {
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
}

unsafe impl Encode for MTLAccelerationStructureInstanceDescriptor {
    const ENCODING: Encoding = Encoding::Struct(
        "{MTLAccelerationStructureInstanceDescriptor=}",
        &[
            <MTLPackedFloat4x3>::ENCODING,
            <MTLAccelerationStructureInstanceOptions>::ENCODING,
            <u32>::ENCODING,
            <u32>::ENCODING,
            <u32>::ENCODING,
        ],
    );
}

unsafe impl RefEncode for MTLAccelerationStructureInstanceDescriptor {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
