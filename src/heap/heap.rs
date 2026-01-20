use objc2::{Message, extern_protocol, msg_send, rc::Retained, runtime::ProtocolObject};
use objc2_foundation::NSString;

use crate::{
    MTLAccelerationStructure, MTLAccelerationStructureDescriptor, MTLAllocation, MTLBuffer,
    MTLCPUCacheMode, MTLHazardTrackingMode, MTLPurgeableState, MTLResourceOptions, MTLStorageMode,
    MTLTexture, MTLTextureDescriptor,
};

extern_protocol!(
    /// Apple's documentation: `https://developer.apple.com/documentation/metal/mtlheap?language=objc`
    ///
    /// Availability: macOS 10.13+, iOS 10.0+
    pub unsafe trait MTLHeap: MTLAllocation {
        /// The device this heap was created against. This heap can only be used with this device.
        #[unsafe(method(device))]
        #[unsafe(method_family = none)]
        fn device(&self) -> Retained<ProtocolObject<dyn crate::MTLDevice>>;

        /// Current heap storage mode, default is StorageMode::Private.
        #[unsafe(method(storageMode))]
        #[unsafe(method_family = none)]
        fn storage_mode(&self) -> MTLStorageMode;

        /// CPU cache mode for the heap. Default is CpuCacheMode::DefaultCache.
        #[unsafe(method(cpuCacheMode))]
        #[unsafe(method_family = none)]
        fn cpu_cache_mode(&self) -> MTLCPUCacheMode;

        /// Whether or not the heap is hazard tracked.
        #[unsafe(method(hazardTrackingMode))]
        #[unsafe(method_family = none)]
        fn hazard_tracking_mode(&self) -> MTLHazardTrackingMode;

        /// A packed tuple of the storageMode, cpuCacheMode and hazardTrackingMode properties.
        #[unsafe(method(resourceOptions))]
        #[unsafe(method_family = none)]
        fn resource_options(&self) -> MTLResourceOptions;

        /// Heap size in bytes, specified at creation time and rounded up to device specific alignment.
        #[unsafe(method(size))]
        #[unsafe(method_family = none)]
        fn size(&self) -> usize;

        /// The size in bytes, of all resources allocated from the heap.
        #[unsafe(method(usedSize))]
        #[unsafe(method_family = none)]
        fn used_size(&self) -> usize;

        /// The size in bytes of the current heap allocation.
        ///
        /// Availability: macOS 10.13+, iOS 11.0+
        #[unsafe(method(currentAllocatedSize))]
        #[unsafe(method_family = none)]
        fn current_allocated_size(&self) -> usize;

        /// The maximum size that can be successfully allocated from the heap in bytes, taking alignment into account.
        #[unsafe(method(maxAvailableSizeWithAlignment:))]
        #[unsafe(method_family = none)]
        fn max_available_size_with_alignment(&self, alignment: usize) -> usize;

        /// Create a new buffer backed by heap memory.
        /// Returns: The buffer or None if heap is full.
        #[unsafe(method(newBufferWithLength:options:))]
        #[unsafe(method_family = new)]
        fn new_buffer(
            &self,
            length: usize,
            options: MTLResourceOptions,
        ) -> Option<Retained<ProtocolObject<dyn MTLBuffer>>>;

        /// Create a new texture backed by heap memory.
        /// Returns: The texture or None if heap is full.
        #[unsafe(method(newTextureWithDescriptor:))]
        #[unsafe(method_family = new)]
        fn new_texture(
            &self,
            descriptor: &MTLTextureDescriptor,
        ) -> Option<Retained<ProtocolObject<dyn MTLTexture>>>;

        /// Set or query the purgeability state of the heap.
        #[unsafe(method(setPurgeableState:))]
        #[unsafe(method_family = none)]
        fn set_purgeable_state(&self, state: MTLPurgeableState) -> MTLPurgeableState;

        /// The type of the heap. The default value is HeapType::Automatic.
        #[unsafe(method(type))]
        #[unsafe(method_family = none)]
        unsafe fn r#type(&self) -> super::MTLHeapType;

        /// Create a new buffer backed by heap memory at the specified placement offset.
        #[unsafe(method(newBufferWithLength:options:offset:))]
        #[unsafe(method_family = new)]
        unsafe fn new_buffer_with_offset(
            &self,
            length: usize,
            options: MTLResourceOptions,
            offset: usize,
        ) -> Option<Retained<ProtocolObject<dyn MTLBuffer>>>;

        /// Create a new texture backed by heap memory at the specified placement offset.
        #[unsafe(method(newTextureWithDescriptor:offset:))]
        #[unsafe(method_family = new)]
        unsafe fn new_texture_with_offset(
            &self,
            descriptor: &MTLTextureDescriptor,
            offset: usize,
        ) -> Option<Retained<ProtocolObject<dyn MTLTexture>>>;

        /// Create a new acceleration structure backed by heap memory.
        #[unsafe(method(newAccelerationStructureWithSize:))]
        #[unsafe(method_family = new)]
        unsafe fn new_acceleration_structure_with_size(
            &self,
            size: usize,
        ) -> Option<Retained<ProtocolObject<dyn MTLAccelerationStructure>>>;

        /// Create a new acceleration structure backed by heap memory.
        /// This is a convenience method which creates the acceleration structure backed by heap memory.
        #[unsafe(method(newAccelerationStructureWithDescriptor:))]
        #[unsafe(method_family = new)]
        unsafe fn new_acceleration_structure_with_descriptor(
            &self,
            descriptor: &MTLAccelerationStructureDescriptor,
        ) -> Option<Retained<ProtocolObject<dyn MTLAccelerationStructure>>>;

        /// Create a new acceleration structure backed by heap memory at the specified placement offset.
        #[unsafe(method(newAccelerationStructureWithSize:offset:))]
        #[unsafe(method_family = new)]
        unsafe fn new_acceleration_structure_with_size_offset(
            &self,
            size: usize,
            offset: usize,
        ) -> Option<Retained<ProtocolObject<dyn MTLAccelerationStructure>>>;

        /// Create a new acceleration structure backed by heap memory at the specified placement offset.
        /// This is a convenience method which computes the acceleration structure size based on the descriptor.
        #[unsafe(method(newAccelerationStructureWithDescriptor:offset:))]
        #[unsafe(method_family = new)]
        unsafe fn new_acceleration_structure_with_descriptor_offset(
            &self,
            descriptor: &MTLAccelerationStructureDescriptor,
            offset: usize,
        ) -> Option<Retained<ProtocolObject<dyn MTLAccelerationStructure>>>;
    }
);

#[allow(unused)]
pub trait MTLHeapExt: MTLHeap + Message {
    fn label(&self) -> Option<String>;
    fn set_label(&self, label: Option<&str>);
}

impl MTLHeapExt for ProtocolObject<dyn MTLHeap> {
    fn label(&self) -> Option<String> {
        let label: Option<Retained<NSString>> = unsafe { msg_send![self, label] };
        label.map(|s| s.to_string())
    }

    fn set_label(&self, label: Option<&str>) {
        unsafe {
            let _: () = msg_send![self, setLabel: label.map(NSString::from_str).as_deref()];
        }
    }
}
