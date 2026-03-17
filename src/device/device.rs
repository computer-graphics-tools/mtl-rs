use std::{ffi::c_void, path::Path, ptr::NonNull};

use dispatch2::DispatchData;
use objc2::{Message, extern_protocol, msg_send, rc::Retained, runtime::ProtocolObject};
use objc2_foundation::{NSArray, NSError, NSObjectProtocol, NSString, NSURL};

use super::{MTLArchitecture, MTLSizeAndAlign};
use crate::{
    MTL4Archive, MTL4ArgumentTable, MTL4ArgumentTableDescriptor, MTL4BinaryFunction,
    MTL4CommandAllocatorDescriptor, MTL4CommandQueueDescriptor, MTL4CompilerDescriptor, MTL4CounterHeap,
    MTL4CounterHeapDescriptor, MTL4CounterHeapType, MTL4PipelineDataSetSerializer,
    MTL4PipelineDataSetSerializerDescriptor, MTLAccelerationStructure, MTLArgumentBuffersTier, MTLArgumentEncoder,
    MTLBuffer, MTLCommandQueue, MTLCompileOptions, MTLComputePipelineDescriptor, MTLComputePipelineState,
    MTLCounterSampleBuffer, MTLCounterSampleBufferDescriptor, MTLCounterSamplingPoint, MTLCounterSet,
    MTLDepthStencilDescriptor, MTLDepthStencilState, MTLDeviceLocation, MTLDynamicLibrary, MTLEvent, MTLFeatureSet,
    MTLFence, MTLFunction, MTLFunctionHandle, MTLGPUFamily, MTLHeap, MTLHeapDescriptor, MTLIOCommandQueue,
    MTLIOCommandQueueDescriptor, MTLLibrary, MTLLogState, MTLLogStateDescriptor, MTLPipelineOption, MTLPixelFormat,
    MTLReadWriteTextureTier, MTLRenderPipelineDescriptor, MTLRenderPipelineState, MTLResidencySet,
    MTLResidencySetDescriptor, MTLResourceOptions, MTLResourceViewPoolDescriptor, MTLSamplerDescriptor,
    MTLSamplerState, MTLSharedEvent, MTLSharedEventHandle, MTLSize, MTLSparsePageSize, MTLTensor,
    MTLTensorDescriptor, MTLTexture, MTLTextureDescriptor, MTLTextureViewPool,
    acceleration_structure::MTLAccelerationStructureDescriptor, argument::MTLArgumentDescriptor,
    compute_pipeline::MTLComputePipelineReflection, function_stitching::MTLStitchedLibraryDescriptor,
};

