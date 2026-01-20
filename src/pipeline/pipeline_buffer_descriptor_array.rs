use objc2::{extern_class, extern_conformance, extern_methods, rc::Retained, runtime::NSObject};
use objc2_foundation::NSObjectProtocol;

use super::MTLPipelineBufferDescriptor;

extern_class!(
    /// Apple's docs: `https://developer.apple.com/documentation/metal/mtlpipelinebufferdescriptorarray?language=objc`
    ///
    /// Availability: macOS 10.13+, iOS 11.0+
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLPipelineBufferDescriptorArray;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLPipelineBufferDescriptorArray {}
);

impl MTLPipelineBufferDescriptorArray {
    extern_methods!(
        #[unsafe(method(objectAtIndexedSubscript:))]
        #[unsafe(method_family = none)]
        pub unsafe fn object_at_indexed_subscript(
            &self,
            buffer_index: usize,
        ) -> Retained<MTLPipelineBufferDescriptor>;

        #[unsafe(method(setObject:atIndexedSubscript:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_object_at_indexed_subscript(
            &self,
            buffer: Option<&MTLPipelineBufferDescriptor>,
            buffer_index: usize,
        );
    );
}
