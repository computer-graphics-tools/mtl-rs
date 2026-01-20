use core::{ffi::c_void, ptr::NonNull};

use objc2::{extern_protocol, rc::Retained, runtime::ProtocolObject};
use objc2_foundation::{NSObjectProtocol, NSRange, NSString};

use crate::{
    MTLAccelerationStructure, MTLBuffer, MTLComputePipelineState, MTLDepthStencilState, MTLDevice,
    MTLIndirectCommandBuffer, MTLRenderPipelineState, MTLSamplerState, MTLTexture,
};

/// When calling functions with an `attributeStrides:` parameter on a render
/// or compute command encoder, this value must be provided for the binding
/// points that are either not part of the set of MTLBufferLayoutDescriptor,
/// or whose stride values in the descriptor is not set to
/// `MTLBufferLayoutStrideDynamic`
pub const MTL_ATTRIBUTE_STRIDE_STATIC: usize = usize::MAX;

extern_protocol!(
    /// Encodes buffer, texture, sampler, pipeline, indirect command buffer,
    /// acceleration structure, and constant data into a buffer.
    ///
    /// Availability: macOS 10.13+, iOS 11.0+
    pub unsafe trait MTLArgumentEncoder: NSObjectProtocol {
        /// The device this argument encoder was created against.
        #[unsafe(method(device))]
        #[unsafe(method_family = none)]
        unsafe fn device(&self) -> Retained<ProtocolObject<dyn MTLDevice>>;

        /// A string to help identify this object.
        #[unsafe(method(label))]
        #[unsafe(method_family = none)]
        fn label(&self) -> Option<Retained<NSString>>;

        /// Setter for `label`.
        #[unsafe(method(setLabel:))]
        #[unsafe(method_family = none)]
        fn set_label(&self, label: Option<&NSString>);

        /// The number of bytes required to store the encoded resource bindings.
        #[unsafe(method(encodedLength))]
        #[unsafe(method_family = none)]
        fn encoded_length(&self) -> usize;

        /// The alignment in bytes required to store the encoded resource bindings.
        #[unsafe(method(alignment))]
        #[unsafe(method_family = none)]
        fn alignment(&self) -> usize;

        /// Sets the destination buffer and offset at which the arguments will be encoded.
        #[unsafe(method(setArgumentBuffer:offset:))]
        #[unsafe(method_family = none)]
        unsafe fn set_argument_buffer(
            &self,
            argument_buffer: Option<&ProtocolObject<dyn MTLBuffer>>,
            offset: usize,
        );

        /// Sets the destination buffer, starting offset and specific array element
        /// that arguments will be encoded into.
        #[unsafe(method(setArgumentBuffer:startOffset:arrayElement:))]
        #[unsafe(method_family = none)]
        unsafe fn set_argument_buffer_with_array_element(
            &self,
            argument_buffer: Option<&ProtocolObject<dyn MTLBuffer>>,
            start_offset: usize,
            array_element: usize,
        );

        /// Set a buffer at the given bind point index.
        #[unsafe(method(setBuffer:offset:atIndex:))]
        #[unsafe(method_family = none)]
        unsafe fn set_buffer(
            &self,
            buffer: Option<&ProtocolObject<dyn MTLBuffer>>,
            offset: usize,
            index: usize,
        );

        /// Set an array of buffers at the given bind point index range.
        ///
        /// Safety: `buffers` and `offsets` must be valid pointers.
        #[unsafe(method(setBuffers:offsets:withRange:))]
        #[unsafe(method_family = none)]
        unsafe fn set_buffers(
            &self,
            buffers: NonNull<*const ProtocolObject<dyn MTLBuffer>>,
            offsets: NonNull<usize>,
            range: NSRange,
        );

        /// Set a texture at the given bind point index.
        #[unsafe(method(setTexture:atIndex:))]
        #[unsafe(method_family = none)]
        unsafe fn set_texture(
            &self,
            texture: Option<&ProtocolObject<dyn MTLTexture>>,
            index: usize,
        );

        /// Set an array of textures at the given bind point index range.
        ///
        /// Safety: `textures` must be a valid pointer.
        #[unsafe(method(setTextures:withRange:))]
        #[unsafe(method_family = none)]
        unsafe fn set_textures(
            &self,
            textures: NonNull<*const ProtocolObject<dyn MTLTexture>>,
            range: NSRange,
        );

        /// Set a sampler at the given bind point index.
        #[unsafe(method(setSamplerState:atIndex:))]
        #[unsafe(method_family = none)]
        unsafe fn set_sampler_state(
            &self,
            sampler: Option<&ProtocolObject<dyn MTLSamplerState>>,
            index: usize,
        );

        /// Set an array of samplers at the given bind point index range.
        ///
        /// Safety: `samplers` must be a valid pointer.
        #[unsafe(method(setSamplerStates:withRange:))]
        #[unsafe(method_family = none)]
        unsafe fn set_sampler_states(
            &self,
            samplers: NonNull<*const ProtocolObject<dyn MTLSamplerState>>,
            range: NSRange,
        );

        /// Returns a pointer to the constant data at the given bind point index.
        ///
        /// Safety: The returned pointer is only valid as long as the encoder and
        /// backing storage are alive.
        #[unsafe(method(constantDataAtIndex:))]
        #[unsafe(method_family = none)]
        unsafe fn constant_data_at_index(&self, index: usize) -> NonNull<c_void>;

        /// Sets a render pipeline state at a given bind point index.
        ///
        /// Availability: macOS 10.14+, iOS 13.0+
        #[unsafe(method(setRenderPipelineState:atIndex:))]
        #[unsafe(method_family = none)]
        unsafe fn set_render_pipeline_state(
            &self,
            pipeline: Option<&ProtocolObject<dyn MTLRenderPipelineState>>,
            index: usize,
        );

        /// Set an array of render pipeline states at a given bind point index range.
        ///
        /// Safety: `pipelines` must be a valid pointer.
        ///
        /// Availability: macOS 10.14+, iOS 13.0+
        #[unsafe(method(setRenderPipelineStates:withRange:))]
        #[unsafe(method_family = none)]
        unsafe fn set_render_pipeline_states(
            &self,
            pipelines: NonNull<*const ProtocolObject<dyn MTLRenderPipelineState>>,
            range: NSRange,
        );

        /// Sets a compute pipeline state at a given bind point index.
        ///
        /// Availability: macOS 11.0+, iOS 13.0+
        #[unsafe(method(setComputePipelineState:atIndex:))]
        #[unsafe(method_family = none)]
        unsafe fn set_compute_pipeline_state(
            &self,
            pipeline: Option<&ProtocolObject<dyn MTLComputePipelineState>>,
            index: usize,
        );

        /// Set an array of compute pipeline states at a given bind point index range.
        ///
        /// Safety: `pipelines` must be a valid pointer.
        ///
        /// Availability: macOS 11.0+, iOS 13.0+
        #[unsafe(method(setComputePipelineStates:withRange:))]
        #[unsafe(method_family = none)]
        unsafe fn set_compute_pipeline_states(
            &self,
            pipelines: NonNull<*const ProtocolObject<dyn MTLComputePipelineState>>,
            range: NSRange,
        );

        /// Sets an indirect command buffer at a given bind point index.
        ///
        /// Availability: macOS 10.14+, iOS 12.0+
        #[unsafe(method(setIndirectCommandBuffer:atIndex:))]
        #[unsafe(method_family = none)]
        unsafe fn set_indirect_command_buffer(
            &self,
            indirect_command_buffer: Option<&ProtocolObject<dyn MTLIndirectCommandBuffer>>,
            index: usize,
        );

        /// Set an array of indirect command buffers at the given bind point index range.
        ///
        /// Safety: `buffers` must be a valid pointer.
        ///
        /// Availability: macOS 10.14+, iOS 12.0+
        #[unsafe(method(setIndirectCommandBuffers:withRange:))]
        #[unsafe(method_family = none)]
        unsafe fn set_indirect_command_buffers(
            &self,
            buffers: NonNull<*const ProtocolObject<dyn MTLIndirectCommandBuffer>>,
            range: NSRange,
        );

        /// Sets an acceleration structure at a given bind point index.
        ///
        /// Availability: macOS 11.0+, iOS 14.0+, tvOS 16.0+
        #[unsafe(method(setAccelerationStructure:atIndex:))]
        #[unsafe(method_family = none)]
        unsafe fn set_acceleration_structure(
            &self,
            acceleration_structure: Option<&ProtocolObject<dyn MTLAccelerationStructure>>,
            index: usize,
        );

        /// Returns a new argument encoder for an argument buffer bound at `index`.
        /// Returns `None` if the resource at `index` is not an argument buffer.
        #[unsafe(method(newArgumentEncoderForBufferAtIndex:))]
        #[unsafe(method_family = new)]
        unsafe fn new_argument_encoder_for_buffer_at_index(
            &self,
            index: usize,
        ) -> Option<Retained<ProtocolObject<dyn super::MTLArgumentEncoder>>>;

        /// Set a visible function table at the given buffer index.
        ///
        /// Availability: macOS 11.0+, iOS 14.0+, tvOS 16.0+
        #[unsafe(method(setVisibleFunctionTable:atIndex:))]
        #[unsafe(method_family = none)]
        unsafe fn set_visible_function_table(
            &self,
            visible_function_table: Option<&ProtocolObject<dyn crate::MTLVisibleFunctionTable>>,
            index: usize,
        );

        /// Set visible function tables at the given buffer index range.
        ///
        /// Safety: `visible_function_tables` must be a valid pointer.
        ///
        /// Availability: macOS 11.0+, iOS 14.0+, tvOS 16.0+
        #[unsafe(method(setVisibleFunctionTables:withRange:))]
        #[unsafe(method_family = none)]
        unsafe fn set_visible_function_tables(
            &self,
            visible_function_tables: NonNull<
                *const ProtocolObject<dyn crate::MTLVisibleFunctionTable>,
            >,
            range: NSRange,
        );

        /// Set an intersection function table at the given buffer index.
        ///
        /// Availability: macOS 11.0+, iOS 14.0+, tvOS 16.0+
        #[unsafe(method(setIntersectionFunctionTable:atIndex:))]
        #[unsafe(method_family = none)]
        unsafe fn set_intersection_function_table(
            &self,
            intersection_function_table: Option<
                &ProtocolObject<dyn crate::MTLIntersectionFunctionTable>,
            >,
            index: usize,
        );

        /// Set intersection function tables at the given buffer index range.
        ///
        /// Safety: `intersection_function_tables` must be a valid pointer.
        ///
        /// Availability: macOS 11.0+, iOS 14.0+, tvOS 16.0+
        #[unsafe(method(setIntersectionFunctionTables:withRange:))]
        #[unsafe(method_family = none)]
        unsafe fn set_intersection_function_tables(
            &self,
            intersection_function_tables: NonNull<
                *const ProtocolObject<dyn crate::MTLIntersectionFunctionTable>,
            >,
            range: NSRange,
        );

        /// Sets a depth stencil state at a given bind point index.
        ///
        /// Availability: macOS 26.0+, iOS 26.0+
        #[unsafe(method(setDepthStencilState:atIndex:))]
        #[unsafe(method_family = none)]
        unsafe fn set_depth_stencil_state(
            &self,
            depth_stencil_state: Option<&ProtocolObject<dyn MTLDepthStencilState>>,
            index: usize,
        );

        /// Sets an array of depth stencil states at a given buffer index range.
        ///
        /// Safety: `depth_stencil_states` must be a valid pointer.
        ///
        /// Availability: macOS 26.0+, iOS 26.0+
        #[unsafe(method(setDepthStencilStates:withRange:))]
        #[unsafe(method_family = none)]
        unsafe fn set_depth_stencil_states(
            &self,
            depth_stencil_states: NonNull<*const ProtocolObject<dyn MTLDepthStencilState>>,
            range: NSRange,
        );
    }
);