extern_protocol!(
    /// A Metal device that represents a GPU.
    ///
    /// See Apple's documentation for  for details.
    pub unsafe trait MTLDevice: NSObjectProtocol + Send + Sync {
        /// Returns the IORegistry ID for the Metal device.
        #[unsafe(method(registryID))]
        #[unsafe(method_family = none)]
        fn registry_id(&self) -> u64;

        /// The device's architecture information.
        #[unsafe(method(architecture))]
        #[unsafe(method_family = none)]
        fn architecture(&self) -> Retained<MTLArchitecture>;

        /// The maximum number of threads along each dimension.
        #[unsafe(method(maxThreadsPerThreadgroup))]
        #[unsafe(method_family = none)]
        fn max_threads_per_threadgroup(&self) -> MTLSize;

        /// Whether this GPU is a low-power device.
        #[unsafe(method(isLowPower))]
        #[unsafe(method_family = none)]
        fn is_low_power(&self) -> bool;

        /// Whether this GPU has no displays attached.
        #[unsafe(method(isHeadless))]
        #[unsafe(method_family = none)]
        fn is_headless(&self) -> bool;

        /// Whether this GPU shares its memory with the CPU.
        #[unsafe(method(hasUnifiedMemory))]
        #[unsafe(method_family = none)]
        fn has_unified_memory(&self) -> bool;

        /// Approximation of how much memory this device can use with good performance.
        #[unsafe(method(recommendedMaxWorkingSetSize))]
        #[unsafe(method_family = none)]
        fn recommended_max_working_set_size(&self) -> u64;

        /// Preferred location of the GPU relative to the host.
        #[unsafe(method(location))]
        #[unsafe(method_family = none)]
        fn location(&self) -> MTLDeviceLocation;

        /// Further specifies the GPU's location (slot or port number).
        #[unsafe(method(locationNumber))]
        #[unsafe(method_family = none)]
        fn location_number(&self) -> usize;

        /// Upper bound of System RAM <=> VRAM transfer rate in bytes/sec.
        #[unsafe(method(maxTransferRate))]
        #[unsafe(method_family = none)]
        fn max_transfer_rate(&self) -> u64;

        /// Query support tier for read-write texture formats.
        #[unsafe(method(readWriteTextureSupport))]
        #[unsafe(method_family = none)]
        fn read_write_texture_support(&self) -> MTLReadWriteTextureTier;

        /// Query support tier for argument buffers.
        #[unsafe(method(argumentBuffersSupport))]
        #[unsafe(method_family = none)]
        fn argument_buffers_support(&self) -> MTLArgumentBuffersTier;

        /// Whether the device supports MTLPixelFormatDepth24Unorm_Stencil8.
        #[unsafe(method(isDepth24Stencil8PixelFormatSupported))]
        #[unsafe(method_family = none)]
        fn is_depth24_stencil8_pixel_format_supported(&self) -> bool;

        /// Whether the device supports the specified GPU family.
        #[unsafe(method(supportsFamily:))]
        #[unsafe(method_family = none)]
        fn supports_family(
            &self,
            family: MTLGPUFamily,
        ) -> bool;

        /// Whether the device supports textures with a given sample count.
        #[unsafe(method(supportsTextureSampleCount:))]
        #[unsafe(method_family = none)]
        fn supports_texture_sample_count(
            &self,
            sample_count: usize,
        ) -> bool;

        /// Minimum alignment for linear textures for a pixel format (offset and rowBytes).
        #[unsafe(method(minimumLinearTextureAlignmentForPixelFormat:))]
        #[unsafe(method_family = none)]
        fn minimum_linear_texture_alignment_for_pixel_format(
            &self,
            format: MTLPixelFormat,
        ) -> usize;

        /// Minimum alignment for texture buffers for a pixel format.
        #[unsafe(method(minimumTextureBufferAlignmentForPixelFormat:))]
        #[unsafe(method_family = none)]
        fn minimum_texture_buffer_alignment_for_pixel_format(
            &self,
            format: MTLPixelFormat,
        ) -> usize;

        /// Returns a new depth/stencil state object.
        #[unsafe(method(newDepthStencilStateWithDescriptor:))]
        #[unsafe(method_family = new)]
        fn new_depth_stencil_state_with_descriptor(
            &self,
            descriptor: &MTLDepthStencilDescriptor,
        ) -> Option<Retained<ProtocolObject<dyn MTLDepthStencilState>>>;

        /// Allocate a new counter sample buffer.
        #[unsafe(method(newCounterSampleBufferWithDescriptor:error:_))]
        #[unsafe(method_family = none)]
        fn new_counter_sample_buffer_with_descriptor_error(
            &self,
            descriptor: &MTLCounterSampleBufferDescriptor,
        ) -> Result<Retained<ProtocolObject<dyn MTLCounterSampleBuffer>>, Retained<NSError>>;

        /// Sample the CPU and GPU timestamps as closely as possible.
        #[unsafe(method(sampleTimestamps:gpuTimestamp:))]
        #[unsafe(method_family = none)]
        fn sample_timestamps_gpu_timestamp(
            &self,
            cpu_timestamp: *mut u64,
            gpu_timestamp: *mut u64,
        );

        /// Query device for counter sampling points support.
        #[unsafe(method(supportsCounterSampling:))]
        #[unsafe(method_family = none)]
        fn supports_counter_sampling(
            &self,
            sampling_point: MTLCounterSamplingPoint,
        ) -> bool;

        /// Whether the device supports query of texture LOD.
        #[unsafe(method(supportsQueryTextureLOD))]
        #[unsafe(method_family = none)]
        fn supports_query_texture_lod(&self) -> bool;

        /// Whether the device supports BC texture compression.
        #[unsafe(method(supportsBCTextureCompression))]
        #[unsafe(method_family = none)]
        fn supports_bc_texture_compression(&self) -> bool;

        /// Whether the device supports pull model interpolation.
        #[unsafe(method(supportsPullModelInterpolation))]
        #[unsafe(method_family = none)]
        fn supports_pull_model_interpolation(&self) -> bool;

        /// Deprecated in favor of .
        #[unsafe(method(areBarycentricCoordsSupported))]
        #[unsafe(method_family = none)]
        fn barycentric_coords_supported(&self) -> bool;

        /// Whether the device supports shader barycentric coordinates.
        #[unsafe(method(supportsShaderBarycentricCoordinates))]
        #[unsafe(method_family = none)]
        fn supports_shader_barycentric_coordinates(&self) -> bool;

        /// Current size in bytes of all resources allocated by this device.
        #[unsafe(method(currentAllocatedSize))]
        #[unsafe(method_family = none)]
        fn current_allocated_size(&self) -> usize;
    }
);

