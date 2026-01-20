use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
    runtime::NSObject,
};
use objc2_foundation::{CopyingHelper, NSCopying, NSObjectProtocol};

extern_class!(
    /// Sample buffer attachment descriptor for resource state passes
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLResourceStatePassSampleBufferAttachmentDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for MTLResourceStatePassSampleBufferAttachmentDescriptor {}
);

unsafe impl CopyingHelper for MTLResourceStatePassSampleBufferAttachmentDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLResourceStatePassSampleBufferAttachmentDescriptor {}
);

impl MTLResourceStatePassSampleBufferAttachmentDescriptor {
    extern_methods!(
        /// The sample index to use to store the sample taken at the start of encoder processing.
        #[unsafe(method(startOfEncoderSampleIndex))]
        #[unsafe(method_family = none)]
        pub unsafe fn start_of_encoder_sample_index(&self) -> usize;

        /// Setter for [`start_of_encoder_sample_index`][Self::start_of_encoder_sample_index].
        #[unsafe(method(setStartOfEncoderSampleIndex:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_start_of_encoder_sample_index(&self, index: usize);

        /// The sample index to use to store the sample taken at the end of encoder processing.
        #[unsafe(method(endOfEncoderSampleIndex))]
        #[unsafe(method_family = none)]
        pub unsafe fn end_of_encoder_sample_index(&self) -> usize;

        /// Setter for [`end_of_encoder_sample_index`][Self::end_of_encoder_sample_index].
        #[unsafe(method(setEndOfEncoderSampleIndex:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_end_of_encoder_sample_index(&self, index: usize);
    );
}

/// Methods declared on superclass `NSObject`.
impl MTLResourceStatePassSampleBufferAttachmentDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
