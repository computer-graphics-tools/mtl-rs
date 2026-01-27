use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
    runtime::NSObject,
};
use objc2_foundation::NSObjectProtocol;

use super::MTLTileRenderPipelineColorAttachmentDescriptor;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtltilerenderpipelinecolorattachmentdescriptorarray?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLTileRenderPipelineColorAttachmentDescriptorArray;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLTileRenderPipelineColorAttachmentDescriptorArray {}
);

impl MTLTileRenderPipelineColorAttachmentDescriptorArray {
    extern_methods!(
        #[unsafe(method(objectAtIndexedSubscript:))]
        #[unsafe(method_family = none)]
        pub fn object_at_indexed_subscript(
            &self,
            attachment_index: usize,
        ) -> Retained<MTLTileRenderPipelineColorAttachmentDescriptor>;

        #[unsafe(method(setObject:atIndexedSubscript:))]
        #[unsafe(method_family = none)]
        pub fn set_object_at_indexed_subscript(
            &self,
            attachment: &MTLTileRenderPipelineColorAttachmentDescriptor,
            attachment_index: usize,
        );
    );
}

/// Methods declared on superclass `NSObject`.
impl MTLTileRenderPipelineColorAttachmentDescriptorArray {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub fn new() -> Retained<Self>;
    );
}