#[allow(unused)]
pub trait MTLDeviceExt: MTLDevice + Message {
    // -- Properties (header order) --

    fn name(&self) -> String;
    fn raster_order_groups_supported(&self) -> bool;
    fn max_threadgroup_memory_length(&self) -> usize;
    fn max_buffer_length(&self) -> usize;
    fn counter_sets(&self) -> Option<Box<[Retained<ProtocolObject<dyn MTLCounterSet>>]>>;

    // -- Methods (header order) --

    fn new_log_state_with_descriptor(&self, descriptor: &MTLLogStateDescriptor) -> Result<Retained<ProtocolObject<dyn MTLLogState>>, Retained<NSError>>;
    fn new_command_queue(&self) -> Option<Retained<ProtocolObject<dyn MTLCommandQueue>>>;
    fn new_command_queue_with_max_command_buffer_count(&self, max_count: usize) -> Option<Retained<ProtocolObject<dyn MTLCommandQueue>>>;
    fn new_heap_with_descriptor(&self, descriptor: &MTLHeapDescriptor) -> Option<Retained<ProtocolObject<dyn MTLHeap>>>;
    fn new_buffer(&self, length: usize, options: MTLResourceOptions) -> Option<Retained<ProtocolObject<dyn MTLBuffer>>>;
    fn new_buffer_with_data(&self, data: &[u8], options: MTLResourceOptions) -> Option<Retained<ProtocolObject<dyn MTLBuffer>>>;
    fn new_buffer_with_bytes_no_copy(&self, ptr: NonNull<c_void>, length: usize, options: MTLResourceOptions) -> Option<Retained<ProtocolObject<dyn MTLBuffer>>>;
    fn new_texture_with_descriptor(&self, descriptor: &MTLTextureDescriptor) -> Option<Retained<ProtocolObject<dyn MTLTexture>>>;
    fn new_sampler_state_with_descriptor(&self, descriptor: &MTLSamplerDescriptor) -> Option<Retained<ProtocolObject<dyn MTLSamplerState>>>;
    fn new_default_library(&self) -> Option<Retained<ProtocolObject<dyn MTLLibrary>>>;
    fn new_library_with_path(&self, path: &Path) -> Result<Retained<ProtocolObject<dyn MTLLibrary>>, Retained<NSError>>;
    fn new_library_with_data(&self, data: &[u8]) -> Result<Retained<ProtocolObject<dyn MTLLibrary>>, Retained<NSError>>;
    fn new_library_with_source(&self, source: &str, options: Option<&MTLCompileOptions>) -> Result<Retained<ProtocolObject<dyn MTLLibrary>>, Retained<NSError>>;
    fn new_library_with_stitched_descriptor(&self, descriptor: &MTLStitchedLibraryDescriptor) -> Result<Retained<ProtocolObject<dyn MTLLibrary>>, Retained<NSError>>;
    fn new_render_pipeline_state_with_descriptor(&self, descriptor: &MTLRenderPipelineDescriptor) -> Result<Retained<ProtocolObject<dyn MTLRenderPipelineState>>, Retained<NSError>>;
    fn new_compute_pipeline_state_with_function(&self, function: &ProtocolObject<dyn MTLFunction>) -> Result<Retained<ProtocolObject<dyn MTLComputePipelineState>>, Retained<NSError>>;
    fn new_compute_pipeline_state_with_descriptor(&self, descriptor: &MTLComputePipelineDescriptor, options: MTLPipelineOption) -> Result<(Retained<ProtocolObject<dyn MTLComputePipelineState>>, Option<Retained<MTLComputePipelineReflection>>), Retained<NSError>>;
    fn new_fence(&self) -> Option<Retained<ProtocolObject<dyn MTLFence>>>;
    fn supports_feature_set(&self, feature_set: MTLFeatureSet) -> bool;
    fn new_argument_encoder_with_arguments(&self, arguments: &[&MTLArgumentDescriptor]) -> Option<Retained<ProtocolObject<dyn MTLArgumentEncoder>>>;
    fn new_event(&self) -> Option<Retained<ProtocolObject<dyn MTLEvent>>>;
    fn new_shared_event(&self) -> Option<Retained<ProtocolObject<dyn MTLSharedEvent>>>;
    fn new_shared_event_with_handle(&self, handle: &MTLSharedEventHandle) -> Option<Retained<ProtocolObject<dyn MTLSharedEvent>>>;
    fn new_io_command_queue_with_descriptor(&self, descriptor: &MTLIOCommandQueueDescriptor) -> Result<Retained<ProtocolObject<dyn MTLIOCommandQueue>>, Retained<NSError>>;
    fn new_dynamic_library(&self, library: &ProtocolObject<dyn MTLLibrary>) -> Result<Retained<ProtocolObject<dyn MTLDynamicLibrary>>, Retained<NSError>>;
    fn new_dynamic_library_with_path(&self, path: &Path) -> Result<Retained<ProtocolObject<dyn MTLDynamicLibrary>>, Retained<NSError>>;
    fn new_acceleration_structure_with_size(&self, size: usize) -> Option<Retained<ProtocolObject<dyn MTLAccelerationStructure>>>;
    fn new_acceleration_structure_with_descriptor(&self, descriptor: &MTLAccelerationStructureDescriptor) -> Option<Retained<ProtocolObject<dyn MTLAccelerationStructure>>>;
    fn new_residency_set_with_descriptor(&self, descriptor: &MTLResidencySetDescriptor) -> Result<Retained<ProtocolObject<dyn MTLResidencySet>>, Retained<NSError>>;
    fn tensor_size_and_align_with_descriptor(&self, descriptor: &MTLTensorDescriptor) -> MTLSizeAndAlign;
    fn new_tensor_with_descriptor(&self, descriptor: &MTLTensorDescriptor) -> Result<Retained<ProtocolObject<dyn MTLTensor>>, Retained<NSError>>;
    fn function_handle_with_function(&self, function: &ProtocolObject<dyn MTLFunction>) -> Option<Retained<ProtocolObject<dyn MTLFunctionHandle>>>;
    fn new_command_allocator(&self) -> Option<Retained<ProtocolObject<dyn crate::MTL4CommandAllocator>>>;
    fn new_command_allocator_with_descriptor(&self, descriptor: &MTL4CommandAllocatorDescriptor) -> Result<Retained<ProtocolObject<dyn crate::MTL4CommandAllocator>>, Retained<NSError>>;
    fn new_mtl4_command_queue(&self) -> Option<Retained<ProtocolObject<dyn crate::MTL4CommandQueue>>>;
    fn new_mtl4_command_queue_with_descriptor(&self, descriptor: &MTL4CommandQueueDescriptor) -> Result<Retained<ProtocolObject<dyn crate::MTL4CommandQueue>>, Retained<NSError>>;
    fn new_mtl4_command_buffer(&self) -> Option<Retained<ProtocolObject<dyn crate::MTL4CommandBuffer>>>;
    fn new_argument_table_with_descriptor(&self, descriptor: &MTL4ArgumentTableDescriptor) -> Result<Retained<ProtocolObject<dyn MTL4ArgumentTable>>, Retained<NSError>>;
    fn new_texture_view_pool_with_descriptor(&self, descriptor: &MTLResourceViewPoolDescriptor) -> Result<Retained<ProtocolObject<dyn MTLTextureViewPool>>, Retained<NSError>>;
    fn new_compiler_with_descriptor(&self, descriptor: &MTL4CompilerDescriptor) -> Result<Retained<ProtocolObject<dyn crate::MTL4Compiler>>, Retained<NSError>>;
    fn new_archive_with_url(&self, url: &NSURL) -> Result<Retained<ProtocolObject<dyn MTL4Archive>>, Retained<NSError>>;
    fn new_pipeline_data_set_serializer_with_descriptor(&self, descriptor: &MTL4PipelineDataSetSerializerDescriptor) -> Retained<ProtocolObject<dyn MTL4PipelineDataSetSerializer>>;
    fn new_buffer_with_length_options_placement_sparse_page_size(&self, length: usize, options: MTLResourceOptions, placement_sparse_page_size: MTLSparsePageSize) -> Option<Retained<ProtocolObject<dyn MTLBuffer>>>;
    fn new_counter_heap_with_descriptor(&self, descriptor: &MTL4CounterHeapDescriptor) -> Result<Retained<ProtocolObject<dyn MTL4CounterHeap>>, Retained<NSError>>;
    fn size_of_counter_heap_entry(&self, counter_heap_type: MTL4CounterHeapType) -> usize;
    fn query_timestamp_frequency(&self) -> u64;
    fn function_handle_with_binary_function(&self, function: &ProtocolObject<dyn MTL4BinaryFunction>) -> Option<Retained<ProtocolObject<dyn MTLFunctionHandle>>>;
}

