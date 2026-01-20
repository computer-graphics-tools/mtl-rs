use core::ffi::c_float;

use objc2::{extern_protocol, runtime::ProtocolObject};
use objc2_foundation::NSObjectProtocol;

use crate::{
    MTLBuffer, MTLCullMode, MTLDepthClipMode, MTLDepthStencilState, MTLIndexType, MTLPrimitiveType,
    MTLRenderPipelineState, MTLSize, MTLTriangleFillMode, MTLWinding,
};

extern_protocol!(
    /// Bridged protocol for `MTLIndirectRenderCommand`.
    ///
    /// Availability: macOS 10.14+, iOS 12.0+
    pub unsafe trait MTLIndirectRenderCommand: NSObjectProtocol {
        /// Availability: macOS 10.14+, iOS 13.0+
        #[unsafe(method(setRenderPipelineState:))]
        #[unsafe(method_family = none)]
        unsafe fn set_render_pipeline_state(
            &self,
            pipeline_state: &ProtocolObject<dyn MTLRenderPipelineState>,
        );

        #[unsafe(method(setVertexBuffer:offset:atIndex:))]
        #[unsafe(method_family = none)]
        unsafe fn set_vertex_buffer_offset_at_index(
            &self,
            buffer: &ProtocolObject<dyn MTLBuffer>,
            offset: usize,
            index: usize,
        );

        #[unsafe(method(setFragmentBuffer:offset:atIndex:))]
        #[unsafe(method_family = none)]
        unsafe fn set_fragment_buffer_offset_at_index(
            &self,
            buffer: &ProtocolObject<dyn MTLBuffer>,
            offset: usize,
            index: usize,
        );

        /// Only call this when the buffer-index is part of the vertex descriptor and stride is dynamic.
        ///
        /// Availability: macOS 14.0+, iOS 17.0+
        #[unsafe(method(setVertexBuffer:offset:attributeStride:atIndex:))]
        #[unsafe(method_family = none)]
        unsafe fn set_vertex_buffer_offset_attribute_stride_at_index(
            &self,
            buffer: &ProtocolObject<dyn MTLBuffer>,
            offset: usize,
            stride: usize,
            index: usize,
        );

        /// Availability: tvOS 14.5+
        #[unsafe(method(drawPatches:patchStart:patchCount:patchIndexBuffer:patchIndexBufferOffset:instanceCount:baseInstance:tessellationFactorBuffer:tessellationFactorBufferOffset:tessellationFactorBufferInstanceStride:))]
        #[unsafe(method_family = none)]
        unsafe fn draw_patches_patch_start_patch_count_patch_index_buffer_patch_index_buffer_offset_instance_count_base_instance_tessellation_factor_buffer_tessellation_factor_buffer_offset_tessellation_factor_buffer_instance_stride(
            &self,
            number_of_patch_control_points: usize,
            patch_start: usize,
            patch_count: usize,
            patch_index_buffer: Option<&ProtocolObject<dyn MTLBuffer>>,
            patch_index_buffer_offset: usize,
            instance_count: usize,
            base_instance: usize,
            buffer: &ProtocolObject<dyn MTLBuffer>,
            offset: usize,
            instance_stride: usize,
        );

        /// Availability: tvOS 14.5+
        #[unsafe(method(drawIndexedPatches:patchStart:patchCount:patchIndexBuffer:patchIndexBufferOffset:controlPointIndexBuffer:controlPointIndexBufferOffset:instanceCount:baseInstance:tessellationFactorBuffer:tessellationFactorBufferOffset:tessellationFactorBufferInstanceStride:))]
        #[unsafe(method_family = none)]
        unsafe fn draw_indexed_patches_patch_start_patch_count_patch_index_buffer_patch_index_buffer_offset_control_point_index_buffer_control_point_index_buffer_offset_instance_count_base_instance_tessellation_factor_buffer_tessellation_factor_buffer_offset_tessellation_factor_buffer_instance_stride(
            &self,
            number_of_patch_control_points: usize,
            patch_start: usize,
            patch_count: usize,
            patch_index_buffer: Option<&ProtocolObject<dyn MTLBuffer>>,
            patch_index_buffer_offset: usize,
            control_point_index_buffer: &ProtocolObject<dyn MTLBuffer>,
            control_point_index_buffer_offset: usize,
            instance_count: usize,
            base_instance: usize,
            buffer: &ProtocolObject<dyn MTLBuffer>,
            offset: usize,
            instance_stride: usize,
        );

        #[unsafe(method(drawPrimitives:vertexStart:vertexCount:instanceCount:baseInstance:))]
        #[unsafe(method_family = none)]
        unsafe fn draw_primitives_vertex_start_vertex_count_instance_count_base_instance(
            &self,
            primitive_type: MTLPrimitiveType,
            vertex_start: usize,
            vertex_count: usize,
            instance_count: usize,
            base_instance: usize,
        );

        #[unsafe(method(drawIndexedPrimitives:indexCount:indexType:indexBuffer:indexBufferOffset:instanceCount:baseVertex:baseInstance:))]
        #[unsafe(method_family = none)]
        unsafe fn draw_indexed_primitives_index_count_index_type_index_buffer_index_buffer_offset_instance_count_base_vertex_base_instance(
            &self,
            primitive_type: MTLPrimitiveType,
            index_count: usize,
            index_type: MTLIndexType,
            index_buffer: &ProtocolObject<dyn MTLBuffer>,
            index_buffer_offset: usize,
            instance_count: usize,
            base_vertex: isize,
            base_instance: usize,
        );

        /// Availability: macOS 14.0+, iOS 17.0+, tvOS 18.1+, visionOS 2.1+
        #[unsafe(method(setObjectThreadgroupMemoryLength:atIndex:))]
        #[unsafe(method_family = none)]
        unsafe fn set_object_threadgroup_memory_length_at_index(&self, length: usize, index: usize);

        /// Availability: macOS 14.0+, iOS 17.0+, tvOS 18.1+, visionOS 2.1+
        #[unsafe(method(setObjectBuffer:offset:atIndex:))]
        #[unsafe(method_family = none)]
        unsafe fn set_object_buffer_offset_at_index(
            &self,
            buffer: &ProtocolObject<dyn MTLBuffer>,
            offset: usize,
            index: usize,
        );

        /// Availability: macOS 14.0+, iOS 17.0+, tvOS 18.1+, visionOS 2.1+
        #[unsafe(method(setMeshBuffer:offset:atIndex:))]
        #[unsafe(method_family = none)]
        unsafe fn set_mesh_buffer_offset_at_index(
            &self,
            buffer: &ProtocolObject<dyn MTLBuffer>,
            offset: usize,
            index: usize,
        );

        /// Availability: macOS 14.0+, iOS 17.0+, tvOS 18.1+, visionOS 2.1+
        #[unsafe(method(drawMeshThreadgroups:threadsPerObjectThreadgroup:threadsPerMeshThreadgroup:))]
        #[unsafe(method_family = none)]
        unsafe fn draw_mesh_threadgroups_threads_per_object_threadgroup_threads_per_mesh_threadgroup(
            &self,
            threadgroups_per_grid: MTLSize,
            threads_per_object_threadgroup: MTLSize,
            threads_per_mesh_threadgroup: MTLSize,
        );

        /// Availability: macOS 14.0+, iOS 17.0+, tvOS 18.1+, visionOS 2.1+
        #[unsafe(method(drawMeshThreads:threadsPerObjectThreadgroup:threadsPerMeshThreadgroup:))]
        #[unsafe(method_family = none)]
        unsafe fn draw_mesh_threads_threads_per_object_threadgroup_threads_per_mesh_threadgroup(
            &self,
            threads_per_grid: MTLSize,
            threads_per_object_threadgroup: MTLSize,
            threads_per_mesh_threadgroup: MTLSize,
        );

        /// Availability: macOS 14.0+, iOS 17.0+, tvOS 18.1+, visionOS 2.1+
        #[unsafe(method(setBarrier))]
        #[unsafe(method_family = none)]
        unsafe fn set_barrier(&self);

        /// Availability: macOS 14.0+, iOS 17.0+, tvOS 18.1+, visionOS 2.1+
        #[unsafe(method(clearBarrier))]
        #[unsafe(method_family = none)]
        unsafe fn clear_barrier(&self);

        /// Availability: macOS 26.0+, iOS 26.0+
        #[unsafe(method(setDepthStencilState:))]
        #[unsafe(method_family = none)]
        unsafe fn set_depth_stencil_state(
            &self,
            depth_stencil_state: Option<&ProtocolObject<dyn MTLDepthStencilState>>,
        );

        /// Availability: macOS 26.0+, iOS 26.0+
        #[unsafe(method(setDepthBias:slopeScale:clamp:))]
        #[unsafe(method_family = none)]
        unsafe fn set_depth_bias_slope_scale_clamp(
            &self,
            depth_bias: c_float,
            slope_scale: c_float,
            clamp: c_float,
        );

        /// Availability: macOS 26.0+, iOS 26.0+
        #[unsafe(method(setDepthClipMode:))]
        #[unsafe(method_family = none)]
        unsafe fn set_depth_clip_mode(&self, depth_clip_mode: MTLDepthClipMode);

        /// Availability: macOS 26.0+, iOS 26.0+
        #[unsafe(method(setCullMode:))]
        #[unsafe(method_family = none)]
        unsafe fn set_cull_mode(&self, cull_mode: MTLCullMode);

        /// Availability: macOS 26.0+, iOS 26.0+
        #[unsafe(method(setFrontFacingWinding:))]
        #[unsafe(method_family = none)]
        unsafe fn set_front_facing_winding(&self, front_facing_winding: MTLWinding);

        /// Availability: macOS 26.0+, iOS 26.0+
        #[unsafe(method(setTriangleFillMode:))]
        #[unsafe(method_family = none)]
        unsafe fn set_triangle_fill_mode(&self, fill_mode: MTLTriangleFillMode);

        #[unsafe(method(reset))]
        #[unsafe(method_family = none)]
        unsafe fn reset(&self);
    }
);
