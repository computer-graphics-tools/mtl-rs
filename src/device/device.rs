use objc2::{Message, extern_protocol, msg_send, rc::Retained, runtime::ProtocolObject};
use objc2_foundation::{NSArray, NSError, NSObjectProtocol, NSString};

use super::MTLArchitecture;
use crate::{
    MTLArgumentBuffersTier, MTLCounterSampleBuffer, MTLCounterSampleBufferDescriptor,
    MTLCounterSamplingPoint, MTLCounterSet, MTLDepthStencilDescriptor, MTLDepthStencilState,
    MTLDeviceLocation, MTLGPUFamily, MTLPixelFormat, MTLReadWriteTextureTier, MTLSize,
};

extern_protocol!(
    /// A Metal device that represents a GPU.
    ///
    /// See Apple's documentation for `MTLDevice` for details.
    pub unsafe trait MTLDevice: NSObjectProtocol + Send + Sync {
        /// Returns the IORegistry ID for the Metal device.
        #[unsafe(method(registryID))]
        #[unsafe(method_family = none)]
        fn registry_id(&self) -> u64;

        /// The device's architecture information.
        #[unsafe(method(architecture))]
        #[unsafe(method_family = none)]
        unsafe fn architecture(&self) -> Retained<MTLArchitecture>;

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
        unsafe fn new_depth_stencil_state_with_descriptor(
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
        unsafe fn sample_timestamps_gpu_timestamp(
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

        /// Deprecated in favor of `supportsShaderBarycentricCoordinates`.
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
}

/// Returns a reference to the preferred system default Metal device.
#[inline]
pub extern "C-unwind" fn create_system_default_device()
-> Option<Retained<ProtocolObject<dyn MTLDevice>>> {
    unsafe extern "C-unwind" {
        fn mtl_create_system_default_device() -> *mut ProtocolObject<dyn MTLDevice>;
    }
    let ret = unsafe { mtl_create_system_default_device() };
    unsafe { Retained::from_raw(ret) }
}
