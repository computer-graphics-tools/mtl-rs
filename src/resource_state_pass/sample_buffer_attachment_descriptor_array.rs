use objc2::{
    extern_class, extern_conformance, extern_methods, msg_send,
    rc::{Allocated, Retained},
    runtime::NSObject,
};
use objc2_foundation::NSObjectProtocol;

use super::sample_buffer_attachment_descriptor::MTLResourceStatePassSampleBufferAttachmentDescriptor;

extern_class!(
    /// Array of sample buffer attachment descriptors
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLResourceStatePassSampleBufferAttachmentDescriptorArray;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLResourceStatePassSampleBufferAttachmentDescriptorArray {}
);

impl MTLResourceStatePassSampleBufferAttachmentDescriptorArray {
    /// Returns the attachment descriptor at `attachment_index` for individual attachment state access.
    pub fn get(
        &self,
        attachment_index: usize,
    ) -> Retained<MTLResourceStatePassSampleBufferAttachmentDescriptor> {
        unsafe { msg_send![self, objectAtIndexedSubscript: attachment_index] }
    }
    /// Sets the attachment descriptor at `attachment_index`.
    ///
    /// This always uses copy semantics. It is safe to pass `None` at any legal index, which resets that
    /// attachment descriptor's state to default values.
    pub fn set(
        &self,
        attachment_index: usize,
        attachment: Option<&MTLResourceStatePassSampleBufferAttachmentDescriptor>,
    ) {
        unsafe {
            let _: () = msg_send![self, setObject: attachment, atIndexedSubscript: attachment_index];
        }
    }
}

/// Methods declared on superclass `NSObject`.
impl MTLResourceStatePassSampleBufferAttachmentDescriptorArray {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub fn new() -> Retained<Self>;
    );
}
