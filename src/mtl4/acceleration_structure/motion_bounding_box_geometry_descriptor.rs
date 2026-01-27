use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
};
use objc2_foundation::{CopyingHelper, NSCopying, NSObject, NSObjectProtocol, NSUInteger};

use crate::*;

extern_class!(
    /// Describes motion bounding box geometry, suitable for motion ray tracing.
    ///
    /// You use bounding boxes to implement procedural geometry for ray tracing, such as spheres or any other shape
    /// you define by using intersection functions.
    ///
    /// Use a ``MTLResidencySet`` to mark residency of all buffers this descriptor references when you build this
    /// acceleration structure.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metal/mtl4accelerationstructuremotionboundingboxgeometrydescriptor?language=objc)
    #[unsafe(super(MTL4AccelerationStructureGeometryDescriptor, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTL4AccelerationStructureMotionBoundingBoxGeometryDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for MTL4AccelerationStructureMotionBoundingBoxGeometryDescriptor {}
);

unsafe impl CopyingHelper for MTL4AccelerationStructureMotionBoundingBoxGeometryDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTL4AccelerationStructureMotionBoundingBoxGeometryDescriptor {}
);

impl MTL4AccelerationStructureMotionBoundingBoxGeometryDescriptor {
    extern_methods!(
        /// Configures a reference to a buffer where each entry contains a reference to a buffer of bounding boxes.
        ///
        /// This property references a buffer that conceptually represents an array with one entry for each keyframe in the
        /// motion animation. Each one of these entries consists of a ``MTL4BufferRange`` that, in turn, references a
        /// vertex buffer containing the bounding box data for the keyframe.
        ///
        /// You are responsible for ensuring the buffer address is not zero for the top-level buffer, as well as for all
        /// the vertex buffers it references.
        #[unsafe(method(boundingBoxBuffers))]
        #[unsafe(method_family = none)]
        pub fn bounding_box_buffers(&self) -> MTL4BufferRange;

        /// Setter for [`boundingBoxBuffers`][Self::boundingBoxBuffers].
        #[unsafe(method(setBoundingBoxBuffers:))]
        #[unsafe(method_family = none)]
        pub fn set_bounding_box_buffers(&self, bounding_box_buffers: MTL4BufferRange);

        /// Declares the stride, in bytes, between bounding boxes in the bounding box buffers each entry in `boundingBoxBuffer`
        /// references.
        ///
        /// All keyframes share the same bounding box stride. You are responsible for ensuring this stride is at least 24 bytes
        /// and a multiple of 4 bytes.
        ///
        /// This property defaults to `24` bytes.
        #[unsafe(method(boundingBoxStride))]
        #[unsafe(method_family = none)]
        pub fn bounding_box_stride(&self) -> NSUInteger;

        /// Setter for [`boundingBoxStride`][Self::boundingBoxStride].
        #[unsafe(method(setBoundingBoxStride:))]
        #[unsafe(method_family = none)]
        pub fn set_bounding_box_stride(&self, bounding_box_stride: NSUInteger);

        /// Declares the number of bounding boxes in each buffer that `boundingBoxBuffer` references.
        ///
        /// All keyframes share the same bounding box count.
        #[unsafe(method(boundingBoxCount))]
        #[unsafe(method_family = none)]
        pub fn bounding_box_count(&self) -> NSUInteger;

        /// Setter for [`boundingBoxCount`][Self::boundingBoxCount].
        #[unsafe(method(setBoundingBoxCount:))]
        #[unsafe(method_family = none)]
        pub fn set_bounding_box_count(&self, bounding_box_count: NSUInteger);
    );
}

/// Methods declared on superclass `NSObject`.
impl MTL4AccelerationStructureMotionBoundingBoxGeometryDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub fn new() -> Retained<Self>;
    );
}
