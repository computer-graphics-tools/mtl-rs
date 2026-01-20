use core::ffi::c_float;

use objc2::{
    extern_class, extern_conformance, extern_methods, msg_send,
    rc::{Allocated, Retained},
};
use objc2_foundation::{CopyingHelper, NSArray, NSCopying, NSObject, NSObjectProtocol, NSUInteger};

use crate::*;

extern_class!(
    /// Descriptor for a primitive acceleration structure that directly references geometric shapes, such as triangles and
    /// bounding boxes.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metal/mtl4primitiveaccelerationstructuredescriptor?language=objc)
    #[unsafe(super(
        MTL4AccelerationStructureDescriptor,
        MTLAccelerationStructureDescriptor,
        NSObject
    ))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTL4PrimitiveAccelerationStructureDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for MTL4PrimitiveAccelerationStructureDescriptor {}
);

unsafe impl CopyingHelper for MTL4PrimitiveAccelerationStructureDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTL4PrimitiveAccelerationStructureDescriptor {}
);

impl MTL4PrimitiveAccelerationStructureDescriptor {
    extern_methods!(
        /// Configures the behavior when the ray-tracing system samples the acceleration structure before the motion start time.
        ///
        /// Use this property to control the behavior when the ray-tracing system samples the acceleration structure
        /// at a time prior to the one you set for ``motionStartTime``.
        ///
        /// The default value of this property is `MTLMotionBorderModeClamp`.
        #[unsafe(method(motionStartBorderMode))]
        #[unsafe(method_family = none)]
        pub unsafe fn motion_start_border_mode(&self) -> MTLMotionBorderMode;

        /// Setter for [`motionStartBorderMode`][Self::motionStartBorderMode].
        #[unsafe(method(setMotionStartBorderMode:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_motion_start_border_mode(
            &self,
            motion_start_border_mode: MTLMotionBorderMode,
        );

        /// Configures the motion border mode.
        ///
        /// This property controls what happens if Metal samples the acceleration structure after ``motionEndTime``.
        ///
        /// Its default value is `MTLMotionBorderModeClamp`.
        #[unsafe(method(motionEndBorderMode))]
        #[unsafe(method_family = none)]
        pub unsafe fn motion_end_border_mode(&self) -> MTLMotionBorderMode;

        /// Setter for [`motionEndBorderMode`][Self::motionEndBorderMode].
        #[unsafe(method(setMotionEndBorderMode:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_motion_end_border_mode(
            &self,
            motion_end_border_mode: MTLMotionBorderMode,
        );

        /// Configures the motion start time for this geometry.
        ///
        /// The default value of this property is `0.0f`.
        #[unsafe(method(motionStartTime))]
        #[unsafe(method_family = none)]
        pub unsafe fn motion_start_time(&self) -> c_float;

        /// Setter for [`motionStartTime`][Self::motionStartTime].
        #[unsafe(method(setMotionStartTime:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_motion_start_time(&self, motion_start_time: c_float);

        /// Configures the motion end time for this geometry.
        ///
        /// The default value of this property is `1.0f`.
        #[unsafe(method(motionEndTime))]
        #[unsafe(method_family = none)]
        pub unsafe fn motion_end_time(&self) -> c_float;

        /// Setter for [`motionEndTime`][Self::motionEndTime].
        #[unsafe(method(setMotionEndTime:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_motion_end_time(&self, motion_end_time: c_float);

        /// Sets the motion keyframe count.
        ///
        /// This property's default is `1`, indicating no motion.
        #[unsafe(method(motionKeyframeCount))]
        #[unsafe(method_family = none)]
        pub unsafe fn motion_keyframe_count(&self) -> NSUInteger;

        /// Setter for [`motionKeyframeCount`][Self::motionKeyframeCount].
        #[unsafe(method(setMotionKeyframeCount:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_motion_keyframe_count(&self, motion_keyframe_count: NSUInteger);
    );
}

/// Methods declared on superclass `NSObject`.
impl MTL4PrimitiveAccelerationStructureDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

impl MTL4PrimitiveAccelerationStructureDescriptor {
    /// Associates the array of geometry descriptors that comprise this primitive acceleration structure.
    pub fn geometry_descriptors(
        &self,
    ) -> Option<Box<[Retained<MTL4AccelerationStructureGeometryDescriptor>]>> {
        let arr: Option<Retained<NSArray<MTL4AccelerationStructureGeometryDescriptor>>> =
            unsafe { msg_send![self, geometryDescriptors] };
        arr.map(|a| a.to_vec().into_boxed_slice())
    }

    /// Setter for geometry_descriptors.
    pub fn set_geometry_descriptors(
        &self,
        geometry_descriptors: Option<&[&MTL4AccelerationStructureGeometryDescriptor]>,
    ) {
        let ns_array = geometry_descriptors.map(NSArray::from_slice);
        unsafe {
            let _: () = msg_send![self, setGeometryDescriptors: ns_array.as_deref()];
        }
    }
}
