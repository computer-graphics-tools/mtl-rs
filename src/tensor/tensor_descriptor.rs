use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
    runtime::NSObject,
};
use objc2_foundation::{CopyingHelper, NSCopying, NSObjectProtocol};

use super::{MTLTensorDataType, MTLTensorExtents, MTLTensorUsage};
use crate::{MTLCPUCacheMode, MTLHazardTrackingMode, MTLResourceOptions, MTLStorageMode};

extern_class!(
    /// A configuration type for creating new tensor instances.
    ///
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtltensordescriptor?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLTensorDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for MTLTensorDescriptor {}
);

unsafe impl CopyingHelper for MTLTensorDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLTensorDescriptor {}
);

impl MTLTensorDescriptor {
    extern_methods!(
        /// An array of sizes, in elements, one for each dimension of the tensors you create with this descriptor.
        ///
        /// The default value of this property is a rank one extents with size one.
        #[unsafe(method(dimensions))]
        #[unsafe(method_family = none)]
        pub fn dimensions(&self) -> Retained<MTLTensorExtents>;

        /// Setter for [`dimensions`][Self::dimensions].
        #[unsafe(method(setDimensions:))]
        #[unsafe(method_family = none)]
        pub fn set_dimensions(&self, dimensions: &MTLTensorExtents);

        /// An array of strides, in elements, one for each dimension in the tensors you create with this descriptor, if applicable.
        ///
        /// This property only applies to tensors you create from a buffer, otherwise it is nil. You are responsible for ensuring `strides` meets the following requirements:
        /// - Elements of `strides`are in monotonically non-decreasing order.
        /// - The first element of `strides` is one.
        /// - For any `i` larger than zero, `strides[i]` is greater than or equal to `strides[i-1] * dimensions[i-1]`.
        /// - If `usage` contains `TensorUsage::MACHINE_LEARNING`, the second element of `strides` is aligned to 64 bytes, and for any `i` larger than one, `strides[i]` is equal to `strides[i-1] * dimensions[i-1]`.
        #[unsafe(method(strides))]
        #[unsafe(method_family = none)]
        pub fn strides(&self) -> Option<Retained<MTLTensorExtents>>;

        /// Setter for [`strides`][Self::strides].
        #[unsafe(method(setStrides:))]
        #[unsafe(method_family = none)]
        pub fn set_strides(&self, strides: Option<&MTLTensorExtents>);

        /// A data format for the tensors you create with this descriptor.
        ///
        /// The default value of this property is `TensorDataType::Float32`.
        #[unsafe(method(dataType))]
        #[unsafe(method_family = none)]
        pub fn data_type(&self) -> MTLTensorDataType;

        /// Setter for [`dataType`][Self::dataType].
        #[unsafe(method(setDataType:))]
        #[unsafe(method_family = none)]
        pub fn set_data_type(&self, data_type: MTLTensorDataType);

        /// A set of contexts in which you can use tensors you create with this descriptor.
        ///
        /// The default value for this property is a bitwise OR of: `TensorUsage::RENDER | TensorUsage::COMPUTE`.
        #[unsafe(method(usage))]
        #[unsafe(method_family = none)]
        pub fn usage(&self) -> MTLTensorUsage;

        /// Setter for [`usage`][Self::usage].
        #[unsafe(method(setUsage:))]
        #[unsafe(method_family = none)]
        pub fn set_usage(&self, usage: MTLTensorUsage);

        /// A packed set of the `storageMode`, `cpuCacheMode` and `hazardTrackingMode` properties.
        #[unsafe(method(resourceOptions))]
        #[unsafe(method_family = none)]
        pub fn resource_options(&self) -> MTLResourceOptions;

        /// Setter for [`resourceOptions`][Self::resourceOptions].
        #[unsafe(method(setResourceOptions:))]
        #[unsafe(method_family = none)]
        pub fn set_resource_options(&self, resource_options: MTLResourceOptions);

        /// A value that configures the cache mode of CPU mapping of tensors you create with this descriptor.
        ///
        /// The default value of this property is `CpuCacheMode::DefaultCache`.
        #[unsafe(method(cpuCacheMode))]
        #[unsafe(method_family = none)]
        pub fn cpu_cache_mode(&self) -> MTLCPUCacheMode;

        /// Setter for [`cpuCacheMode`][Self::cpuCacheMode].
        #[unsafe(method(setCpuCacheMode:))]
        #[unsafe(method_family = none)]
        pub fn set_cpu_cache_mode(&self, cpu_cache_mode: MTLCPUCacheMode);

        /// A value that configures the memory location and access permissions of tensors you create with this descriptor.
        ///
        /// The default value of this property defaults to `StorageMode::Shared`.
        #[unsafe(method(storageMode))]
        #[unsafe(method_family = none)]
        pub fn storage_mode(&self) -> MTLStorageMode;

        /// Setter for [`storageMode`][Self::storageMode].
        #[unsafe(method(setStorageMode:))]
        #[unsafe(method_family = none)]
        pub fn set_storage_mode(&self, storage_mode: MTLStorageMode);

        /// A value that configures the hazard tracking of tensors you create with this descriptor.
        ///
        /// The default value of this property is `HazardTrackingMode::Default`.
        #[unsafe(method(hazardTrackingMode))]
        #[unsafe(method_family = none)]
        pub fn hazard_tracking_mode(&self) -> MTLHazardTrackingMode;

        /// Setter for [`hazardTrackingMode`][Self::hazardTrackingMode].
        #[unsafe(method(setHazardTrackingMode:))]
        #[unsafe(method_family = none)]
        pub fn set_hazard_tracking_mode(&self, hazard_tracking_mode: MTLHazardTrackingMode);
    );
}

