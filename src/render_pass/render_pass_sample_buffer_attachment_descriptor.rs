use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::Retained,
    runtime::{NSObject, ProtocolObject},
};
use objc2_foundation::{CopyingHelper, NSCopying, NSObjectProtocol};

use crate::MTLCounterSampleBuffer;

extern_class!(
    /// Sample buffer attachment descriptor for a render pass.
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLRenderPassSampleBufferAttachmentDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for MTLRenderPassSampleBufferAttachmentDescriptor {}
);

unsafe impl CopyingHelper for MTLRenderPassSampleBufferAttachmentDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLRenderPassSampleBufferAttachmentDescriptor {}
);

impl MTLRenderPassSampleBufferAttachmentDescriptor {
    extern_methods!(
        /// The sample buffer to store samples for the render-pass defined samples.
        #[unsafe(method(sampleBuffer))]
        #[unsafe(method_family = none)]
        pub fn sample_buffer(&self) -> Option<Retained<ProtocolObject<dyn MTLCounterSampleBuffer>>>;

        /// Setter for [`sample_buffer`][Self::sample_buffer].
        #[unsafe(method(setSampleBuffer:))]
        #[unsafe(method_family = none)]
        pub fn set_sample_buffer(
            &self,
            sample_buffer: Option<&ProtocolObject<dyn MTLCounterSampleBuffer>>,
        );

        /// The sample index used at the start of vertex processing.
        #[unsafe(method(startOfVertexSampleIndex))]
        #[unsafe(method_family = none)]
        pub fn start_of_vertex_sample_index(&self) -> usize;

        #[unsafe(method(setStartOfVertexSampleIndex:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_start_of_vertex_sample_index(&self, index: usize);

        /// The sample index used at the end of vertex processing.
        #[unsafe(method(endOfVertexSampleIndex))]
        #[unsafe(method_family = none)]
        pub fn end_of_vertex_sample_index(&self) -> usize;

        #[unsafe(method(setEndOfVertexSampleIndex:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_end_of_vertex_sample_index(&self, index: usize);

        /// The sample index used at the start of fragment processing.
        #[unsafe(method(startOfFragmentSampleIndex))]
        #[unsafe(method_family = none)]
        pub fn start_of_fragment_sample_index(&self) -> usize;

        #[unsafe(method(setStartOfFragmentSampleIndex:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_start_of_fragment_sample_index(&self, index: usize);

        /// The sample index used at the end of fragment processing.
        #[unsafe(method(endOfFragmentSampleIndex))]
        #[unsafe(method_family = none)]
        pub fn end_of_fragment_sample_index(&self) -> usize;

        #[unsafe(method(setEndOfFragmentSampleIndex:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_end_of_fragment_sample_index(&self, index: usize);
    );
}
