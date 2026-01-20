use objc2::{
    extern_class, extern_conformance, extern_methods,
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
    extern_methods!(
        /// Return the attachment descriptor at the specified index.
        ///
        /// This provides individual attachment state access.
        #[unsafe(method(objectAtIndexedSubscript:))]
        #[unsafe(method_family = none)]
        pub unsafe fn object_at_indexed_subscript(
            &self,
            attachment_index: usize,
        ) -> Retained<MTLAccelerationStructurePassSampleBufferAttachmentDescriptor>;

        /// Set the attachment descriptor at the specified index.
        ///
        /// This method uses copy semantics. It is safe to set the attachment
        /// state at any legal index to `None`, which resets that attachment
        /// descriptor's state to default values.
        #[unsafe(method(setObject:atIndexedSubscript:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_object_at_indexed_subscript(
            &self,
            attachment: Option<&MTLAccelerationStructurePassSampleBufferAttachmentDescriptor>,
            attachment_index: usize,
        );
    );
}

impl MTLAccelerationStructurePassSampleBufferAttachmentDescriptorArray {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
