use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
};
use objc2_foundation::{CopyingHelper, NSCopying, NSObject, NSObjectProtocol, NSUInteger};

use crate::*;

extern_class!(
    /// Describes bounding-box geometry suitable for ray tracing.
    ///
    /// You use bounding boxes to implement procedural geometry for ray tracing, such as spheres or any other shape
    /// you define by using intersection functions.
    ///
    /// Use a ``MTLResidencySet`` to mark residency of all buffers this descriptor references when you build this
    /// acceleration structure.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metal/mtl4accelerationstructureboundingboxgeometrydescriptor?language=objc)
    #[unsafe(super(MTL4AccelerationStructureGeometryDescriptor, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTL4AccelerationStructureBoundingBoxGeometryDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for MTL4AccelerationStructureBoundingBoxGeometryDescriptor {}
);

unsafe impl CopyingHelper for MTL4AccelerationStructureBoundingBoxGeometryDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTL4AccelerationStructureBoundingBoxGeometryDescriptor {}
);

impl MTL4AccelerationStructureBoundingBoxGeometryDescriptor {
    extern_methods!(
        /// References a buffer containing bounding box data in `MTLAxisAlignedBoundingBoxes` format.
        ///
        /// You are responsible for ensuring the buffer address of the range is not zero.
        #[unsafe(method(boundingBoxBuffer))]
        #[unsafe(method_family = none)]
        pub unsafe fn bounding_box_buffer(&self) -> MTL4BufferRange;

        /// Setter for [`boundingBoxBuffer`][Self::boundingBoxBuffer].
        #[unsafe(method(setBoundingBoxBuffer:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_bounding_box_buffer(&self, bounding_box_buffer: MTL4BufferRange);

        /// Assigns the stride, in bytes, between bounding boxes in the bounding box buffer `boundingBoxBuffer` references.
        ///
        /// You are responsible for ensuring this stride is at least 24 bytes and a multiple of 4 bytes.
        ///
        /// This property defaults to `24` bytes.
        #[unsafe(method(boundingBoxStride))]
        #[unsafe(method_family = none)]
        pub unsafe fn bounding_box_stride(&self) -> NSUInteger;

        /// Setter for [`boundingBoxStride`][Self::boundingBoxStride].
        #[unsafe(method(setBoundingBoxStride:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_bounding_box_stride(&self, bounding_box_stride: NSUInteger);

        /// Describes the number of bounding boxes the `boundingBoxBuffer` contains.
        #[unsafe(method(boundingBoxCount))]
        #[unsafe(method_family = none)]
        pub unsafe fn bounding_box_count(&self) -> NSUInteger;

        /// Setter for [`boundingBoxCount`][Self::boundingBoxCount].
        #[unsafe(method(setBoundingBoxCount:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_bounding_box_count(&self, bounding_box_count: NSUInteger);
    );
}

/// Methods declared on superclass `NSObject`.
impl MTL4AccelerationStructureBoundingBoxGeometryDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
