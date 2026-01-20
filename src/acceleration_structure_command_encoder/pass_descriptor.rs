use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
    runtime::NSObject,
};
use objc2_foundation::{CopyingHelper, NSCopying, NSObjectProtocol};

use super::MTLAccelerationStructurePassSampleBufferAttachmentDescriptorArray;

extern_class!(
    /// Pass descriptor representing a collection of attachments used to create
    /// a concrete acceleration structure encoder.
    ///
    /// Availability: macOS 13.0+, iOS 16.0+
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLAccelerationStructurePassDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for MTLAccelerationStructurePassDescriptor {}
);

unsafe impl CopyingHelper for MTLAccelerationStructurePassDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLAccelerationStructurePassDescriptor {}
);

impl MTLAccelerationStructurePassDescriptor {
    extern_methods!(
        /// Create a default acceleration structure pass descriptor (autoreleased
        /// in Objective-C terms).
        #[unsafe(method(accelerationStructurePassDescriptor))]
        #[unsafe(method_family = none)]
        pub unsafe fn acceleration_structure_pass_descriptor()
        -> Retained<MTLAccelerationStructurePassDescriptor>;

        /// An array of sample buffers and associated sample indices.
        #[unsafe(method(sampleBufferAttachments))]
        #[unsafe(method_family = none)]
        pub unsafe fn sample_buffer_attachments(
            &self,
        ) -> Retained<MTLAccelerationStructurePassSampleBufferAttachmentDescriptorArray>;
    );
}

impl MTLAccelerationStructurePassDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
