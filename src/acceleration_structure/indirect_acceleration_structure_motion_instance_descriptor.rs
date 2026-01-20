use objc2::{Encode, Encoding, RefEncode};

use super::{MTLAccelerationStructureInstanceOptions, MTLMotionBorderMode};
use crate::MTLResourceID;

/// Motion instance descriptor with a resource handle for the instanced acceleration structure
/// (from `MTLIndirectAccelerationStructureMotionInstanceDescriptor`).
///
/// Availability: API_AVAILABLE(macos(14.0), ios(17.0))
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MTLIndirectAccelerationStructureMotionInstanceDescriptor {
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
    /// The index of the first set of transforms describing one keyframe of the animation.
    pub motion_transforms_start_index: u32,
    /// The count of motion transforms belonging to this motion.
    pub motion_transforms_count: u32,
    /// Motion border mode describing what happens if sampled before `motionStartTime`.
    pub motion_start_border_mode: MTLMotionBorderMode,
    /// Motion border mode describing what happens if sampled after `motionEndTime`.
    pub motion_end_border_mode: MTLMotionBorderMode,
    /// Motion start time of this instance.
    pub motion_start_time: f32,
    /// Motion end time of this instance.
    pub motion_end_time: f32,
}

unsafe impl Encode for MTLIndirectAccelerationStructureMotionInstanceDescriptor {
    const ENCODING: Encoding = Encoding::Struct(
        "{MTLIndirectAccelerationStructureMotionInstanceDescriptor=}",
        &[
            <MTLAccelerationStructureInstanceOptions>::ENCODING,
            <u32>::ENCODING,
            <u32>::ENCODING,
            <u32>::ENCODING,
            <MTLResourceID>::ENCODING,
            <u32>::ENCODING,
            <u32>::ENCODING,
            <MTLMotionBorderMode>::ENCODING,
            <MTLMotionBorderMode>::ENCODING,
            <f32>::ENCODING,
            <f32>::ENCODING,
        ],
    );
}

unsafe impl RefEncode for MTLIndirectAccelerationStructureMotionInstanceDescriptor {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
