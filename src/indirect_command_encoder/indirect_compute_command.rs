use objc2::{extern_protocol, runtime::ProtocolObject};
use objc2_foundation::NSObjectProtocol;

use crate::{
    MTLBuffer, MTLComputePipelineState,
    types::{MTLRegion, MTLSize},
};

extern_protocol!(
    /// Bridged protocol for `MTLIndirectComputeCommand`.
    ///
    /// Availability: macOS 11.0+, iOS 13.0+
    pub unsafe trait MTLIndirectComputeCommand: NSObjectProtocol {
        /// Availability: macOS 11.0+, iOS 13.0+
        #[unsafe(method(setComputePipelineState:))]
        #[unsafe(method_family = none)]
        unsafe fn set_compute_pipeline_state(
            &self,
            pipeline_state: &ProtocolObject<dyn MTLComputePipelineState>,
        );

        #[unsafe(method(setKernelBuffer:offset:atIndex:))]
        #[unsafe(method_family = none)]
        unsafe fn set_kernel_buffer_offset_at_index(
            &self,
            buffer: &ProtocolObject<dyn MTLBuffer>,
            offset: usize,
            index: usize,
        );

        /// Only call when stride is dynamic per stage input descriptor.
        ///
        /// Availability: macOS 14.0+, iOS 17.0+
        #[unsafe(method(setKernelBuffer:offset:attributeStride:atIndex:))]
        #[unsafe(method_family = none)]
        unsafe fn set_kernel_buffer_offset_attribute_stride_at_index(
            &self,
            buffer: &ProtocolObject<dyn MTLBuffer>,
            offset: usize,
            stride: usize,
            index: usize,
        );

        #[unsafe(method(concurrentDispatchThreadgroups:threadsPerThreadgroup:))]
        #[unsafe(method_family = none)]
        unsafe fn concurrent_dispatch_threadgroups_threads_per_threadgroup(
            &self,
            threadgroups_per_grid: MTLSize,
            threads_per_threadgroup: MTLSize,
        );

        #[unsafe(method(concurrentDispatchThreads:threadsPerThreadgroup:))]
        #[unsafe(method_family = none)]
        unsafe fn concurrent_dispatch_threads_threads_per_threadgroup(
            &self,
            threads_per_grid: MTLSize,
            threads_per_threadgroup: MTLSize,
        );

        #[unsafe(method(setBarrier))]
        #[unsafe(method_family = none)]
        unsafe fn set_barrier(&self);

        #[unsafe(method(clearBarrier))]
        #[unsafe(method_family = none)]
        unsafe fn clear_barrier(&self);

        /// Availability: macOS 11.0+ (iOS 14.0+ for setter)
        #[unsafe(method(setImageblockWidth:height:))]
        #[unsafe(method_family = none)]
        unsafe fn set_imageblock_width_height(&self, width: usize, height: usize);

        #[unsafe(method(reset))]
        #[unsafe(method_family = none)]
        unsafe fn reset(&self);

        #[unsafe(method(setThreadgroupMemoryLength:atIndex:))]
        #[unsafe(method_family = none)]
        unsafe fn set_threadgroup_memory_length_at_index(&self, length: usize, index: usize);

        #[unsafe(method(setStageInRegion:))]
        #[unsafe(method_family = none)]
        unsafe fn set_stage_in_region(&self, region: MTLRegion);
    }
);
