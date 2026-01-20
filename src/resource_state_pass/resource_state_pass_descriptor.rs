use objc2::{extern_class, extern_conformance, extern_methods, rc::Retained, runtime::NSObject};
use objc2_foundation::{CopyingHelper, NSCopying, NSObjectProtocol};

use super::sample_buffer_attachment_descriptor_array::MTLResourceStatePassSampleBufferAttachmentDescriptorArray;

extern_class!(
    /// Resource state pass descriptor
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLResourceStatePassDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for MTLResourceStatePassDescriptor {}
);

unsafe impl CopyingHelper for MTLResourceStatePassDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLResourceStatePassDescriptor {}
);

impl MTLResourceStatePassDescriptor {
    extern_methods!(
        /// Create a default resource state pass descriptor
        #[unsafe(method(resourceStatePassDescriptor))]
        #[unsafe(method_family = none)]
        pub unsafe fn resource_state_pass_descriptor() -> Retained<MTLResourceStatePassDescriptor>;

        /// An array of sample buffers and associated sample indices.
        #[unsafe(method(sampleBufferAttachments))]
        #[unsafe(method_family = none)]
        pub unsafe fn sample_buffer_attachments(
            &self,
        ) -> Retained<MTLResourceStatePassSampleBufferAttachmentDescriptorArray>;
    );
}
