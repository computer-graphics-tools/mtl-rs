use objc2::{
    extern_class, extern_conformance, extern_methods, msg_send,
    rc::{Allocated, Retained},
    runtime::NSObject,
};
use objc2_foundation::NSObjectProtocol;

use super::MTLAccelerationStructurePassSampleBufferAttachmentDescriptor;

extern_class!(
    /// Array of acceleration structure pass sample buffer attachment descriptors.
    ///
    /// Availability: macOS 13.0+, iOS 16.0+
    ///
    /// This array uses copy semantics. It is safe to set the attachment state
    /// at any legal index to `None`, which resets that attachment descriptor's
    /// state to default values.
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLAccelerationStructurePassSampleBufferAttachmentDescriptorArray;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLAccelerationStructurePassSampleBufferAttachmentDescriptorArray {}
);

impl MTLAccelerationStructurePassSampleBufferAttachmentDescriptorArray {
    /// Returns the attachment descriptor at `attachment_index` for individual attachment state access.
    pub fn get(
        &self,
        attachment_index: usize,
    ) -> Retained<MTLAccelerationStructurePassSampleBufferAttachmentDescriptor> {
        unsafe { msg_send![self, objectAtIndexedSubscript: attachment_index] }
    }
    /// Sets the attachment descriptor at `attachment_index`.
    ///
    /// This always uses copy semantics. It is safe to pass `None` at any legal index, which resets that
    /// attachment descriptor's state to default values.
    pub fn set(
        &self,
        attachment_index: usize,
        attachment: Option<&MTLAccelerationStructurePassSampleBufferAttachmentDescriptor>,
    ) {
        unsafe {
            let _: () = msg_send![self, setObject: attachment, atIndexedSubscript: attachment_index];
        }
    }
}

impl MTLAccelerationStructurePassSampleBufferAttachmentDescriptorArray {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub fn new() -> Retained<Self>;
    );
}
