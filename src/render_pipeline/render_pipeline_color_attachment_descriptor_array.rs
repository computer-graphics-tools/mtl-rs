use objc2::{extern_class, extern_conformance, msg_send, rc::Retained, runtime::NSObject};
use objc2_foundation::NSObjectProtocol;

use super::MTLRenderPipelineColorAttachmentDescriptor;

extern_class!(
    /// Array wrapper for color attachment descriptors.
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLRenderPipelineColorAttachmentDescriptorArray;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLRenderPipelineColorAttachmentDescriptorArray {}
);

impl MTLRenderPipelineColorAttachmentDescriptorArray {
    /// Returns the attachment descriptor at `attachment_index` for individual attachment state access.
    pub fn get(
        &self,
        attachment_index: usize,
    ) -> Retained<MTLRenderPipelineColorAttachmentDescriptor> {
        unsafe { msg_send![self, objectAtIndexedSubscript: attachment_index] }
    }
    /// Sets the attachment descriptor at `attachment_index`.
    ///
    /// This always uses copy semantics. It is safe to pass `None` at any legal index, which resets that
    /// attachment descriptor's state to default values.
    pub fn set(
        &self,
        attachment_index: usize,
        attachment: Option<&MTLRenderPipelineColorAttachmentDescriptor>,
    ) {
        unsafe {
            let _: () = msg_send![self, setObject: attachment, atIndexedSubscript: attachment_index];
        }
    }
}