impl MTLDeviceExt for ProtocolObject<dyn MTLDevice> {
    // -- Properties --

    fn name(&self) -> String {
        let ns: Retained<NSString> = unsafe { msg_send![self, name] };
        ns.to_string()
    }

    fn raster_order_groups_supported(&self) -> bool {
        unsafe { msg_send![self, areRasterOrderGroupsSupported] }
    }

    fn max_threadgroup_memory_length(&self) -> usize {
        unsafe { msg_send![self, maxThreadgroupMemoryLength] }
    }

    fn max_buffer_length(&self) -> usize {
        unsafe { msg_send![self, maxBufferLength] }
    }

    fn counter_sets(&self) -> Option<Box<[Retained<ProtocolObject<dyn MTLCounterSet>>]>> {
        let array: Option<Retained<NSArray<ProtocolObject<dyn MTLCounterSet>>>> =
            unsafe { msg_send![self, counterSets] };
        array.map(|a| a.to_vec().into_boxed_slice())
    }

    // -- Methods --

    fn new_log_state_with_descriptor(
        &self,
        descriptor: &MTLLogStateDescriptor,
    ) -> Result<Retained<ProtocolObject<dyn MTLLogState>>, Retained<NSError>> {
        unsafe { msg_send![self, newLogStateWithDescriptor: descriptor, error: _] }
    }

