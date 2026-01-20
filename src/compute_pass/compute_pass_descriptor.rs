use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
    runtime::NSObject,
};
use objc2_foundation::{CopyingHelper, NSCopying, NSObjectProtocol};

use super::MTLComputePassSampleBufferAttachmentDescriptorArray;
use crate::compute_command_encoder::MTLDispatchType;

extern_class!(
    /// MTLComputePassDescriptor represents a collection of attachments to be used to create a concrete compute command encoder.
    ///
    /// Availability: macOS 11.0+, iOS 14.0+
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLComputePassDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for MTLComputePassDescriptor {}
);

unsafe impl CopyingHelper for MTLComputePassDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLComputePassDescriptor {}
);

impl MTLComputePassDescriptor {
    extern_methods!(
        /// Create an autoreleased default frame buffer descriptor.
        #[unsafe(method(computePassDescriptor))]
        #[unsafe(method_family = none)]
        pub unsafe fn compute_pass_descriptor() -> Retained<MTLComputePassDescriptor>;

        /// The dispatch type of the compute command encoder.
        #[unsafe(method(dispatchType))]
        #[unsafe(method_family = none)]
        pub fn dispatch_type(&self) -> MTLDispatchType;

        #[unsafe(method(setDispatchType:))]
        #[unsafe(method_family = none)]
        pub fn set_dispatch_type(&self, dispatch_type: MTLDispatchType);

        /// An array of sample buffers and associated sample indices.
        #[unsafe(method(sampleBufferAttachments))]
        #[unsafe(method_family = none)]
        pub fn sample_buffer_attachments(
            &self,
        ) -> Retained<MTLComputePassSampleBufferAttachmentDescriptorArray>;
    );
}

impl MTLComputePassDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
