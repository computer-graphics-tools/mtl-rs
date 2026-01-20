use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
    runtime::{NSObject, ProtocolObject},
};
use objc2_foundation::{CopyingHelper, NSCopying, NSObjectProtocol};

use crate::MTLCounterSampleBuffer;

extern_class!(
    /// Sample buffer attachment descriptor for acceleration structure passes.
    ///
    /// Availability: macOS 13.0+, iOS 16.0+
    ///
    /// - `sample_buffer`: The sample buffer to store samples for the
    ///   acceleration structure pass defined samples. If non-`None`, the sample
    ///   indices will be used to store samples into the sample buffer. If no
    ///   sample buffer is provided, no samples will be taken. If any of the
    ///   sample indices are specified as `MTLCounterDontSample`, no sample will
    ///   be taken for that action.
    /// - `start_of_encoder_sample_index`: Sample index to use to store the
    ///   sample taken at the start of command encoder processing. Setting the
    ///   value to `MTLCounterDontSample` omits this sample. On devices where
    ///   `MTLCounterSamplingPointAtStageBoundary` is unsupported, this index is
    ///   invalid and must be set to `MTLCounterDontSample` or creation of an
    ///   acceleration structure pass will fail.
    /// - `end_of_encoder_sample_index`: Sample index to use to store the sample
    ///   taken at the end of command encoder processing. Setting the value to
    ///   `MTLCounterDontSample` omits this sample. On devices where
    ///   `MTLCounterSamplingPointAtStageBoundary` is unsupported, this index is
    ///   invalid and must be set to `MTLCounterDontSample` or creation of an
    ///   acceleration structure pass will fail.
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLAccelerationStructurePassSampleBufferAttachmentDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for MTLAccelerationStructurePassSampleBufferAttachmentDescriptor {}
);

unsafe impl CopyingHelper for MTLAccelerationStructurePassSampleBufferAttachmentDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLAccelerationStructurePassSampleBufferAttachmentDescriptor {}
);

impl MTLAccelerationStructurePassSampleBufferAttachmentDescriptor {
    extern_methods!(
        /// The sample buffer to store samples for the acceleration structure
        /// pass defined samples. If `Some`, the sample indices will be used to
        /// store samples into the sample buffer. If `None`, no samples will be
        /// taken. If any sample index is `MTLCounterDontSample`, no sample is
        /// taken for that action.
        #[unsafe(method(sampleBuffer))]
        #[unsafe(method_family = none)]
        pub unsafe fn sample_buffer(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn MTLCounterSampleBuffer>>>;

        /// Set the sample buffer used to store samples for encoder-defined
        /// samples. See getter for behavior when `None` or when indices are
        /// `MTLCounterDontSample`.
        #[unsafe(method(setSampleBuffer:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_sample_buffer(
            &self,
            sample_buffer: Option<&ProtocolObject<dyn MTLCounterSampleBuffer>>,
        );

        /// Sample index used to store the sample taken at the start of command
        /// encoder processing. Set to `MTLCounterDontSample` to omit.
        /// On devices where `MTLCounterSamplingPointAtStageBoundary` is
        /// unsupported, this must be `MTLCounterDontSample` or pass creation
        /// will fail.
        #[unsafe(method(startOfEncoderSampleIndex))]
        #[unsafe(method_family = none)]
        pub unsafe fn start_of_encoder_sample_index(&self) -> usize;

        #[unsafe(method(setStartOfEncoderSampleIndex:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_start_of_encoder_sample_index(&self, value: usize);

        /// Sample index used to store the sample taken at the end of command
        /// encoder processing. Set to `MTLCounterDontSample` to omit.
        /// On devices where `MTLCounterSamplingPointAtStageBoundary` is
        /// unsupported, this must be `MTLCounterDontSample` or pass creation
        /// will fail.
        #[unsafe(method(endOfEncoderSampleIndex))]
        #[unsafe(method_family = none)]
        pub unsafe fn end_of_encoder_sample_index(&self) -> usize;

        #[unsafe(method(setEndOfEncoderSampleIndex:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_end_of_encoder_sample_index(&self, value: usize);
    );
}

impl MTLAccelerationStructurePassSampleBufferAttachmentDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