    fn new_command_queue(&self) -> Option<Retained<ProtocolObject<dyn MTLCommandQueue>>> {
        unsafe { msg_send![self, newCommandQueue] }
    }

    fn new_command_queue_with_max_command_buffer_count(
        &self,
        max_count: usize,
    ) -> Option<Retained<ProtocolObject<dyn MTLCommandQueue>>> {
        unsafe { msg_send![self, newCommandQueueWithMaxCommandBufferCount: max_count] }
    }

    fn new_heap_with_descriptor(
        &self,
        descriptor: &MTLHeapDescriptor,
    ) -> Option<Retained<ProtocolObject<dyn MTLHeap>>> {
        unsafe { msg_send![self, newHeapWithDescriptor: descriptor] }
    }

    fn new_buffer(
        &self,
        length: usize,
        options: MTLResourceOptions,
    ) -> Option<Retained<ProtocolObject<dyn MTLBuffer>>> {
        unsafe { msg_send![self, newBufferWithLength: length, options: options] }
    }

    fn new_buffer_with_data(
        &self,
        data: &[u8],
        options: MTLResourceOptions,
    ) -> Option<Retained<ProtocolObject<dyn MTLBuffer>>> {
        unsafe {
            msg_send![
                self,
                newBufferWithBytes: data.as_ptr() as *const c_void,
                length: data.len(),
                options: options
            ]
        }
    }

    fn new_buffer_with_bytes_no_copy(
        &self,
        ptr: NonNull<c_void>,
        length: usize,
        options: MTLResourceOptions,
    ) -> Option<Retained<ProtocolObject<dyn MTLBuffer>>> {
        let deallocator: Option<&crate::block2::Block<dyn Fn(*mut c_void, usize)>> = None;
        unsafe {
            msg_send![
                self,
                newBufferWithBytesNoCopy: ptr.as_ptr(),
                length: length,
                options: options,
                deallocator: deallocator
            ]
        }
    }

