use core::ffi::c_float;

use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
    runtime::NSObject,
};
use objc2_foundation::{CopyingHelper, NSArray, NSCopying, NSObjectProtocol};

use crate::{
    MTLAccelerationStructureDescriptor, MTLAccelerationStructureGeometryDescriptor,
    MTLMotionBorderMode,
};

extern_class!(
    /// Descriptor for a primitive acceleration structure
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metal/mtlprimitiveaccelerationstructuredescriptor?language=objc)
    #[unsafe(super(MTLAccelerationStructureDescriptor, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLPrimitiveAccelerationStructureDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for MTLPrimitiveAccelerationStructureDescriptor {}
);

unsafe impl CopyingHelper for MTLPrimitiveAccelerationStructureDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLPrimitiveAccelerationStructureDescriptor {}
);

impl MTLPrimitiveAccelerationStructureDescriptor {
    extern_methods!(
        /// Array of geometry descriptors. If motionKeyframeCount is greater than one all geometryDescriptors
        /// must be motion versions and have motionKeyframeCount of primitive buffers.
        #[unsafe(method(geometryDescriptors))]
        #[unsafe(method_family = none)]
        pub fn geometry_descriptors(
            &self,
        ) -> Option<Retained<NSArray<MTLAccelerationStructureGeometryDescriptor>>>;

        /// Setter for [`geometryDescriptors`][Self::geometryDescriptors].
        #[unsafe(method(setGeometryDescriptors:))]
        #[unsafe(method_family = none)]
        pub fn set_geometry_descriptors(
            &self,
            geometry_descriptors: Option<&NSArray<MTLAccelerationStructureGeometryDescriptor>>,
        );

        /// Motion border mode describing what happens if acceleration structure is sampled before
        /// motionStartTime. If not set defaults to MTLMotionBorderModeClamp.
        #[unsafe(method(motionStartBorderMode))]
        #[unsafe(method_family = none)]
        pub fn motion_start_border_mode(&self) -> MTLMotionBorderMode;

        /// Setter for [`motionStartBorderMode`][Self::motionStartBorderMode].
        #[unsafe(method(setMotionStartBorderMode:))]
        #[unsafe(method_family = none)]
        pub fn set_motion_start_border_mode(
            &self,
            motion_start_border_mode: MTLMotionBorderMode,
        );

        /// Motion border mode describing what happens if acceleration structure is sampled after
        /// motionEndTime. If not set defaults to MTLMotionBorderModeClamp.
        #[unsafe(method(motionEndBorderMode))]
        #[unsafe(method_family = none)]
        pub fn motion_end_border_mode(&self) -> MTLMotionBorderMode;

        /// Setter for [`motionEndBorderMode`][Self::motionEndBorderMode].
        #[unsafe(method(setMotionEndBorderMode:))]
        #[unsafe(method_family = none)]
        pub fn set_motion_end_border_mode(
            &self,
            motion_end_border_mode: MTLMotionBorderMode,
        );

        /// Motion start time of this geometry. If not set defaults to 0.0f.
        #[unsafe(method(motionStartTime))]
        #[unsafe(method_family = none)]
        pub fn motion_start_time(&self) -> c_float;

        /// Setter for [`motionStartTime`][Self::motionStartTime].
        #[unsafe(method(setMotionStartTime:))]
        #[unsafe(method_family = none)]
        pub fn set_motion_start_time(&self, motion_start_time: c_float);

        /// Motion end time of this geometry. If not set defaults to 1.0f.
        #[unsafe(method(motionEndTime))]
        #[unsafe(method_family = none)]
        pub fn motion_end_time(&self) -> c_float;

        /// Setter for [`motionEndTime`][Self::motionEndTime].
        #[unsafe(method(setMotionEndTime:))]
        #[unsafe(method_family = none)]
        pub fn set_motion_end_time(&self, motion_end_time: c_float);

        /// Motion keyframe count. Is 1 by default which means no motion.
        #[unsafe(method(motionKeyframeCount))]
        #[unsafe(method_family = none)]
        pub fn motion_keyframe_count(&self) -> usize;

        /// Setter for [`motionKeyframeCount`][Self::motionKeyframeCount].
        #[unsafe(method(setMotionKeyframeCount:))]
        #[unsafe(method_family = none)]
        pub fn set_motion_keyframe_count(&self, motion_keyframe_count: usize);

        #[unsafe(method(descriptor))]
        #[unsafe(method_family = none)]
        pub fn descriptor() -> Retained<Self>;
    );
}

/// Methods declared on superclass `NSObject`.
impl MTLPrimitiveAccelerationStructureDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub fn new() -> Retained<Self>;
    );
}
