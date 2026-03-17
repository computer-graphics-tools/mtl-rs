use core::{ffi::c_void, ops::Range, ptr::NonNull};

use objc2::{Message, extern_protocol, msg_send, rc::Retained, runtime::ProtocolObject};
use objc2_foundation::{NSObjectProtocol, NSRange, NSString};

use crate::util::option_ref_ptr_cast_const;
use crate::{
    MTLAccelerationStructure, MTLBuffer, MTLComputePipelineState, MTLDepthStencilState, MTLDevice,
    MTLIndirectCommandBuffer, MTLIntersectionFunctionTable, MTLRenderPipelineState, MTLSamplerState, MTLTexture,
    MTLVisibleFunctionTable,
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
        fn device(&self) -> Retained<ProtocolObject<dyn MTLDevice>>;

        /// A string to help identify this object.
        #[unsafe(method(label))]
        #[unsafe(method_family = none)]
        fn label(&self) -> Option<Retained<NSString>>;

        /// Setter for `label`.
        #[unsafe(method(setLabel:))]
        #[unsafe(method_family = none)]
        fn set_label(
            &self,
            label: Option<&NSString>,
        );

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
        fn set_argument_buffer(
            &self,
            argument_buffer: Option<&ProtocolObject<dyn MTLBuffer>>,
            offset: usize,
        );

        /// Sets the destination buffer, starting offset and specific array element
        /// that arguments will be encoded into.
        #[unsafe(method(setArgumentBuffer:startOffset:arrayElement:))]
        #[unsafe(method_family = none)]
        fn set_argument_buffer_with_array_element(
            &self,
            argument_buffer: Option<&ProtocolObject<dyn MTLBuffer>>,
            start_offset: usize,
            array_element: usize,
        );

        /// Set a buffer at the given bind point index.
        #[unsafe(method(setBuffer:offset:atIndex:))]
        #[unsafe(method_family = none)]
        fn set_buffer(
            &self,
            buffer: Option<&ProtocolObject<dyn MTLBuffer>>,
            offset: usize,
            index: usize,
        );

        /// Set a texture at the given bind point index.
        #[unsafe(method(setTexture:atIndex:))]
        #[unsafe(method_family = none)]
        fn set_texture(
            &self,
            texture: Option<&ProtocolObject<dyn MTLTexture>>,
            index: usize,
        );

        /// Set a sampler at the given bind point index.
        #[unsafe(method(setSamplerState:atIndex:))]
        #[unsafe(method_family = none)]
        fn set_sampler_state(
            &self,
            sampler: Option<&ProtocolObject<dyn MTLSamplerState>>,
            index: usize,
        );

        /// Returns a pointer to the constant data at the given bind point index.
        ///
        /// Safety: The returned pointer is only valid as long as the encoder and
        /// backing storage are alive.
        #[unsafe(method(constantDataAtIndex:))]
        #[unsafe(method_family = none)]
        fn constant_data_at_index(
            &self,
            index: usize,
        ) -> NonNull<c_void>;

        /// Sets a render pipeline state at a given bind point index.
        ///
        /// Availability: macOS 10.14+, iOS 13.0+
        #[unsafe(method(setRenderPipelineState:atIndex:))]
        #[unsafe(method_family = none)]
        fn set_render_pipeline_state(
            &self,
            pipeline: Option<&ProtocolObject<dyn MTLRenderPipelineState>>,
            index: usize,
        );

        /// Sets a compute pipeline state at a given bind point index.
        ///
        /// Availability: macOS 11.0+, iOS 13.0+
        #[unsafe(method(setComputePipelineState:atIndex:))]
        #[unsafe(method_family = none)]
        fn set_compute_pipeline_state(
            &self,
            pipeline: Option<&ProtocolObject<dyn MTLComputePipelineState>>,
            index: usize,
        );

        /// Sets an indirect command buffer at a given bind point index.
        ///
        /// Availability: macOS 10.14+, iOS 12.0+
        #[unsafe(method(setIndirectCommandBuffer:atIndex:))]
        #[unsafe(method_family = none)]
        fn set_indirect_command_buffer(
            &self,
            indirect_command_buffer: Option<&ProtocolObject<dyn MTLIndirectCommandBuffer>>,
            index: usize,
        );

        /// Sets an acceleration structure at a given bind point index.
        ///
        /// Availability: macOS 11.0+, iOS 14.0+, tvOS 16.0+
        #[unsafe(method(setAccelerationStructure:atIndex:))]
        #[unsafe(method_family = none)]
        fn set_acceleration_structure(
            &self,
            acceleration_structure: Option<&ProtocolObject<dyn MTLAccelerationStructure>>,
            index: usize,
        );

        /// Returns a new argument encoder for an argument buffer bound at `index`.
        /// Returns `None` if the resource at `index` is not an argument buffer.
        #[unsafe(method(newArgumentEncoderForBufferAtIndex:))]
        #[unsafe(method_family = new)]
        fn new_argument_encoder_for_buffer_at_index(
            &self,
            index: usize,
        ) -> Option<Retained<ProtocolObject<dyn super::MTLArgumentEncoder>>>;

        /// Set a visible function table at the given buffer index.
        ///
        /// Availability: macOS 11.0+, iOS 14.0+, tvOS 16.0+
        #[unsafe(method(setVisibleFunctionTable:atIndex:))]
        #[unsafe(method_family = none)]
        fn set_visible_function_table(
            &self,
            visible_function_table: Option<&ProtocolObject<dyn MTLVisibleFunctionTable>>,
            index: usize,
        );

        /// Set an intersection function table at the given buffer index.
        ///
        /// Availability: macOS 11.0+, iOS 14.0+, tvOS 16.0+
        #[unsafe(method(setIntersectionFunctionTable:atIndex:))]
        #[unsafe(method_family = none)]
        fn set_intersection_function_table(
            &self,
            intersection_function_table: Option<&ProtocolObject<dyn MTLIntersectionFunctionTable>>,
            index: usize,
        );

        /// Sets a depth stencil state at a given bind point index.
        ///
        /// Availability: macOS 26.0+, iOS 26.0+
        #[unsafe(method(setDepthStencilState:atIndex:))]
        #[unsafe(method_family = none)]
        fn set_depth_stencil_state(
            &self,
            depth_stencil_state: Option<&ProtocolObject<dyn MTLDepthStencilState>>,
            index: usize,
        );
    }
);