    fn new_texture_with_descriptor(
        &self,
        descriptor: &MTLTextureDescriptor,
    ) -> Option<Retained<ProtocolObject<dyn MTLTexture>>> {
        unsafe { msg_send![self, newTextureWithDescriptor: descriptor] }
    }

    fn new_sampler_state_with_descriptor(
        &self,
        descriptor: &MTLSamplerDescriptor,
    ) -> Option<Retained<ProtocolObject<dyn MTLSamplerState>>> {
        unsafe { msg_send![self, newSamplerStateWithDescriptor: descriptor] }
    }

    fn new_default_library(&self) -> Option<Retained<ProtocolObject<dyn MTLLibrary>>> {
        unsafe { msg_send![self, newDefaultLibrary] }
    }

    fn new_library_with_path(
        &self,
        path: &Path,
    ) -> Result<Retained<ProtocolObject<dyn MTLLibrary>>, Retained<NSError>> {
        let url = NSURL::from_file_path(path).expect("path must be a valid file URL path");
        unsafe { msg_send![self, newLibraryWithURL: &*url, error: _] }
    }

    fn new_library_with_data(
        &self,
        data: &[u8],
    ) -> Result<Retained<ProtocolObject<dyn MTLLibrary>>, Retained<NSError>> {
        let dispatch_data = DispatchData::from_bytes(data);
        unsafe { msg_send![self, newLibraryWithData: &*dispatch_data, error: _] }
    }

    fn new_library_with_source(
        &self,
        source: &str,
        options: Option<&MTLCompileOptions>,
    ) -> Result<Retained<ProtocolObject<dyn MTLLibrary>>, Retained<NSError>> {
        let source = NSString::from_str(source);
        unsafe { msg_send![self, newLibraryWithSource: &*source, options: options, error: _] }
    }

    fn new_library_with_stitched_descriptor(
        &self,
        descriptor: &MTLStitchedLibraryDescriptor,
    ) -> Result<Retained<ProtocolObject<dyn MTLLibrary>>, Retained<NSError>> {
        unsafe { msg_send![self, newLibraryWithStitchedDescriptor: descriptor, error: _] }
    }

    fn new_render_pipeline_state_with_descriptor(
        &self,
        descriptor: &MTLRenderPipelineDescriptor,
    ) -> Result<Retained<ProtocolObject<dyn MTLRenderPipelineState>>, Retained<NSError>> {
        unsafe { msg_send![self, newRenderPipelineStateWithDescriptor: descriptor, error: _] }
    }

    fn new_compute_pipeline_state_with_function(
        &self,
        function: &ProtocolObject<dyn MTLFunction>,
    ) -> Result<Retained<ProtocolObject<dyn MTLComputePipelineState>>, Retained<NSError>> {
        unsafe { msg_send![self, newComputePipelineStateWithFunction: function, error: _] }
    }

    fn new_compute_pipeline_state_with_descriptor(
        &self,
        descriptor: &MTLComputePipelineDescriptor,
        options: MTLPipelineOption,
    ) -> Result<
        (Retained<ProtocolObject<dyn MTLComputePipelineState>>, Option<Retained<MTLComputePipelineReflection>>),
        Retained<NSError>,
    > {
        let mut reflection: *mut MTLComputePipelineReflection = std::ptr::null_mut();
        let mut error: *mut NSError = std::ptr::null_mut();
        let result: Option<Retained<ProtocolObject<dyn MTLComputePipelineState>>> = unsafe {
            msg_send![
                self,
                newComputePipelineStateWithDescriptor: descriptor,
                options: options,
                reflection: &mut reflection,
                error: &mut error
            ]
        };
        match result {
            Some(pso) => {
                let reflection_obj = if reflection.is_null() {
                    None
                } else {
                    unsafe { Retained::retain(reflection) }
                };
                Ok((pso, reflection_obj))
            },
            None => Err(unsafe { Retained::retain(error).unwrap() }),
        }
    }

    fn new_fence(&self) -> Option<Retained<ProtocolObject<dyn MTLFence>>> {
        unsafe { msg_send![self, newFence] }
    }

    fn supports_feature_set(&self, feature_set: MTLFeatureSet) -> bool {
        unsafe { msg_send![self, supportsFeatureSet: feature_set] }
    }

