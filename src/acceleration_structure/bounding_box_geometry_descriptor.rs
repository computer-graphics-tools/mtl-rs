use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
    runtime::{NSObject, ProtocolObject},
};
use objc2_foundation::{CopyingHelper, NSCopying, NSObjectProtocol};

use crate::{MTLAccelerationStructureGeometryDescriptor, MTLBuffer};

extern_class!(
    /// Descriptor for bounding box geometry
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metal/mtlaccelerationstructureboundingboxgeometrydescriptor?language=objc)
    #[unsafe(super(MTLAccelerationStructureGeometryDescriptor, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLAccelerationStructureBoundingBoxGeometryDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for MTLAccelerationStructureBoundingBoxGeometryDescriptor {}
);

unsafe impl CopyingHelper for MTLAccelerationStructureBoundingBoxGeometryDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLAccelerationStructureBoundingBoxGeometryDescriptor {}
);

impl MTLAccelerationStructureBoundingBoxGeometryDescriptor {
    extern_methods!(
        /// Bounding box buffer containing MTLAxisAlignedBoundingBoxes. Must not be nil.
        #[unsafe(method(boundingBoxBuffer))]
        #[unsafe(method_family = none)]
        pub unsafe fn bounding_box_buffer(&self)
        -> Option<Retained<ProtocolObject<dyn MTLBuffer>>>;

        /// Setter for [`boundingBoxBuffer`][Self::boundingBoxBuffer].
        #[unsafe(method(setBoundingBoxBuffer:))]
        #[unsafe(method_family = none)]
        pub fn set_bounding_box_buffer(
            &self,
            bounding_box_buffer: Option<&ProtocolObject<dyn MTLBuffer>>,
        );

        /// Bounding box buffer offset. Must be a multiple of the bounding box stride and must be
        /// aligned to the platform's buffer offset alignment.
        #[unsafe(method(boundingBoxBufferOffset))]
        #[unsafe(method_family = none)]
        pub unsafe fn bounding_box_buffer_offset(&self) -> usize;

        /// Setter for [`boundingBoxBufferOffset`][Self::boundingBoxBufferOffset].
        #[unsafe(method(setBoundingBoxBufferOffset:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_bounding_box_buffer_offset(&self, bounding_box_buffer_offset: usize);

        /// Stride, in bytes, between bounding boxes in the bounding box buffer. Must be at least 24
        /// bytes and must be a multiple of 4 bytes. Defaults to 24 bytes.
        #[unsafe(method(boundingBoxStride))]
        #[unsafe(method_family = none)]
        pub unsafe fn bounding_box_stride(&self) -> usize;

        /// Setter for [`boundingBoxStride`][Self::boundingBoxStride].
        #[unsafe(method(setBoundingBoxStride:))]
        #[unsafe(method_family = none)]
        pub fn set_bounding_box_stride(&self, bounding_box_stride: usize);

        /// Number of bounding boxes
        #[unsafe(method(boundingBoxCount))]
        #[unsafe(method_family = none)]
        pub unsafe fn bounding_box_count(&self) -> usize;

        /// Setter for [`boundingBoxCount`][Self::boundingBoxCount].
        #[unsafe(method(setBoundingBoxCount:))]
        #[unsafe(method_family = none)]
        pub fn set_bounding_box_count(&self, bounding_box_count: usize);

        #[unsafe(method(descriptor))]
        #[unsafe(method_family = none)]
        pub fn descriptor() -> Retained<Self>;
    );
}

/// Methods declared on superclass `NSObject`.
impl MTLAccelerationStructureBoundingBoxGeometryDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
