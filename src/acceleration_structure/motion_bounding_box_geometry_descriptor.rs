use objc2::{
    extern_class, extern_conformance, extern_methods, msg_send,
    rc::{Allocated, Retained},
    runtime::NSObject,
};
use objc2_foundation::{CopyingHelper, NSArray, NSCopying, NSObjectProtocol};

use crate::{MTLAccelerationStructureGeometryDescriptor, MTLMotionKeyframeData};

extern_class!(
    /// Descriptor for motion bounding box geometry
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metal/mtlaccelerationstructuremotionboundingboxgeometrydescriptor?language=objc)
    #[unsafe(super(MTLAccelerationStructureGeometryDescriptor, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLAccelerationStructureMotionBoundingBoxGeometryDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for MTLAccelerationStructureMotionBoundingBoxGeometryDescriptor {}
);

unsafe impl CopyingHelper for MTLAccelerationStructureMotionBoundingBoxGeometryDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLAccelerationStructureMotionBoundingBoxGeometryDescriptor {}
);

impl MTLAccelerationStructureMotionBoundingBoxGeometryDescriptor {
    extern_methods!(
        /// Stride, in bytes, between bounding boxes in the bounding box buffer. Must be at least 24
        /// bytes and must be a multiple of 4 bytes. Defaults to 24 bytes.
        #[unsafe(method(boundingBoxStride))]
        #[unsafe(method_family = none)]
        pub fn bounding_box_stride(&self) -> usize;

        /// Setter for [`boundingBoxStride`][Self::boundingBoxStride].
        #[unsafe(method(setBoundingBoxStride:))]
        #[unsafe(method_family = none)]
        pub fn set_bounding_box_stride(
            &self,
            bounding_box_stride: usize,
        );

        /// Number of bounding boxes
        #[unsafe(method(boundingBoxCount))]
        #[unsafe(method_family = none)]
        pub fn bounding_box_count(&self) -> usize;

        /// Setter for [`boundingBoxCount`][Self::boundingBoxCount].
        #[unsafe(method(setBoundingBoxCount:))]
        #[unsafe(method_family = none)]
        pub fn set_bounding_box_count(
            &self,
            bounding_box_count: usize,
        );

        #[unsafe(method(descriptor))]
        #[unsafe(method_family = none)]
        pub fn descriptor() -> Retained<Self>;
    );

    /// Bounding box buffers for each keyframe.
    pub fn bounding_box_buffers(&self) -> Box<[Retained<MTLMotionKeyframeData>]> {
        let bounding_box_buffers: Retained<NSArray<MTLMotionKeyframeData>> =
            unsafe { msg_send![self, boundingBoxBuffers] };
        bounding_box_buffers.to_vec().into_boxed_slice()
    }

    pub fn set_bounding_box_buffers(
        &self,
        bounding_box_buffers: &[&MTLMotionKeyframeData],
    ) {
        let bounding_box_buffers = NSArray::from_slice(bounding_box_buffers);
        unsafe {
            let _: () = msg_send![self, setBoundingBoxBuffers: &*bounding_box_buffers];
        }
    }
}

/// Methods declared on superclass `NSObject`.
impl MTLAccelerationStructureMotionBoundingBoxGeometryDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub fn new() -> Retained<Self>;
    );
}
