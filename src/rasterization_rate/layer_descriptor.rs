use core::ffi::c_float;
use core::ptr::NonNull;
use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
    runtime::NSObject,
};
use objc2_foundation::{CopyingHelper, NSCopying, NSObjectProtocol};

use super::MTLRasterizationRateSampleArray;
use crate::types::MTLSize;

extern_class!(
    /// Describes the minimum rasterization rate screen space using two piecewise linear functions.
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLRasterizationRateLayerDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for MTLRasterizationRateLayerDescriptor {}
);

unsafe impl CopyingHelper for MTLRasterizationRateLayerDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLRasterizationRateLayerDescriptor {}
);

impl MTLRasterizationRateLayerDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        /// Initialize a descriptor for a layer with the given number of quality samples on the horizontal and vertical axis.
        #[unsafe(method(initWithSampleCount:))]
        #[unsafe(method_family = init)]
        pub unsafe fn init_with_sample_count(
            this: Allocated<Self>,
            sample_count: MTLSize,
        ) -> Retained<Self>;

        /// Initialize a descriptor for a layer with the given number of quality samples and initial values.
        /// Safety: `horizontal` and `vertical` must be valid pointers with appropriate lengths.
        #[unsafe(method(initWithSampleCount:horizontal:vertical:))]
        #[unsafe(method_family = init)]
        pub unsafe fn init_with_sample_count_horizontal_vertical(
            this: Allocated<Self>,
            sample_count: MTLSize,
            horizontal: NonNull<c_float>,
            vertical: NonNull<c_float>,
        ) -> Retained<Self>;

        /// The maximum number of quality samples this descriptor can use for the horizontal and vertical axes.
        #[unsafe(method(maxSampleCount))]
        #[unsafe(method_family = none)]
        pub unsafe fn max_sample_count(&self) -> MTLSize;

        /// Pointer to mutable storage array for horizontal samples.
        #[unsafe(method(horizontalSampleStorage))]
        #[unsafe(method_family = none)]
        pub unsafe fn horizontal_sample_storage(&self) -> NonNull<c_float>;

        /// Pointer to mutable storage array for vertical samples.
        #[unsafe(method(verticalSampleStorage))]
        #[unsafe(method_family = none)]
        pub unsafe fn vertical_sample_storage(&self) -> NonNull<c_float>;

        /// Bounds-checked access helper for horizontal samples.
        #[unsafe(method(horizontal))]
        #[unsafe(method_family = none)]
        pub unsafe fn horizontal(&self) -> Retained<MTLRasterizationRateSampleArray>;

        /// Bounds-checked access helper for vertical samples.
        #[unsafe(method(vertical))]
        #[unsafe(method_family = none)]
        pub unsafe fn vertical(&self) -> Retained<MTLRasterizationRateSampleArray>;

        /// Setter for `sampleCount`.
        #[unsafe(method(setSampleCount:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_sample_count(&self, sample_count: MTLSize);
    );
}
