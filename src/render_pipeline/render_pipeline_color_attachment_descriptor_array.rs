use objc2::{extern_class, extern_conformance, extern_methods, rc::Retained, runtime::NSObject};
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
    extern_methods!(
        #[unsafe(method(objectAtIndexedSubscript:))]
        #[unsafe(method_family = none)]
        pub unsafe fn object_at_indexed_subscript(
            &self,
            attachment_index: usize,
        ) -> Retained<MTLRenderPipelineColorAttachmentDescriptor>;

        #[unsafe(method(setObject:atIndexedSubscript:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_object_at_indexed_subscript(
            &self,
            attachment: Option<&MTLRenderPipelineColorAttachmentDescriptor>,
            attachment_index: usize,
        );
    );
}
