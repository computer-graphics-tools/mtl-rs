use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
    runtime::{NSObject, ProtocolObject},
};
use objc2_foundation::{CopyingHelper, NSCopying, NSObjectProtocol};

use crate::counters::MTLCounterSampleBuffer;

extern_class!(
    /// Descriptor for attaching a counter sample buffer to a blit pass.
    ///
    /// Availability: macOS 11.0+, iOS 14.0+
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLBlitPassSampleBufferAttachmentDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for MTLBlitPassSampleBufferAttachmentDescriptor {}
);

unsafe impl CopyingHelper for MTLBlitPassSampleBufferAttachmentDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLBlitPassSampleBufferAttachmentDescriptor {}
);

impl MTLBlitPassSampleBufferAttachmentDescriptor {
    extern_methods!(
        /// The sample buffer to store samples for the blit-pass defined samples.
        ///
        /// If `sampleBuffer` is non-nil, the sample indices will be used to store samples into the sample buffer.
        /// If no sample buffer is provided, no samples will be taken.
        /// If any of the sample indices are specified as `MTLCounterDontSample`, no sample will be taken for that action.
        #[unsafe(method(sampleBuffer))]
        #[unsafe(method_family = none)]
        pub fn sample_buffer(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn MTLCounterSampleBuffer>>>;

        /// Setter for `sample_buffer`.
        #[unsafe(method(setSampleBuffer:))]
        #[unsafe(method_family = none)]
        pub fn set_sample_buffer(
            &self,
            sample_buffer: Option<&ProtocolObject<dyn MTLCounterSampleBuffer>>,
        );

        /// The sample index to use to store the sample taken at the start of command encoder processing.
        ///
        /// Setting the value to `MTLCounterDontSample` will cause this sample to be omitted.
        ///
        /// On devices where `MTLCounterSamplingPointAtStageBoundary` is unsupported, this sample index is invalid and must be set to `MTLCounterDontSample` or creation of a blit pass will fail.
        #[unsafe(method(startOfEncoderSampleIndex))]
        #[unsafe(method_family = none)]
        pub fn start_of_encoder_sample_index(&self) -> usize;

        #[unsafe(method(setStartOfEncoderSampleIndex:))]
        #[unsafe(method_family = none)]
        pub fn set_start_of_encoder_sample_index(&self, value: usize);

        /// The sample index to use to store the sample taken at the end of command encoder processing.
        ///
        /// Setting the value to `MTLCounterDontSample` will cause this sample to be omitted.
        ///
        /// On devices where `MTLCounterSamplingPointAtStageBoundary` is unsupported, this sample index is invalid and must be set to `MTLCounterDontSample` or creation of a blit pass will fail.
        #[unsafe(method(endOfEncoderSampleIndex))]
        #[unsafe(method_family = none)]
        pub fn end_of_encoder_sample_index(&self) -> usize;

        #[unsafe(method(setEndOfEncoderSampleIndex:))]
        #[unsafe(method_family = none)]
        pub fn set_end_of_encoder_sample_index(&self, value: usize);
    );
}

impl MTLBlitPassSampleBufferAttachmentDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub fn new() -> Retained<Self>;
    );
}