    fn new_argument_encoder_with_arguments(
        &self,
        arguments: &[&MTLArgumentDescriptor],
    ) -> Option<Retained<ProtocolObject<dyn MTLArgumentEncoder>>> {
        let arguments = NSArray::from_slice(arguments);
        unsafe { msg_send![self, newArgumentEncoderWithArguments: &*arguments] }
    }

    fn new_event(&self) -> Option<Retained<ProtocolObject<dyn MTLEvent>>> {
        unsafe { msg_send![self, newEvent] }
    }

    fn new_shared_event(&self) -> Option<Retained<ProtocolObject<dyn MTLSharedEvent>>> {
        unsafe { msg_send![self, newSharedEvent] }
    }

    fn new_shared_event_with_handle(
        &self,
        handle: &MTLSharedEventHandle,
    ) -> Option<Retained<ProtocolObject<dyn MTLSharedEvent>>> {
        unsafe { msg_send![self, newSharedEventWithHandle: handle] }
    }

    fn new_io_command_queue_with_descriptor(
        &self,
        descriptor: &MTLIOCommandQueueDescriptor,
    ) -> Result<Retained<ProtocolObject<dyn MTLIOCommandQueue>>, Retained<NSError>> {
        unsafe { msg_send![self, newIOCommandQueueWithDescriptor: descriptor, error: _] }
    }

    fn new_dynamic_library(
        &self,
        library: &ProtocolObject<dyn MTLLibrary>,
    ) -> Result<Retained<ProtocolObject<dyn MTLDynamicLibrary>>, Retained<NSError>> {
        unsafe { msg_send![self, newDynamicLibrary: library, error: _] }
    }

    fn new_dynamic_library_with_path(
        &self,
        path: &Path,
    ) -> Result<Retained<ProtocolObject<dyn MTLDynamicLibrary>>, Retained<NSError>> {
        let url = NSURL::from_file_path(path).expect("path must be a valid file URL path");
        unsafe { msg_send![self, newDynamicLibraryWithURL: &*url, error: _] }
    }

    fn new_acceleration_structure_with_size(
        &self,
        size: usize,
    ) -> Option<Retained<ProtocolObject<dyn MTLAccelerationStructure>>> {
        unsafe { msg_send![self, newAccelerationStructureWithSize: size] }
    }

    fn new_acceleration_structure_with_descriptor(
        &self,
        descriptor: &MTLAccelerationStructureDescriptor,
    ) -> Option<Retained<ProtocolObject<dyn MTLAccelerationStructure>>> {
        unsafe { msg_send![self, newAccelerationStructureWithDescriptor: descriptor] }
    }

    fn new_residency_set_with_descriptor(
        &self,
        descriptor: &MTLResidencySetDescriptor,
    ) -> Result<Retained<ProtocolObject<dyn MTLResidencySet>>, Retained<NSError>> {
        unsafe { msg_send![self, newResidencySetWithDescriptor: descriptor, error: _] }
    }

    fn tensor_size_and_align_with_descriptor(
        &self,
        descriptor: &MTLTensorDescriptor,
    ) -> MTLSizeAndAlign {
        unsafe { msg_send![self, tensorSizeAndAlignWithDescriptor: descriptor] }
    }

    fn new_tensor_with_descriptor(
        &self,
        descriptor: &MTLTensorDescriptor,
    ) -> Result<Retained<ProtocolObject<dyn MTLTensor>>, Retained<NSError>> {
        unsafe { msg_send![self, newTensorWithDescriptor: descriptor, error: _] }
    }

    fn function_handle_with_function(
        &self,
        function: &ProtocolObject<dyn MTLFunction>,
    ) -> Option<Retained<ProtocolObject<dyn MTLFunctionHandle>>> {
        unsafe { msg_send![self, functionHandleWithFunction: function] }
    }

    fn new_command_allocator(&self) -> Option<Retained<ProtocolObject<dyn crate::MTL4CommandAllocator>>> {
        unsafe { msg_send![self, newCommandAllocator] }
    }

    fn new_command_allocator_with_descriptor(
        &self,
        descriptor: &MTL4CommandAllocatorDescriptor,
    ) -> Result<Retained<ProtocolObject<dyn crate::MTL4CommandAllocator>>, Retained<NSError>> {
        unsafe { msg_send![self, newCommandAllocatorWithDescriptor: descriptor, error: _] }
    }

