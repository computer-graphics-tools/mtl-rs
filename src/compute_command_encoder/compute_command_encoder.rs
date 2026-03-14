use core::{ffi::c_void, ops::Range, ptr::NonNull};

use objc2::{Message, extern_protocol, msg_send, runtime::ProtocolObject};
use objc2_foundation::NSRange;

use crate::{
    MTLAccelerationStructure, MTLBarrierScope, MTLBuffer, MTLCommandEncoder, MTLCounterSampleBuffer, MTLFence, MTLHeap,
    MTLIndirectCommandBuffer, MTLResource, MTLResourceUsage, MTLSamplerState, MTLTexture,
    compute_pipeline::MTLComputePipelineState,
    intersection_function_table::MTLIntersectionFunctionTable,
    types::{MTLRegion, MTLSize},
    visible_function_table::MTLVisibleFunctionTable,
};

extern_protocol!(
    /// A command encoder that writes data parallel compute commands.
    pub unsafe trait MTLComputeCommandEncoder: MTLCommandEncoder {
        #[unsafe(method(dispatchType))]
        #[unsafe(method_family = none)]
        fn dispatch_type(&self) -> super::MTLDispatchType;

        #[unsafe(method(setComputePipelineState:))]
        #[unsafe(method_family = none)]
        fn set_compute_pipeline_state(
            &self,
            state: &ProtocolObject<dyn MTLComputePipelineState>,
        );

        /// Set data for a buffer binding point, by copy.
        /// Safety: `bytes` must be valid.
        #[unsafe(method(setBytes:length:atIndex:))]
        #[unsafe(method_family = none)]
        fn set_bytes(
            &self,
            bytes: NonNull<c_void>,
            length: usize,
            index: usize,
        );

        #[unsafe(method(setBuffer:offset:atIndex:))]
        #[unsafe(method_family = none)]
        fn set_buffer(
            &self,
            buffer: Option<&ProtocolObject<dyn MTLBuffer>>,
            offset: usize,
            index: usize,
        );

        #[unsafe(method(setBufferOffset:atIndex:))]
        #[unsafe(method_family = none)]
        fn set_buffer_offset(
            &self,
            offset: usize,
            index: usize,
        );

        #[unsafe(method(setBuffer:offset:attributeStride:atIndex:))]
        #[unsafe(method_family = none)]
        fn set_buffer_with_attribute_stride(
            &self,
            buffer: &ProtocolObject<dyn MTLBuffer>,
            offset: usize,
            stride: usize,
            index: usize,
        );

        #[unsafe(method(setBufferOffset:attributeStride:atIndex:))]
        #[unsafe(method_family = none)]
        fn set_buffer_offset_with_attribute_stride(
            &self,
            offset: usize,
            stride: usize,
            index: usize,
        );

        /// Safety: `bytes` must be valid.
        #[unsafe(method(setBytes:length:attributeStride:atIndex:))]
        #[unsafe(method_family = none)]
        fn set_bytes_with_attribute_stride(
            &self,
            bytes: NonNull<c_void>,
            length: usize,
            stride: usize,
            index: usize,
        );

        #[unsafe(method(setVisibleFunctionTable:atBufferIndex:))]
        #[unsafe(method_family = none)]
        fn set_visible_function_table(
            &self,
            table: Option<&ProtocolObject<dyn MTLVisibleFunctionTable>>,
            buffer_index: usize,
        );

        #[unsafe(method(setIntersectionFunctionTable:atBufferIndex:))]
        #[unsafe(method_family = none)]
        fn set_intersection_function_table(
            &self,
            table: Option<&ProtocolObject<dyn MTLIntersectionFunctionTable>>,
            buffer_index: usize,
        );

        #[unsafe(method(setAccelerationStructure:atBufferIndex:))]
        #[unsafe(method_family = none)]
        fn set_acceleration_structure(
            &self,
            structure: Option<&ProtocolObject<dyn MTLAccelerationStructure>>,
            buffer_index: usize,
        );

        #[unsafe(method(setTexture:atIndex:))]
        #[unsafe(method_family = none)]
        fn set_texture(
            &self,
            texture: Option<&ProtocolObject<dyn MTLTexture>>,
            index: usize,
        );

        #[unsafe(method(setSamplerState:atIndex:))]
        #[unsafe(method_family = none)]
        fn set_sampler_state(
            &self,
            sampler: Option<&ProtocolObject<dyn MTLSamplerState>>,
            index: usize,
        );

        #[unsafe(method(setSamplerState:lodMinClamp:lodMaxClamp:atIndex:))]
        #[unsafe(method_family = none)]
        fn set_sampler_state_with_lod(
            &self,
            sampler: Option<&ProtocolObject<dyn MTLSamplerState>>,
            lod_min_clamp: f32,
            lod_max_clamp: f32,
            index: usize,
        );

        #[unsafe(method(setThreadgroupMemoryLength:atIndex:))]
        #[unsafe(method_family = none)]
        fn set_threadgroup_memory_length(
            &self,
            length: usize,
            index: usize,
        );

        #[unsafe(method(setImageblockWidth:height:))]
        #[unsafe(method_family = none)]
        fn set_imageblock_width_height(
            &self,
            width: usize,
            height: usize,
        );

        #[unsafe(method(setStageInRegion:))]
        #[unsafe(method_family = none)]
        fn set_stage_in_region(
            &self,
            region: MTLRegion,
        );

        #[unsafe(method(setStageInRegionWithIndirectBuffer:indirectBufferOffset:))]
        #[unsafe(method_family = none)]
        fn set_stage_in_region_indirect(
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
        fn dispatch_threadgroups_indirect(
            &self,
            indirect_buffer: &ProtocolObject<dyn MTLBuffer>,
            indirect_buffer_offset: usize,
            threads_per_threadgroup: MTLSize,
        );

        #[unsafe(method(dispatchThreads:threadsPerThreadgroup:))]
        #[unsafe(method_family = none)]
        fn dispatch_threads(
            &self,
            threads_per_grid: MTLSize,
            threads_per_threadgroup: MTLSize,
        );

        #[unsafe(method(updateFence:))]
        #[unsafe(method_family = none)]
        fn update_fence(
            &self,
            fence: &ProtocolObject<dyn MTLFence>,
        );

        #[unsafe(method(waitForFence:))]
        #[unsafe(method_family = none)]
        fn wait_for_fence(
            &self,
            fence: &ProtocolObject<dyn MTLFence>,
        );

        #[unsafe(method(useResource:usage:))]
        #[unsafe(method_family = none)]
        fn use_resource(
            &self,
            resource: &ProtocolObject<dyn MTLResource>,
            usage: MTLResourceUsage,
        );

        /// Safety: `resources` must be valid.
        #[unsafe(method(useResources:count:usage:))]
        #[unsafe(method_family = none)]
        fn use_resources(
            &self,
            resources: NonNull<NonNull<ProtocolObject<dyn MTLResource>>>,
            count: usize,
            usage: MTLResourceUsage,
        );

        #[unsafe(method(useHeap:))]
        #[unsafe(method_family = none)]
        fn use_heap(
            &self,
            heap: &ProtocolObject<dyn MTLHeap>,
        );

        /// Safety: `heaps` must be valid.
        #[unsafe(method(useHeaps:count:))]
        #[unsafe(method_family = none)]
        fn use_heaps(
            &self,
            heaps: NonNull<NonNull<ProtocolObject<dyn MTLHeap>>>,
            count: usize,
        );

        #[unsafe(method(executeCommandsInBuffer:indirectBuffer:indirectBufferOffset:))]
        #[unsafe(method_family = none)]
        fn execute_commands_in_buffer_indirect(
            &self,
            icb: &ProtocolObject<dyn MTLIndirectCommandBuffer>,
            indirect_range_buffer: &ProtocolObject<dyn MTLBuffer>,
            indirect_buffer_offset: usize,
        );

        #[unsafe(method(memoryBarrierWithScope:))]
        #[unsafe(method_family = none)]
        fn memory_barrier_with_scope(
            &self,
            scope: MTLBarrierScope,
        );

        /// Safety: `resources` must be valid.
        #[unsafe(method(memoryBarrierWithResources:count:))]
        #[unsafe(method_family = none)]
        fn memory_barrier_with_resources(
            &self,
            resources: NonNull<NonNull<ProtocolObject<dyn MTLResource>>>,
            count: usize,
        );

        #[unsafe(method(sampleCountersInBuffer:atSampleIndex:withBarrier:))]
        #[unsafe(method_family = none)]
        fn sample_counters_in_buffer(
            &self,
            sample_buffer: &ProtocolObject<dyn MTLCounterSampleBuffer>,
            sample_index: usize,
            barrier: bool,
        );
    }
);

pub trait MTLComputeCommandEncoderExt: MTLComputeCommandEncoder + Message {
    /// Safety: pointers must be valid.
    fn set_buffers_with_range(
        &self,
        buffers: NonNull<*const ProtocolObject<dyn MTLBuffer>>,
        offsets: NonNull<usize>,
        range: Range<usize>,
    ) where
        Self: Sized,
    {
        unsafe {
            let _: () = msg_send![
                self,
                setBuffers: buffers,
                offsets: offsets,
                withRange: NSRange::from(range)
            ];
        }
    }

    /// Safety: pointers must be valid.
    fn set_buffers_with_attribute_strides_with_range(
        &self,
        buffers: NonNull<*const ProtocolObject<dyn MTLBuffer>>,
        offsets: NonNull<usize>,
        strides: NonNull<usize>,
        range: Range<usize>,
    ) where
        Self: Sized,
    {
        unsafe {
            let _: () = msg_send![
                self,
                setBuffers: buffers,
                offsets: offsets,
                attributeStrides: strides,
                withRange: NSRange::from(range)
            ];
        }
    }

    /// Safety: `tables` must be valid.
    fn set_visible_function_tables_with_range(
        &self,
        tables: NonNull<*const ProtocolObject<dyn MTLVisibleFunctionTable>>,
        range: Range<usize>,
    ) where
        Self: Sized,
    {
        unsafe {
            let _: () = msg_send![
                self,
                setVisibleFunctionTables: tables,
                withBufferRange: NSRange::from(range)
            ];
        }
    }

    /// Safety: `tables` must be valid.
    fn set_intersection_function_tables_with_range(
        &self,
        tables: NonNull<*const ProtocolObject<dyn MTLIntersectionFunctionTable>>,
        range: Range<usize>,
    ) where
        Self: Sized,
    {
        unsafe {
            let _: () = msg_send![
                self,
                setIntersectionFunctionTables: tables,
                withBufferRange: NSRange::from(range)
            ];
        }
    }

    /// Safety: `textures` must be valid.
    fn set_textures_with_range(
        &self,
        textures: NonNull<*const ProtocolObject<dyn MTLTexture>>,
        range: Range<usize>,
    ) where
        Self: Sized,
    {
        unsafe {
            let _: () = msg_send![
                self,
                setTextures: textures,
                withRange: NSRange::from(range)
            ];
        }
    }

    /// Safety: `samplers` must be valid.
    fn set_sampler_states_with_range(
        &self,
        samplers: NonNull<*const ProtocolObject<dyn MTLSamplerState>>,
        range: Range<usize>,
    ) where
        Self: Sized,
    {
        unsafe {
            let _: () = msg_send![
                self,
                setSamplerStates: samplers,
                withRange: NSRange::from(range)
            ];
        }
    }

    /// Safety: pointers must be valid.
    fn set_sampler_states_with_lods_with_range(
        &self,
        samplers: NonNull<*const ProtocolObject<dyn MTLSamplerState>>,
        lod_min_clamps: NonNull<f32>,
        lod_max_clamps: NonNull<f32>,
        range: Range<usize>,
    ) where
        Self: Sized,
    {
        unsafe {
            let _: () = msg_send![
                self,
                setSamplerStates: samplers,
                lodMinClamps: lod_min_clamps,
                lodMaxClamps: lod_max_clamps,
                withRange: NSRange::from(range)
            ];
        }
    }

    fn execute_commands_in_buffer_with_range(
        &self,
        icb: &ProtocolObject<dyn MTLIndirectCommandBuffer>,
        execution_range: Range<usize>,
    ) where
        Self: Sized,
    {
        unsafe {
            let _: () = msg_send![
                self,
                executeCommandsInBuffer: icb,
                withRange: NSRange::from(execution_range)
            ];
        }
    }
}

impl<T: MTLComputeCommandEncoder + Message> MTLComputeCommandEncoderExt for T {}
