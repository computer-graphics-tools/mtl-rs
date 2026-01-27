use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
    runtime::NSObject,
};
use objc2_foundation::NSObjectProtocol;

use super::MTLComputePassSampleBufferAttachmentDescriptor;

extern_class!(
    /// Individual attachment state access for compute-pass sample buffers.
    ///
    /// Availability: macOS 11.0+, iOS 14.0+
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLComputePassSampleBufferAttachmentDescriptorArray;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLComputePassSampleBufferAttachmentDescriptorArray {}
);

impl MTLComputePassSampleBufferAttachmentDescriptorArray {
    extern_methods!(
        /// Returns the attachment descriptor at the given index.
        #[unsafe(method(objectAtIndexedSubscript:))]
        #[unsafe(method_family = none)]
        pub fn object_at_indexed_subscript(
            &self,
            attachment_index: usize,
        ) -> Retained<MTLComputePassSampleBufferAttachmentDescriptor>;

        /// Sets the attachment descriptor state at the given index.
        ///
        /// This always uses copy semantics. It is safe to set the attachment state at any legal index to `nil`, which resets that attachment descriptor state to default values.
        #[unsafe(method(setObject:atIndexedSubscript:))]
        #[unsafe(method_family = none)]
        pub fn set_object_at_indexed_subscript(
            &self,
            attachment: Option<&MTLComputePassSampleBufferAttachmentDescriptor>,
            attachment_index: usize,
        );
    );
}

impl MTLComputePassSampleBufferAttachmentDescriptorArray {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub fn new() -> Retained<Self>;
    );
}
