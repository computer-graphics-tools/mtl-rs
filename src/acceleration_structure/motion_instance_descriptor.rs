use objc2::{Encode, Encoding, RefEncode};

use super::{MTLAccelerationStructureInstanceOptions, MTLMotionBorderMode};

/// Motion instance descriptor describing per-instance motion parameters
/// (from `MTLAccelerationStructureMotionInstanceDescriptor`).
///
/// Availability: API_AVAILABLE(macos(12.0), ios(15.0), tvos(16.0))
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MTLAccelerationStructureMotionInstanceDescriptor {
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

unsafe impl Encode for MTLAccelerationStructureMotionInstanceDescriptor {
    const ENCODING: Encoding = Encoding::Struct(
        "{MTLAccelerationStructureMotionInstanceDescriptor=}",
        &[
            <MTLAccelerationStructureInstanceOptions>::ENCODING,
            <u32>::ENCODING,
            <u32>::ENCODING,
            <u32>::ENCODING,
            <u32>::ENCODING,
            <u32>::ENCODING,
            <u32>::ENCODING,
            <MTLMotionBorderMode>::ENCODING,
            <MTLMotionBorderMode>::ENCODING,
            <f32>::ENCODING,
            <f32>::ENCODING,
        ],
    );
}

unsafe impl RefEncode for MTLAccelerationStructureMotionInstanceDescriptor {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
