use std::{ffi::c_void, ptr::NonNull};

use dispatch2::DispatchData;
use objc2::{Message, extern_protocol, msg_send, rc::Retained, runtime::ProtocolObject};
use objc2_foundation::{NSArray, NSError, NSObjectProtocol, NSString};

use super::MTLArchitecture;
use crate::{
    MTLAccelerationStructure, MTLArgumentBuffersTier, MTLArgumentEncoder, MTLBuffer,
    MTLCommandQueue, MTLComputePipelineDescriptor, MTLComputePipelineState, MTLCounterSampleBuffer,
    MTLCounterSampleBufferDescriptor, MTLCounterSamplingPoint, MTLCounterSet,
    MTLDepthStencilDescriptor, MTLDepthStencilState, MTLDeviceLocation, MTLDynamicLibrary,
    MTLEvent, MTLFeatureSet, MTLFence, MTLFunction, MTLGPUFamily, MTLHeap, MTLHeapDescriptor,
    MTLIOCommandQueue, MTLIOCommandQueueDescriptor, MTLLibrary, MTLLogState, MTLLogStateDescriptor,
    MTLPipelineOption, MTLPixelFormat, MTLReadWriteTextureTier, MTLResourceOptions,
    MTLSamplerDescriptor, MTLSamplerState, MTLSharedEvent, MTLSharedEventHandle, MTLSize,
    MTLTexture, MTLTextureDescriptor, acceleration_structure::MTLAccelerationStructureDescriptor,
    argument::MTLArgumentDescriptor, compute_pipeline::MTLComputePipelineReflection,
    function_stitching::MTLStitchedLibraryDescriptor,
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
        fn supports_family(&self, family: MTLGPUFamily) -> bool;

        /// Whether the device supports textures with a given sample count.
        #[unsafe(method(supportsTextureSampleCount:))]
        #[unsafe(method_family = none)]
        fn supports_texture_sample_count(&self, sample_count: usize) -> bool;

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
        fn supports_counter_sampling(&self, sampling_point: MTLCounterSamplingPoint) -> bool;

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
    /// The full name of the vendor device.
    fn name(&self) -> String;

    /// Array of the Counter Sets exposed by the device, or None.
    fn counter_sets(&self) -> Option<Box<[Retained<ProtocolObject<dyn MTLCounterSet>>]>>;

    /// Create a buffer by allocating new memory.
    fn new_buffer(
        &self,
        length: usize,
        options: MTLResourceOptions,
    ) -> Option<Retained<ProtocolObject<dyn MTLBuffer>>>;

    /// Create a buffer by copying data from a byte slice.
    fn new_buffer_with_data(
        &self,
        data: &[u8],
        options: MTLResourceOptions,
    ) -> Option<Retained<ProtocolObject<dyn MTLBuffer>>>;

    /// Create a buffer wrapping existing memory without copying.
    /// Safety: The pointer must remain valid for the lifetime of the buffer.
    fn new_buffer_with_bytes_no_copy(
        &self,
        ptr: NonNull<c_void>,
        length: usize,
        options: MTLResourceOptions,
    ) -> Option<Retained<ProtocolObject<dyn MTLBuffer>>>;

    /// Create a new command queue with default max command buffer count (64).
    fn new_command_queue(&self) -> Option<Retained<ProtocolObject<dyn MTLCommandQueue>>>;

    /// Create a new command queue with specified max command buffer count.
    fn new_command_queue_with_max_command_buffer_count(
        &self,
        max_count: usize,
    ) -> Option<Retained<ProtocolObject<dyn MTLCommandQueue>>>;

    /// Get the default library from the main bundle.
    fn new_default_library(&self) -> Option<Retained<ProtocolObject<dyn MTLLibrary>>>;

    /// Load a library from compiled metallib data.
    fn new_library_with_data(
        &self,
        data: &[u8],
    ) -> Result<Retained<ProtocolObject<dyn MTLLibrary>>, Retained<NSError>>;

    /// Load a library from a file URL.
    fn new_library_with_url(
        &self,
        url: &objc2_foundation::NSURL,
    ) -> Result<Retained<ProtocolObject<dyn MTLLibrary>>, Retained<NSError>>;

    /// Create a compute pipeline state from a function.
    fn new_compute_pipeline_state_with_function(
        &self,
        function: &ProtocolObject<dyn MTLFunction>,
    ) -> Result<Retained<ProtocolObject<dyn MTLComputePipelineState>>, Retained<NSError>>;

    /// Create a compute pipeline state from a descriptor with reflection.
    fn new_compute_pipeline_state_with_descriptor(
        &self,
        descriptor: &MTLComputePipelineDescriptor,
        options: MTLPipelineOption,
    ) -> Result<
        (
            Retained<ProtocolObject<dyn MTLComputePipelineState>>,
            Option<Retained<MTLComputePipelineReflection>>,
        ),
        Retained<NSError>,
    >;

    /// Create a new heap.
    fn new_heap_with_descriptor(
        &self,
        descriptor: &MTLHeapDescriptor,
    ) -> Option<Retained<ProtocolObject<dyn MTLHeap>>>;

    /// Create a new texture.
    fn new_texture_with_descriptor(
        &self,
        descriptor: &MTLTextureDescriptor,
    ) -> Option<Retained<ProtocolObject<dyn MTLTexture>>>;

    /// Create a new sampler state.
    fn new_sampler_state_with_descriptor(
        &self,
        descriptor: &MTLSamplerDescriptor,
    ) -> Option<Retained<ProtocolObject<dyn MTLSamplerState>>>;

    /// Create a new event for GPU synchronization.
    fn new_event(&self) -> Option<Retained<ProtocolObject<dyn MTLEvent>>>;

    /// Create a new fence for resource tracking.
    fn new_fence(&self) -> Option<Retained<ProtocolObject<dyn MTLFence>>>;

    /// The maximum threadgroup memory available, in bytes.
    fn max_threadgroup_memory_length(&self) -> usize;

    /// The maximum buffer length supported by this device.
    fn max_buffer_length(&self) -> usize;

    /// Whether raster order groups are supported.
    fn raster_order_groups_supported(&self) -> bool;

    /// Whether the device supports a specific feature set (deprecated, use supports_family).
    fn supports_feature_set(&self, feature_set: MTLFeatureSet) -> bool;

    /// Creates a new shareable event.
    fn new_shared_event(&self) -> Option<Retained<ProtocolObject<dyn MTLSharedEvent>>>;

    /// Creates a shareable event from a shared event handle.
    fn new_shared_event_with_handle(
        &self,
        handle: &MTLSharedEventHandle,
    ) -> Option<Retained<ProtocolObject<dyn MTLSharedEvent>>>;

    /// Creates a dynamic library from a library.
    fn new_dynamic_library(
        &self,
        library: &ProtocolObject<dyn MTLLibrary>,
    ) -> Result<Retained<ProtocolObject<dyn MTLDynamicLibrary>>, Retained<NSError>>;

    /// Creates a dynamic library from a URL.
    fn new_dynamic_library_with_url(
        &self,
        url: &objc2_foundation::NSURL,
    ) -> Result<Retained<ProtocolObject<dyn MTLDynamicLibrary>>, Retained<NSError>>;

    /// Creates a new I/O command queue.
    fn new_io_command_queue_with_descriptor(
        &self,
        descriptor: &MTLIOCommandQueueDescriptor,
    ) -> Result<Retained<ProtocolObject<dyn MTLIOCommandQueue>>, Retained<NSError>>;

    /// Creates a new acceleration structure with the specified size.
    fn new_acceleration_structure_with_size(
        &self,
        size: usize,
    ) -> Option<Retained<ProtocolObject<dyn MTLAccelerationStructure>>>;

    /// Creates a new acceleration structure with the specified descriptor.
    fn new_acceleration_structure_with_descriptor(
        &self,
        descriptor: &MTLAccelerationStructureDescriptor,
    ) -> Option<Retained<ProtocolObject<dyn MTLAccelerationStructure>>>;

    /// Creates an argument encoder with the specified arguments.
    fn new_argument_encoder_with_arguments(
        &self,
        arguments: &NSArray<MTLArgumentDescriptor>,
    ) -> Option<Retained<ProtocolObject<dyn MTLArgumentEncoder>>>;

    /// Creates a new log state with the specified descriptor.
    fn new_log_state_with_descriptor(
        &self,
        descriptor: &MTLLogStateDescriptor,
    ) -> Result<Retained<ProtocolObject<dyn MTLLogState>>, Retained<NSError>>;

    /// Creates a new library with the specified stitched library descriptor.
    fn new_library_with_stitched_descriptor(
        &self,
        descriptor: &MTLStitchedLibraryDescriptor,
    ) -> Result<Retained<ProtocolObject<dyn MTLLibrary>>, Retained<NSError>>;
}

