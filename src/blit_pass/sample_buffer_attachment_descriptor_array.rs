use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
    runtime::NSObject,
};
use objc2_foundation::NSObjectProtocol;

use super::MTLBlitPassSampleBufferAttachmentDescriptor;

extern_class!(
    /// Individual attachment state access for blit-pass sample buffers.
    ///
    /// Availability: macOS 11.0+, iOS 14.0+
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLBlitPassSampleBufferAttachmentDescriptorArray;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLBlitPassSampleBufferAttachmentDescriptorArray {}
);

impl MTLBlitPassSampleBufferAttachmentDescriptorArray {
    extern_methods!(
        /// Returns the attachment descriptor at the given index.
        #[unsafe(method(objectAtIndexedSubscript:))]
        #[unsafe(method_family = none)]
        pub fn object_at_indexed_subscript(
            &self,
            attachment_index: usize,
        ) -> Retained<MTLBlitPassSampleBufferAttachmentDescriptor>;

        /// Sets the attachment descriptor state at the given index.
        ///
        /// This always uses copy semantics. It is safe to set the attachment state at any legal index to `nil`, which resets that attachment descriptor state to default values.
        #[unsafe(method(setObject:atIndexedSubscript:))]
        #[unsafe(method_family = none)]
        pub fn set_object_at_indexed_subscript(
            &self,
            attachment: Option<&MTLBlitPassSampleBufferAttachmentDescriptor>,
            attachment_index: usize,
        );
    );
}

impl MTLBlitPassSampleBufferAttachmentDescriptorArray {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub fn new() -> Retained<Self>;
    );
}
