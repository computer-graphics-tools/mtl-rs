use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
    runtime::NSObject,
};
use objc2_foundation::{CopyingHelper, NSCopying, NSObjectProtocol};

use super::MTLHeapType;
use crate::{
    MTLCPUCacheMode, MTLHazardTrackingMode, MTLResourceOptions, MTLSparsePageSize, MTLStorageMode,
};

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlheapdescriptor?language=objc)
    ///
    /// Availability: macOS 10.13+, iOS 10.0+
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLHeapDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for MTLHeapDescriptor {}
);

unsafe impl CopyingHelper for MTLHeapDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLHeapDescriptor {}
);

impl MTLHeapDescriptor {
    extern_methods!(
        /// Requested size of the heap's backing memory.
        ///
        /// The size may be rounded up to GPU page granularity.
        #[unsafe(method(size))]
        #[unsafe(method_family = none)]
        pub fn size(&self) -> usize;

        /// Setter for [`size`][Self::size].
        #[unsafe(method(setSize:))]
        #[unsafe(method_family = none)]
        pub fn set_size(&self, size: usize);

        /// Storage mode for the heap. Default is `StorageMode::Private`.
        ///
        /// All resources created from this heap share the same storage mode.
        /// MTLStorageModeManaged and MTLStorageModeMemoryless are disallowed.
        #[unsafe(method(storageMode))]
        #[unsafe(method_family = none)]
        pub fn storage_mode(&self) -> MTLStorageMode;

        /// Setter for [`storageMode`][Self::storageMode].
        #[unsafe(method(setStorageMode:))]
        #[unsafe(method_family = none)]
        pub fn set_storage_mode(&self, storage_mode: MTLStorageMode);

        /// CPU cache mode for the heap. Default is `CpuCacheMode::DefaultCache`.
        ///
        /// All resources created from this heap share the same cache mode.
        /// CPU cache mode is ignored for `StorageMode::Private`.
        #[unsafe(method(cpuCacheMode))]
        #[unsafe(method_family = none)]
        pub fn cpu_cache_mode(&self) -> MTLCPUCacheMode;

        /// Setter for [`cpuCacheMode`][Self::cpuCacheMode].
        #[unsafe(method(setCpuCacheMode:))]
        #[unsafe(method_family = none)]
        pub fn set_cpu_cache_mode(&self, cpu_cache_mode: MTLCPUCacheMode);

        /// The sparse page size to use for resources created from the heap.
        ///
        /// Availability: macOS 13.0+, iOS 16.0+
        #[unsafe(method(sparsePageSize))]
        #[unsafe(method_family = none)]
        pub unsafe fn sparse_page_size(&self) -> MTLSparsePageSize;

        /// Setter for [`sparsePageSize`][Self::sparsePageSize].
        #[unsafe(method(setSparsePageSize:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_sparse_page_size(&self, sparse_page_size: MTLSparsePageSize);

        /// Set hazard tracking mode for the heap. The default value is `HazardTrackingMode::Default`.
        ///
        /// For heaps, MTLHazardTrackingModeDefault is treated as MTLHazardTrackingModeUntracked.
        /// Setting hazardTrackingMode to MTLHazardTrackingModeTracked causes hazard tracking to be enabled heap.
        /// When a resource on a hazard tracked heap is modified, reads and writes from all resources suballocated on that heap will be delayed until the modification is complete.
        /// Similarly, modifying heap resources will be delayed until all in-flight reads and writes from all resources suballocated on that heap have completed.
        /// For optimal performance, perform hazard tracking manually through MTLFence or MTLEvent instead.
        /// All resources created from this heap shared the same hazard tracking mode.
        /// Availability: macOS 10.15+, iOS 13.0+
        #[unsafe(method(hazardTrackingMode))]
        #[unsafe(method_family = none)]
        pub fn hazard_tracking_mode(&self) -> MTLHazardTrackingMode;

        /// Setter for [`hazardTrackingMode`][Self::hazardTrackingMode].
        #[unsafe(method(setHazardTrackingMode:))]
        #[unsafe(method_family = none)]
        pub fn set_hazard_tracking_mode(&self, hazard_tracking_mode: MTLHazardTrackingMode);

        /// A packed tuple of the storageMode, cpuCacheMode and hazardTrackingMode properties.
        ///
        /// Modifications to this property are reflected in the other properties and vice versa.
        ///
        /// Availability: macOS 10.15+, iOS 13.0+
        #[unsafe(method(resourceOptions))]
        #[unsafe(method_family = none)]
        pub fn resource_options(&self) -> MTLResourceOptions;

        /// Setter for [`resourceOptions`][Self::resourceOptions].
        #[unsafe(method(setResourceOptions:))]
        #[unsafe(method_family = none)]
        pub fn set_resource_options(&self, resource_options: MTLResourceOptions);

        /// The type of the heap. The default value is `HeapType::Automatic`.
        ///
        /// This constrains the resource creation functions that are available.
        ///
        /// Availability: macOS 10.15+, iOS 13.0+
        #[unsafe(method(type))]
        #[unsafe(method_family = none)]
        pub unsafe fn r#type(&self) -> MTLHeapType;

        /// Setter for [`type`][Self::type].
        #[unsafe(method(setType:))]
        #[unsafe(method_family = none)]
        pub fn set_type(&self, r#type: MTLHeapType);

        /// Specifies the largest sparse page size that the Metal heap supports.
        ///
        /// This parameter only affects the heap if you set the [`type`][Self::type] property of this descriptor
        /// to [`HeapType::Placement`][HeapType::Placement].
        ///
        /// The value you assign to this property determines the compatibility of the Metal heap with with placement sparse
        /// resources, because placement sparse resources require that their sparse page size be less than or equal to the
        /// placement sparse page of the Metal heap that this property controls.
        ///
        /// Availability: macOS 26.0+, iOS 26.0+
        #[unsafe(method(maxCompatiblePlacementSparsePageSize))]
        #[unsafe(method_family = none)]
        pub unsafe fn max_compatible_placement_sparse_page_size(&self) -> MTLSparsePageSize;

        /// Setter for [`maxCompatiblePlacementSparsePageSize`][Self::maxCompatiblePlacementSparsePageSize].
        #[unsafe(method(setMaxCompatiblePlacementSparsePageSize:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_max_compatible_placement_sparse_page_size(
            &self,
            page_size: MTLSparsePageSize,
        );
    );
}

/// Methods declared on superclass `NSObject`.
impl MTLHeapDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