pub trait MTLArgumentEncoderExt: MTLArgumentEncoder + Message {
    /// Set an array of buffers at the given bind point index range.
    fn set_buffers(
        &self,
        buffers: &[Option<&ProtocolObject<dyn MTLBuffer>>],
        offsets: &[usize],
        range: Range<usize>,
    ) where
        Self: Sized,
    {
        assert_eq!(buffers.len(), offsets.len());
        let ptr = option_ref_ptr_cast_const(buffers.as_ptr());
        unsafe { msg_send![self, setBuffers: ptr, offsets: offsets.as_ptr(), withRange: NSRange::from(range)] }
    }

    /// Set an array of textures at the given bind point index range.
    fn set_textures(
        &self,
        textures: &[Option<&ProtocolObject<dyn MTLTexture>>],
        range: Range<usize>,
    ) where
        Self: Sized,
    {
        let ptr = option_ref_ptr_cast_const(textures.as_ptr());
        unsafe { msg_send![self, setTextures: ptr, withRange: NSRange::from(range)] }
    }

    /// Set an array of samplers at the given bind point index range.
    fn set_sampler_states(
        &self,
        samplers: &[Option<&ProtocolObject<dyn MTLSamplerState>>],
        range: Range<usize>,
    ) where
        Self: Sized,
    {
        let ptr = option_ref_ptr_cast_const(samplers.as_ptr());
        unsafe { msg_send![self, setSamplerStates: ptr, withRange: NSRange::from(range)] }
    }

    /// Set an array of render pipeline states at a given bind point index range.
    ///
    /// Availability: macOS 10.14+, iOS 13.0+
    fn set_render_pipeline_states(
        &self,
        pipelines: &[Option<&ProtocolObject<dyn MTLRenderPipelineState>>],
        range: Range<usize>,
    ) where
        Self: Sized,
    {
        let ptr = option_ref_ptr_cast_const(pipelines.as_ptr());
        unsafe { msg_send![self, setRenderPipelineStates: ptr, withRange: NSRange::from(range)] }
    }

    /// Set an array of compute pipeline states at a given bind point index range.
    ///
    /// Availability: macOS 11.0+, iOS 13.0+
    fn set_compute_pipeline_states(
        &self,
        pipelines: &[Option<&ProtocolObject<dyn MTLComputePipelineState>>],
        range: Range<usize>,
    ) where
        Self: Sized,
    {
        let ptr = option_ref_ptr_cast_const(pipelines.as_ptr());
        unsafe { msg_send![self, setComputePipelineStates: ptr, withRange: NSRange::from(range)] }
    }

    /// Set an array of indirect command buffers at the given bind point index range.
    ///
    /// Availability: macOS 10.14+, iOS 12.0+
    fn set_indirect_command_buffers(
        &self,
        buffers: &[Option<&ProtocolObject<dyn MTLIndirectCommandBuffer>>],
        range: Range<usize>,
    ) where
        Self: Sized,
    {
        let ptr = option_ref_ptr_cast_const(buffers.as_ptr());
        unsafe { msg_send![self, setIndirectCommandBuffers: ptr, withRange: NSRange::from(range)] }
    }

    /// Set visible function tables at the given buffer index range.
    ///
    /// Availability: macOS 11.0+, iOS 14.0+, tvOS 16.0+
    fn set_visible_function_tables(
        &self,
        tables: &[Option<&ProtocolObject<dyn MTLVisibleFunctionTable>>],
        range: Range<usize>,
    ) where
        Self: Sized,
    {
        let ptr = option_ref_ptr_cast_const(tables.as_ptr());
        unsafe { msg_send![self, setVisibleFunctionTables: ptr, withRange: NSRange::from(range)] }
    }

    /// Set intersection function tables at the given buffer index range.
    ///
    /// Availability: macOS 11.0+, iOS 14.0+, tvOS 16.0+
    fn set_intersection_function_tables(
        &self,
        tables: &[Option<&ProtocolObject<dyn MTLIntersectionFunctionTable>>],
        range: Range<usize>,
    ) where
        Self: Sized,
    {
        let ptr = option_ref_ptr_cast_const(tables.as_ptr());
        unsafe { msg_send![self, setIntersectionFunctionTables: ptr, withRange: NSRange::from(range)] }
    }

    /// Sets an array of depth stencil states at a given buffer index range.
    ///
    /// Availability: macOS 26.0+, iOS 26.0+
    fn set_depth_stencil_states(
        &self,
        states: &[Option<&ProtocolObject<dyn MTLDepthStencilState>>],
        range: Range<usize>,
    ) where
        Self: Sized,
    {
        let ptr = option_ref_ptr_cast_const(states.as_ptr());
        unsafe { msg_send![self, setDepthStencilStates: ptr, withRange: NSRange::from(range)] }
    }
}

impl<T: MTLArgumentEncoder + Message> MTLArgumentEncoderExt for T {}