/// Methods declared on superclass `NSObject`.
impl MTLTensorDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

#[allow(unused)]
mod tests {
    use super::*;
    use crate::tensor::MTLTensorExtents;

    fn make_extents(vals: &[isize]) -> Retained<MTLTensorExtents> {
        // Safety: We pass a correct pointer or null based on rank.
        MTLTensorExtents::new_with_rank_values(vals.len(), Some(vals)).expect("init extents")
    }

    #[test]
    fn tensor_descriptor_round_trip() {
        let desc = unsafe { MTLTensorDescriptor::new() };

        // dimensions
        let dims_in = make_extents(&[2, 3, 4]);
        desc.set_dimensions(&dims_in);
        let dims_out = desc.dimensions();
        assert_eq!(dims_out.rank(), 3);
        assert_eq!(dims_out.extent_at_dimension_index(0), 2);
        assert_eq!(dims_out.extent_at_dimension_index(1), 3);
        assert_eq!(dims_out.extent_at_dimension_index(2), 4);

        // strides
        let strides_in = make_extents(&[1, 2, 6]);
        desc.set_strides(Some(&strides_in));
        let strides_out = desc.strides().expect("strides set");
        assert_eq!(strides_out.rank(), 3);
        assert_eq!(strides_out.extent_at_dimension_index(0), 1);
        assert_eq!(strides_out.extent_at_dimension_index(1), 2);
        assert_eq!(strides_out.extent_at_dimension_index(2), 6);

        // data type
        desc.set_data_type(MTLTensorDataType::Float16);
        assert_eq!(desc.data_type(), MTLTensorDataType::Float16);

        // usage
        let usage = MTLTensorUsage::COMPUTE | MTLTensorUsage::RENDER;
        desc.set_usage(usage);
        let usage_out = desc.usage();
        assert!(usage_out.contains(MTLTensorUsage::COMPUTE));
        assert!(usage_out.contains(MTLTensorUsage::RENDER));

        // cpu cache mode
        desc.set_cpu_cache_mode(MTLCPUCacheMode::WriteCombined);
        assert_eq!(desc.cpu_cache_mode(), MTLCPUCacheMode::WriteCombined);

        // storage mode
        desc.set_storage_mode(MTLStorageMode::Private);
        assert_eq!(desc.storage_mode(), MTLStorageMode::Private);

        // hazard tracking
        desc.set_hazard_tracking_mode(MTLHazardTrackingMode::Untracked);
        assert_eq!(
            desc.hazard_tracking_mode(),
            MTLHazardTrackingMode::Untracked
        );

        // resource options should reflect the above modes
        let ro = desc.resource_options();
        let expected = MTLResourceOptions::CPU_CACHE_MODE_WRITE_COMBINED
            | MTLResourceOptions::STORAGE_MODE_PRIVATE
            | MTLResourceOptions::HAZARD_TRACKING_MODE_UNTRACKED;
        assert!(ro.contains(expected));
    }
}
