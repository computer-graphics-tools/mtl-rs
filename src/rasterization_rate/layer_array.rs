use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
    runtime::NSObject,
};
use objc2_foundation::NSObjectProtocol;

use super::MTLRasterizationRateLayerDescriptor;

extern_class!(
    /// Mutable array of rasterization rate layer descriptors
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLRasterizationRateLayerArray;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLRasterizationRateLayerArray {}
);

impl MTLRasterizationRateLayerArray {
    extern_methods!(
        /// Returns the layer descriptor for the given index, if any.
        #[unsafe(method(objectAtIndexedSubscript:))]
        #[unsafe(method_family = none)]
        pub unsafe fn object_at_indexed_subscript(
            &self,
            layer_index: usize,
        ) -> Option<Retained<MTLRasterizationRateLayerDescriptor>>;

        /// Sets the layer descriptor at the given index.
        #[unsafe(method(setObject:atIndexedSubscript:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_object_at_indexed_subscript(
            &self,
            layer: Option<&MTLRasterizationRateLayerDescriptor>,
            layer_index: usize,
        );
    );
}

/// Methods declared on superclass `NSObject`.
impl MTLRasterizationRateLayerArray {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
