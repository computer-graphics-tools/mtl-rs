use objc2::{Message, extern_protocol, msg_send, rc::Retained, runtime::ProtocolObject};
use objc2_foundation::{NSError, NSObjectProtocol, NSString};

use crate::{
    MTL4BinaryFunction, MTL4BinaryFunctionDescriptor, MTL4ComputePipelineDescriptor,
    MTL4PipelineDescriptor, MTL4PipelineStageDynamicLinkingDescriptor,
    MTL4RenderPipelineDynamicLinkingDescriptor, MTLComputePipelineState, MTLRenderPipelineState,
};

extern_protocol!(
    /// A read-only container that stores pipeline states from a shader compiler.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metal/mtl4archive?language=objc)
    pub unsafe trait MTL4Archive: NSObjectProtocol + Send + Sync {
        /// Creates a compute pipeline state from the archive with a descriptor.
        #[unsafe(method(newComputePipelineStateWithDescriptor:error:_))]
        #[unsafe(method_family = new)]
        fn new_compute_pipeline_state_with_descriptor(
            &self,
            descriptor: &MTL4ComputePipelineDescriptor,
        ) -> Result<Retained<ProtocolObject<dyn MTLComputePipelineState>>, Retained<NSError>>;

        /// Creates a compute pipeline state from the archive with a compute descriptor and a dynamic linking descriptor.
        #[unsafe(method(newComputePipelineStateWithDescriptor:dynamicLinkingDescriptor:error:_))]
        #[unsafe(method_family = new)]
        fn new_compute_pipeline_state_with_descriptor_dynamic_linking_descriptor(
            &self,
            descriptor: &MTL4ComputePipelineDescriptor,
            dynamic_linking_descriptor: &MTL4PipelineStageDynamicLinkingDescriptor,
        ) -> Result<Retained<ProtocolObject<dyn MTLComputePipelineState>>, Retained<NSError>>;

        /// Creates a render pipeline state from the archive with a descriptor.
        #[unsafe(method(newRenderPipelineStateWithDescriptor:error:_))]
        #[unsafe(method_family = new)]
        fn new_render_pipeline_state_with_descriptor(
            &self,
            descriptor: &MTL4PipelineDescriptor,
        ) -> Result<Retained<ProtocolObject<dyn MTLRenderPipelineState>>, Retained<NSError>>;

        /// Creates a render pipeline state from the archive with a render descriptor and a dynamic linking descriptor.
        #[unsafe(method(newRenderPipelineStateWithDescriptor:dynamicLinkingDescriptor:error:_))]
        #[unsafe(method_family = new)]
        fn new_render_pipeline_state_with_descriptor_dynamic_linking_descriptor(
            &self,
            descriptor: &MTL4PipelineDescriptor,
            dynamic_linking_descriptor: &MTL4RenderPipelineDynamicLinkingDescriptor,
        ) -> Result<Retained<ProtocolObject<dyn MTLRenderPipelineState>>, Retained<NSError>>;

        /// Method used to create a binary function, with a given descriptor, from the contents of the archive.
        #[unsafe(method(newBinaryFunctionWithDescriptor:error:_))]
        #[unsafe(method_family = new)]
        fn new_binary_function_with_descriptor(
            &self,
            descriptor: &MTL4BinaryFunctionDescriptor,
        ) -> Result<Retained<ProtocolObject<dyn MTL4BinaryFunction>>, Retained<NSError>>;
    }
);

pub trait MTL4ArchiveExt: MTL4Archive + Message {
    /// A label that you can associate with this archive.
    fn label(&self) -> Option<String> {
        let s: Option<Retained<NSString>> = unsafe { msg_send![self, label] };
        s.map(|v| v.to_string())
    }

    /// Setter for [`label`][Self::label].
    fn set_label(&self, label: Option<&str>) {
        unsafe {
            let _: () = msg_send![self, setLabel: label.map(NSString::from_str).as_deref()];
        }
    }
}

impl<T: MTL4Archive + Message> MTL4ArchiveExt for T {}
