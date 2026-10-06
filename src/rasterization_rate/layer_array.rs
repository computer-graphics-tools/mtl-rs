use objc2::{
    extern_class, extern_conformance, extern_methods, msg_send,
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
    /// Returns the layer descriptor for `layer_index`, or `None` if no layer has been set for this index.
    ///
    /// Use [`set`](Self::set) to set the layer.
    pub fn get(
        &self,
        layer_index: usize,
    ) -> Option<Retained<MTLRasterizationRateLayerDescriptor>> {
        unsafe { msg_send![self, objectAtIndexedSubscript: layer_index] }
    }

    /// Sets `layer` as the layer descriptor for `layer_index`.
    ///
    /// The previous layer at this index is overwritten; `None` removes it.
    pub fn set(
        &self,
        layer_index: usize,
        layer: Option<&MTLRasterizationRateLayerDescriptor>,
    ) {
        unsafe {
            let _: () = msg_send![self, setObject: layer, atIndexedSubscript: layer_index];
        }
    }
}

/// Methods declared on superclass `NSObject`.
impl MTLRasterizationRateLayerArray {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub fn new() -> Retained<Self>;
    );
}
