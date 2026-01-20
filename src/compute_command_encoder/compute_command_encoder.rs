use core::ffi::c_void;
use core::ptr::NonNull;
use objc2::{extern_protocol, runtime::ProtocolObject};
use objc2_foundation::NSRange;

use crate::compute_pipeline::MTLComputePipelineState;
use crate::intersection_function_table::MTLIntersectionFunctionTable;
use crate::types::{MTLRegion, MTLSize};
use crate::visible_function_table::MTLVisibleFunctionTable;
use crate::{
    MTLAccelerationStructure, MTLBuffer, MTLCommandEncoder, MTLFence, MTLResource, MTLSamplerState,
    MTLTexture,
};

extern_protocol!(
    /// A command encoder that writes data parallel compute commands.
    pub unsafe trait MTLComputeCommandEncoder: MTLCommandEncoder {
        #[unsafe(method(dispatchType))]
        #[unsafe(method_family = none)]
        unsafe fn dispatch_type(&self) -> super::MTLDispatchType;

        #[unsafe(method(setComputePipelineState:))]
        #[unsafe(method_family = none)]
        fn set_compute_pipeline_state(&self, state: &ProtocolObject<dyn MTLComputePipelineState>);

        /// Set data for a buffer binding point, by copy.
        /// Safety: `bytes` must be valid.
        #[unsafe(method(setBytes:length:atIndex:))]
        #[unsafe(method_family = none)]
        unsafe fn set_bytes(&self, bytes: NonNull<c_void>, length: usize, index: usize);

        #[unsafe(method(setBuffer:offset:atIndex:))]
        #[unsafe(method_family = none)]
        unsafe fn set_buffer(
            &self,
            buffer: Option<&ProtocolObject<dyn MTLBuffer>>,
            offset: usize,
            index: usize,
        );

        #[unsafe(method(setBufferOffset:atIndex:))]
        #[unsafe(method_family = none)]
        unsafe fn set_buffer_offset(&self, offset: usize, index: usize);

        /// Safety: pointers must be valid.
        #[unsafe(method(setBuffers:offsets:withRange:))]
        #[unsafe(method_family = none)]
        unsafe fn set_buffers(
            &self,
            buffers: NonNull<*const ProtocolObject<dyn MTLBuffer>>,
            offsets: NonNull<usize>,
            range: NSRange,
        );

        #[unsafe(method(setBuffer:offset:attributeStride:atIndex:))]
        #[unsafe(method_family = none)]
        unsafe fn set_buffer_with_attribute_stride(
            &self,
            buffer: &ProtocolObject<dyn MTLBuffer>,
            offset: usize,
            stride: usize,
            index: usize,
        );

        /// Safety: pointers must be valid.
        #[unsafe(method(setBuffers:offsets:attributeStrides:withRange:))]
        #[unsafe(method_family = none)]
        unsafe fn set_buffers_with_attribute_strides(
            &self,
            buffers: NonNull<*const ProtocolObject<dyn MTLBuffer>>,
            offsets: NonNull<usize>,
            strides: NonNull<usize>,
            range: NSRange,
        );

        #[unsafe(method(setBufferOffset:attributeStride:atIndex:))]
        #[unsafe(method_family = none)]
        unsafe fn set_buffer_offset_with_attribute_stride(
            &self,
            offset: usize,
            stride: usize,
            index: usize,
        );

        /// Safety: `bytes` must be valid.
        #[unsafe(method(setBytes:length:attributeStride:atIndex:))]
        #[unsafe(method_family = none)]
        unsafe fn set_bytes_with_attribute_stride(
            &self,
            bytes: NonNull<c_void>,
            length: usize,
            stride: usize,
            index: usize,
        );

        #[unsafe(method(setVisibleFunctionTable:atBufferIndex:))]
        #[unsafe(method_family = none)]
        unsafe fn set_visible_function_table(
            &self,
            table: Option<&ProtocolObject<dyn MTLVisibleFunctionTable>>,
            buffer_index: usize,
        );

        /// Safety: `tables` must be valid.
        #[unsafe(method(setVisibleFunctionTables:withBufferRange:))]
        #[unsafe(method_family = none)]
        unsafe fn set_visible_function_tables(
            &self,
            tables: NonNull<*const ProtocolObject<dyn MTLVisibleFunctionTable>>,
            range: NSRange,
        );

        #[unsafe(method(setIntersectionFunctionTable:atBufferIndex:))]
        #[unsafe(method_family = none)]
        unsafe fn set_intersection_function_table(
            &self,
            table: Option<&ProtocolObject<dyn MTLIntersectionFunctionTable>>,
            buffer_index: usize,
        );

        /// Safety: `tables` must be valid.
        #[unsafe(method(setIntersectionFunctionTables:withBufferRange:))]
        #[unsafe(method_family = none)]
        unsafe fn set_intersection_function_tables(
            &self,
            tables: NonNull<*const ProtocolObject<dyn MTLIntersectionFunctionTable>>,
            range: NSRange,
        );

        #[unsafe(method(setAccelerationStructure:atBufferIndex:))]
        #[unsafe(method_family = none)]
        unsafe fn set_acceleration_structure(
            &self,
            structure: Option<&ProtocolObject<dyn MTLAccelerationStructure>>,
            buffer_index: usize,
        );

        #[unsafe(method(setTexture:atIndex:))]
        #[unsafe(method_family = none)]
        unsafe fn set_texture(
            &self,
            texture: Option<&ProtocolObject<dyn MTLTexture>>,
            index: usize,
        );

        /// Safety: `textures` must be valid.
        #[unsafe(method(setTextures:withRange:))]
        #[unsafe(method_family = none)]
        unsafe fn set_textures(
            &self,
            textures: NonNull<*const ProtocolObject<dyn MTLTexture>>,
            range: NSRange,
        );

        #[unsafe(method(setSamplerState:atIndex:))]
        #[unsafe(method_family = none)]
        unsafe fn set_sampler_state(
            &self,
            sampler: Option<&ProtocolObject<dyn MTLSamplerState>>,
            index: usize,
        );

        /// Safety: `samplers` must be valid.
        #[unsafe(method(setSamplerStates:withRange:))]
        #[unsafe(method_family = none)]
        unsafe fn set_sampler_states(
            &self,
            samplers: NonNull<*const ProtocolObject<dyn MTLSamplerState>>,
            range: NSRange,
        );

        #[unsafe(method(setSamplerState:lodMinClamp:lodMaxClamp:atIndex:))]
        #[unsafe(method_family = none)]
        unsafe fn set_sampler_state_with_lod(
            &self,
            sampler: Option<&ProtocolObject<dyn MTLSamplerState>>,
            lod_min_clamp: f32,
            lod_max_clamp: f32,
            index: usize,
        );

        /// Safety: pointers must be valid.
        #[unsafe(method(setSamplerStates:lodMinClamps:lodMaxClamps:withRange:))]
        #[unsafe(method_family = none)]
        unsafe fn set_sampler_states_with_lods(
            &self,
            samplers: NonNull<*const ProtocolObject<dyn MTLSamplerState>>,
            lod_min_clamps: NonNull<f32>,
            lod_max_clamps: NonNull<f32>,
            range: NSRange,
        );

        #[unsafe(method(setThreadgroupMemoryLength:atIndex:))]
        #[unsafe(method_family = none)]
        unsafe fn set_threadgroup_memory_length(&self, length: usize, index: usize);

        #[unsafe(method(setImageblockWidth:height:))]
        #[unsafe(method_family = none)]
        unsafe fn set_imageblock_width_height(&self, width: usize, height: usize);

        #[unsafe(method(setStageInRegion:))]
        #[unsafe(method_family = none)]
        unsafe fn set_stage_in_region(&self, region: MTLRegion);

        #[unsafe(method(setStageInRegionWithIndirectBuffer:indirectBufferOffset:))]
        #[unsafe(method_family = none)]
        unsafe fn set_stage_in_region_indirect(
            &self,
            indirect_buffer: &ProtocolObject<dyn MTLBuffer>,
            indirect_buffer_offset: usize,
        );

        #[unsafe(method(dispatchThreadgroups:threadsPerThreadgroup:))]
        #[unsafe(method_family = none)]
        fn dispatch_threadgroups(
            &self,
            threadgroups_per_grid: MTLSize,
            threads_per_threadgroup: MTLSize,
        );

        #[unsafe(method(dispatchThreadgroupsWithIndirectBuffer:indirectBufferOffset:threadsPerThreadgroup:))]
        #[unsafe(method_family = none)]
        unsafe fn dispatch_threadgroups_indirect(
            &self,
            indirect_buffer: &ProtocolObject<dyn MTLBuffer>,
            indirect_buffer_offset: usize,
            threads_per_threadgroup: MTLSize,
        );

        #[unsafe(method(dispatchThreads:threadsPerThreadgroup:))]
        #[unsafe(method_family = none)]
        fn dispatch_threads(&self, threads_per_grid: MTLSize, threads_per_threadgroup: MTLSize);

        #[unsafe(method(updateFence:))]
        #[unsafe(method_family = none)]
        fn update_fence(&self, fence: &ProtocolObject<dyn MTLFence>);

        #[unsafe(method(waitForFence:))]
        #[unsafe(method_family = none)]
        fn wait_for_fence(&self, fence: &ProtocolObject<dyn MTLFence>);

        #[unsafe(method(useResource:usage:))]
        #[unsafe(method_family = none)]
        fn use_resource(
            &self,
            resource: &ProtocolObject<dyn MTLResource>,
            usage: crate::MTLResourceUsage,
        );

        /// Safety: `resources` must be valid.
        #[unsafe(method(useResources:count:usage:))]
        #[unsafe(method_family = none)]
        unsafe fn use_resources(
            &self,
            resources: NonNull<NonNull<ProtocolObject<dyn MTLResource>>>,
            count: usize,
            usage: crate::MTLResourceUsage,
        );

        #[unsafe(method(useHeap:))]
        #[unsafe(method_family = none)]
        fn use_heap(&self, heap: &ProtocolObject<dyn crate::MTLHeap>);

        /// Safety: `heaps` must be valid.
        #[unsafe(method(useHeaps:count:))]
        #[unsafe(method_family = none)]
        unsafe fn use_heaps(
            &self,
            heaps: NonNull<NonNull<ProtocolObject<dyn crate::MTLHeap>>>,
            count: usize,
        );

        #[unsafe(method(executeCommandsInBuffer:withRange:))]
        #[unsafe(method_family = none)]
        unsafe fn execute_commands_in_buffer(
            &self,
            icb: &ProtocolObject<dyn crate::MTLIndirectCommandBuffer>,
            execution_range: NSRange,
        );

        #[unsafe(method(executeCommandsInBuffer:indirectBuffer:indirectBufferOffset:))]
        #[unsafe(method_family = none)]
        unsafe fn execute_commands_in_buffer_indirect(
            &self,
            icb: &ProtocolObject<dyn crate::MTLIndirectCommandBuffer>,
            indirect_range_buffer: &ProtocolObject<dyn MTLBuffer>,
            indirect_buffer_offset: usize,
        );

        #[unsafe(method(memoryBarrierWithScope:))]
        #[unsafe(method_family = none)]
        unsafe fn memory_barrier_with_scope(&self, scope: crate::MTLBarrierScope);

        /// Safety: `resources` must be valid.
        #[unsafe(method(memoryBarrierWithResources:count:))]
        #[unsafe(method_family = none)]
        unsafe fn memory_barrier_with_resources(
            &self,
            resources: NonNull<NonNull<ProtocolObject<dyn MTLResource>>>,
            count: usize,
        );

        #[unsafe(method(sampleCountersInBuffer:atSampleIndex:withBarrier:))]
        #[unsafe(method_family = none)]
        unsafe fn sample_counters_in_buffer(
            &self,
            sample_buffer: &ProtocolObject<dyn crate::MTLCounterSampleBuffer>,
            sample_index: usize,
            barrier: bool,
        );
    }
);