    fn new_mtl4_command_queue(&self) -> Option<Retained<ProtocolObject<dyn crate::MTL4CommandQueue>>> {
        unsafe { msg_send![self, newMTL4CommandQueue] }
    }

    fn new_mtl4_command_queue_with_descriptor(
        &self,
        descriptor: &MTL4CommandQueueDescriptor,
    ) -> Result<Retained<ProtocolObject<dyn crate::MTL4CommandQueue>>, Retained<NSError>> {
        unsafe { msg_send![self, newMTL4CommandQueueWithDescriptor: descriptor, error: _] }
    }

    fn new_mtl4_command_buffer(&self) -> Option<Retained<ProtocolObject<dyn crate::MTL4CommandBuffer>>> {
        unsafe { msg_send![self, newCommandBuffer] }
    }

    fn new_argument_table_with_descriptor(
        &self,
        descriptor: &MTL4ArgumentTableDescriptor,
    ) -> Result<Retained<ProtocolObject<dyn MTL4ArgumentTable>>, Retained<NSError>> {
        unsafe { msg_send![self, newArgumentTableWithDescriptor: descriptor, error: _] }
    }

    fn new_texture_view_pool_with_descriptor(
        &self,
        descriptor: &MTLResourceViewPoolDescriptor,
    ) -> Result<Retained<ProtocolObject<dyn MTLTextureViewPool>>, Retained<NSError>> {
        unsafe { msg_send![self, newTextureViewPoolWithDescriptor: descriptor, error: _] }
    }

    fn new_compiler_with_descriptor(
        &self,
        descriptor: &MTL4CompilerDescriptor,
    ) -> Result<Retained<ProtocolObject<dyn crate::MTL4Compiler>>, Retained<NSError>> {
        unsafe { msg_send![self, newCompilerWithDescriptor: descriptor, error: _] }
    }

    fn new_archive_with_url(
        &self,
        url: &NSURL,
    ) -> Result<Retained<ProtocolObject<dyn MTL4Archive>>, Retained<NSError>> {
        unsafe { msg_send![self, newArchiveWithURL: url, error: _] }
    }

    fn new_pipeline_data_set_serializer_with_descriptor(
        &self,
        descriptor: &MTL4PipelineDataSetSerializerDescriptor,
    ) -> Retained<ProtocolObject<dyn MTL4PipelineDataSetSerializer>> {
        unsafe { msg_send![self, newPipelineDataSetSerializerWithDescriptor: descriptor] }
    }

    fn new_buffer_with_length_options_placement_sparse_page_size(
        &self,
        length: usize,
        options: MTLResourceOptions,
        placement_sparse_page_size: MTLSparsePageSize,
    ) -> Option<Retained<ProtocolObject<dyn MTLBuffer>>> {
        unsafe {
            msg_send![
                self,
                newBufferWithLength: length,
                options: options,
                placementSparsePageSize: placement_sparse_page_size
            ]
        }
    }

    fn new_counter_heap_with_descriptor(
        &self,
        descriptor: &MTL4CounterHeapDescriptor,
    ) -> Result<Retained<ProtocolObject<dyn MTL4CounterHeap>>, Retained<NSError>> {
        unsafe { msg_send![self, newCounterHeapWithDescriptor: descriptor, error: _] }
    }

    fn size_of_counter_heap_entry(&self, counter_heap_type: MTL4CounterHeapType) -> usize {
        unsafe { msg_send![self, sizeOfCounterHeapEntry: counter_heap_type] }
    }

    fn query_timestamp_frequency(&self) -> u64 {
        unsafe { msg_send![self, queryTimestampFrequency] }
    }

    fn function_handle_with_binary_function(
        &self,
        function: &ProtocolObject<dyn MTL4BinaryFunction>,
    ) -> Option<Retained<ProtocolObject<dyn MTLFunctionHandle>>> {
        unsafe { msg_send![self, functionHandleWithBinaryFunction: function] }
    }
}

impl dyn MTLDevice {
    /// Returns a reference to the preferred system default Metal device.
    #[inline]
    pub fn system_default() -> Option<Retained<ProtocolObject<dyn MTLDevice>>> {
        unsafe extern "C" {
            fn MTLCreateSystemDefaultDevice() -> *mut ProtocolObject<dyn MTLDevice>;
        }
        let ret = unsafe { MTLCreateSystemDefaultDevice() };
        unsafe { Retained::from_raw(ret) }
    }
}
