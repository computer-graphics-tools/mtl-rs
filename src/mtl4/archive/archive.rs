use objc2::{Message, extern_protocol, msg_send, rc::Retained, runtime::ProtocolObject};
use objc2_foundation::{NSObjectProtocol, NSString};

use crate::{
    MTL4BinaryFunction, MTL4BinaryFunctionDescriptor, MTL4ComputePipelineDescriptor, MTL4PipelineDescriptor,
    MTL4PipelineStageDynamicLinkingDescriptor, MTL4RenderPipelineDynamicLinkingDescriptor, MTLComputePipelineState,
    MTLRenderPipelineState,
};

extern_protocol!(
    /// A read-only container that stores pipeline states from a shader compiler.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metal/mtl4archive?language=objc)
    ///
    /// # Thread safety
    ///
    /// [Apple's declaration](https://developer.apple.com/documentation/metal/mtl4archive):
    ///
    /// > `protocol MTL4Archive : NSObjectProtocol, Sendable`
    ///
    /// The `Send` and `Sync` bounds rely on this guarantee.
    pub unsafe trait MTL4Archive: NSObjectProtocol + Send + Sync {}
);

pub trait MTL4ArchiveExt: MTL4Archive + Message {
    /// Creates a compute pipeline state from the archive with a descriptor.
    fn new_compute_pipeline_state_with_descriptor(
        &self,
        descriptor: &MTL4ComputePipelineDescriptor,
    ) -> Result<Retained<ProtocolObject<dyn MTLComputePipelineState>>, crate::MetalError>
    where
        Self: Sized,
    {
        unsafe { msg_send![self, newComputePipelineStateWithDescriptor: descriptor, error: _] }
            .map_err(crate::MetalError::from_nserror)
    }

    /// Creates a compute pipeline state with dynamic-linking configuration from the archive.
    fn new_compute_pipeline_state_with_descriptor_dynamic_linking_descriptor(
        &self,
        descriptor: &MTL4ComputePipelineDescriptor,
        dynamic_linking_descriptor: &MTL4PipelineStageDynamicLinkingDescriptor,
    ) -> Result<Retained<ProtocolObject<dyn MTLComputePipelineState>>, crate::MetalError>
    where
        Self: Sized,
    {
        unsafe {
            msg_send![
                self,
                newComputePipelineStateWithDescriptor: descriptor,
                dynamicLinkingDescriptor: dynamic_linking_descriptor,
                error: _
            ]
        }
        .map_err(crate::MetalError::from_nserror)
    }

    /// Creates a render pipeline state from the archive with a descriptor.
    fn new_render_pipeline_state_with_descriptor(
        &self,
        descriptor: &MTL4PipelineDescriptor,
    ) -> Result<Retained<ProtocolObject<dyn MTLRenderPipelineState>>, crate::MetalError>
    where
        Self: Sized,
    {
        unsafe { msg_send![self, newRenderPipelineStateWithDescriptor: descriptor, error: _] }
            .map_err(crate::MetalError::from_nserror)
    }

    /// Creates a render pipeline state with dynamic-linking configuration from the archive.
    fn new_render_pipeline_state_with_descriptor_dynamic_linking_descriptor(
        &self,
        descriptor: &MTL4PipelineDescriptor,
        dynamic_linking_descriptor: &MTL4RenderPipelineDynamicLinkingDescriptor,
    ) -> Result<Retained<ProtocolObject<dyn MTLRenderPipelineState>>, crate::MetalError>
    where
        Self: Sized,
    {
        unsafe {
            msg_send![
                self,
                newRenderPipelineStateWithDescriptor: descriptor,
                dynamicLinkingDescriptor: dynamic_linking_descriptor,
                error: _
            ]
        }
        .map_err(crate::MetalError::from_nserror)
    }

    /// Creates a binary function from the contents of the archive.
    fn new_binary_function_with_descriptor(
        &self,
        descriptor: &MTL4BinaryFunctionDescriptor,
    ) -> Result<Retained<ProtocolObject<dyn MTL4BinaryFunction>>, crate::MetalError>
    where
        Self: Sized,
    {
        unsafe { msg_send![self, newBinaryFunctionWithDescriptor: descriptor, error: _] }
            .map_err(crate::MetalError::from_nserror)
    }

    /// A label that you can associate with this archive.
    fn label(&self) -> Option<String> {
        let s: Option<Retained<NSString>> = unsafe { msg_send![self, label] };
        s.map(|v| v.to_string())
    }

    /// Setter for [`label`][Self::label].
    fn set_label(
        &self,
        label: Option<&str>,
    ) {
        unsafe {
            let _: () = msg_send![self, setLabel: label.map(NSString::from_str).as_deref()];
        }
    }
}

impl<T: MTL4Archive + Message> MTL4ArchiveExt for T {}