impl MTLDeviceExt for ProtocolObject<dyn MTLDevice> {
    fn name(&self) -> String {
        let ns: Retained<NSString> = unsafe { msg_send![self, name] };
        ns.to_string()
    }

    fn counter_sets(&self) -> Option<Box<[Retained<ProtocolObject<dyn MTLCounterSet>>]>> {
        let array: Option<Retained<NSArray<ProtocolObject<dyn MTLCounterSet>>>> =
            unsafe { msg_send![self, counterSets] };
        array.map(|a| a.to_vec().into_boxed_slice())
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
        // Pass null for deallocator - caller is responsible for memory management
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

    fn new_command_queue(&self) -> Option<Retained<ProtocolObject<dyn MTLCommandQueue>>> {
        unsafe { msg_send![self, newCommandQueue] }
    }

    fn new_command_queue_with_max_command_buffer_count(
        &self,
        max_count: usize,
    ) -> Option<Retained<ProtocolObject<dyn MTLCommandQueue>>> {
        unsafe { msg_send![self, newCommandQueueWithMaxCommandBufferCount: max_count] }
    }

    fn new_default_library(&self) -> Option<Retained<ProtocolObject<dyn MTLLibrary>>> {
        unsafe { msg_send![self, newDefaultLibrary] }
    }

    fn new_library_with_data(
        &self,
        data: &[u8],
    ) -> Result<Retained<ProtocolObject<dyn MTLLibrary>>, Retained<NSError>> {
        let dispatch_data = DispatchData::from_bytes(data);

        let mut error: *mut NSError = std::ptr::null_mut();
        let result: Option<Retained<ProtocolObject<dyn MTLLibrary>>> =
            unsafe { msg_send![self, newLibraryWithData: &*dispatch_data, error: &mut error] };

        match result {
            Some(lib) => Ok(lib),
            None => Err(unsafe { Retained::retain(error).unwrap() }),
        }
    }

    fn new_library_with_url(
        &self,
        url: &objc2_foundation::NSURL,
    ) -> Result<Retained<ProtocolObject<dyn MTLLibrary>>, Retained<NSError>> {
        let mut error: *mut NSError = std::ptr::null_mut();
        let result: Option<Retained<ProtocolObject<dyn MTLLibrary>>> =
            unsafe { msg_send![self, newLibraryWithURL: url, error: &mut error] };

        match result {
            Some(lib) => Ok(lib),
            None => Err(unsafe { Retained::retain(error).unwrap() }),
        }
    }

    fn new_compute_pipeline_state_with_function(
        &self,
        function: &ProtocolObject<dyn MTLFunction>,
    ) -> Result<Retained<ProtocolObject<dyn MTLComputePipelineState>>, Retained<NSError>> {
        let mut error: *mut NSError = std::ptr::null_mut();
        let result: Option<Retained<ProtocolObject<dyn MTLComputePipelineState>>> = unsafe {
            msg_send![self, newComputePipelineStateWithFunction: function, error: &mut error]
        };

        match result {
            Some(pso) => Ok(pso),
            None => Err(unsafe { Retained::retain(error).unwrap() }),
        }
    }

    fn new_compute_pipeline_state_with_descriptor(
        &self,
        descriptor: &MTLComputePipelineDescriptor,
        options: MTLPipelineOption,
    ) -> Result<
        (
            Retained<ProtocolObject<dyn MTLComputePipelineState>>,
            Option<Retained<MTLComputePipelineReflection>>,
        ),
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
            }
            None => Err(unsafe { Retained::retain(error).unwrap() }),
        }
    }

    fn new_heap_with_descriptor(
        &self,
        descriptor: &MTLHeapDescriptor,
    ) -> Option<Retained<ProtocolObject<dyn MTLHeap>>> {
        unsafe { msg_send![self, newHeapWithDescriptor: descriptor] }
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

    fn new_event(&self) -> Option<Retained<ProtocolObject<dyn MTLEvent>>> {
        unsafe { msg_send![self, newEvent] }
    }

    fn new_fence(&self) -> Option<Retained<ProtocolObject<dyn MTLFence>>> {
        unsafe { msg_send![self, newFence] }
    }

    fn max_threadgroup_memory_length(&self) -> usize {
        unsafe { msg_send![self, maxThreadgroupMemoryLength] }
    }

    fn max_buffer_length(&self) -> usize {
        unsafe { msg_send![self, maxBufferLength] }
    }

    fn raster_order_groups_supported(&self) -> bool {
        unsafe { msg_send![self, areRasterOrderGroupsSupported] }
    }

    fn supports_feature_set(&self, feature_set: MTLFeatureSet) -> bool {
        unsafe { msg_send![self, supportsFeatureSet: feature_set] }
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

    fn new_dynamic_library(
        &self,
        library: &ProtocolObject<dyn MTLLibrary>,
    ) -> Result<Retained<ProtocolObject<dyn MTLDynamicLibrary>>, Retained<NSError>> {
        let mut error: *mut NSError = std::ptr::null_mut();
        let result: Option<Retained<ProtocolObject<dyn MTLDynamicLibrary>>> =
            unsafe { msg_send![self, newDynamicLibrary: library, error: &mut error] };
        match result {
            Some(lib) => Ok(lib),
            None => Err(unsafe { Retained::retain(error).unwrap() }),
        }
    }

    fn new_dynamic_library_with_url(
        &self,
        url: &objc2_foundation::NSURL,
    ) -> Result<Retained<ProtocolObject<dyn MTLDynamicLibrary>>, Retained<NSError>> {
        let mut error: *mut NSError = std::ptr::null_mut();
        let result: Option<Retained<ProtocolObject<dyn MTLDynamicLibrary>>> =
            unsafe { msg_send![self, newDynamicLibraryWithURL: url, error: &mut error] };
        match result {
            Some(lib) => Ok(lib),
            None => Err(unsafe { Retained::retain(error).unwrap() }),
        }
    }

    fn new_io_command_queue_with_descriptor(
        &self,
        descriptor: &MTLIOCommandQueueDescriptor,
    ) -> Result<Retained<ProtocolObject<dyn MTLIOCommandQueue>>, Retained<NSError>> {
        let mut error: *mut NSError = std::ptr::null_mut();
        let result: Option<Retained<ProtocolObject<dyn MTLIOCommandQueue>>> = unsafe {
            msg_send![self, newIOCommandQueueWithDescriptor: descriptor, error: &mut error]
        };
        match result {
            Some(queue) => Ok(queue),
            None => Err(unsafe { Retained::retain(error).unwrap() }),
        }
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

    fn new_argument_encoder_with_arguments(
        &self,
        arguments: &NSArray<MTLArgumentDescriptor>,
    ) -> Option<Retained<ProtocolObject<dyn MTLArgumentEncoder>>> {
        unsafe { msg_send![self, newArgumentEncoderWithArguments: arguments] }
    }

    fn new_log_state_with_descriptor(
        &self,
        descriptor: &MTLLogStateDescriptor,
    ) -> Result<Retained<ProtocolObject<dyn MTLLogState>>, Retained<NSError>> {
        let mut error: *mut NSError = std::ptr::null_mut();
        let result: Option<Retained<ProtocolObject<dyn MTLLogState>>> =
            unsafe { msg_send![self, newLogStateWithDescriptor: descriptor, error: &mut error] };
        match result {
            Some(log) => Ok(log),
            None => Err(unsafe { Retained::retain(error).unwrap() }),
        }
    }

    fn new_library_with_stitched_descriptor(
        &self,
        descriptor: &MTLStitchedLibraryDescriptor,
    ) -> Result<Retained<ProtocolObject<dyn MTLLibrary>>, Retained<NSError>> {
        let mut error: *mut NSError = std::ptr::null_mut();
        let result: Option<Retained<ProtocolObject<dyn MTLLibrary>>> = unsafe {
            msg_send![self, newLibraryWithStitchedDescriptor: descriptor, error: &mut error]
        };
        match result {
            Some(lib) => Ok(lib),
            None => Err(unsafe { Retained::retain(error).unwrap() }),
        }
    }
}

/// Returns a reference to the preferred system default Metal device.
#[inline]
pub fn create_system_default_device() -> Option<Retained<ProtocolObject<dyn MTLDevice>>> {
    unsafe extern "C" {
        fn MTLCreateSystemDefaultDevice() -> *mut ProtocolObject<dyn MTLDevice>;
    }
    let ret = unsafe { MTLCreateSystemDefaultDevice() };
    unsafe { Retained::from_raw(ret) }
}

impl dyn MTLDevice {
    /// Returns a reference to the preferred system default Metal device.
    ///
    /// This is an alias for `create_system_default_device`.
    #[inline]
    pub fn system_default() -> Option<Retained<ProtocolObject<dyn MTLDevice>>> {
        create_system_default_device()
    }
}
