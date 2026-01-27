use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
    runtime::NSObject,
};
use objc2_foundation::{CopyingHelper, NSCopying, NSObjectProtocol};

use super::MTLBlitPassSampleBufferAttachmentDescriptorArray;

extern_class!(
    /// MTLBlitPassDescriptor represents a collection of attachments to be used to create a concrete blit command encoder.
    ///
    /// Availability: macOS 11.0+, iOS 14.0+
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLBlitPassDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for MTLBlitPassDescriptor {}
);

unsafe impl CopyingHelper for MTLBlitPassDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLBlitPassDescriptor {}
);

impl MTLBlitPassDescriptor {
    extern_methods!(
        /// Create an autoreleased default frame buffer descriptor.
        #[unsafe(method(blitPassDescriptor))]
        #[unsafe(method_family = none)]
        pub fn blit_pass_descriptor() -> Retained<MTLBlitPassDescriptor>;

        /// An array of sample buffers and associated sample indices.
        #[unsafe(method(sampleBufferAttachments))]
        #[unsafe(method_family = none)]
        pub fn sample_buffer_attachments(
            &self,
        ) -> Retained<MTLBlitPassSampleBufferAttachmentDescriptorArray>;
    );
}

impl MTLBlitPassDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub fn new() -> Retained<Self>;
    );
}
