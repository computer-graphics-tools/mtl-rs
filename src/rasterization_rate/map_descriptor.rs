use objc2::{
    extern_class, extern_conformance, extern_methods, msg_send,
    rc::{Allocated, Retained},
    runtime::NSObject,
};
use objc2_foundation::{CopyingHelper, NSCopying, NSObjectProtocol, NSString};

use super::{MTLRasterizationRateLayerArray, MTLRasterizationRateLayerDescriptor};
use crate::types::MTLSize;

extern_class!(
    /// Describes a rasterization rate map containing layer descriptors.
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLRasterizationRateMapDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for MTLRasterizationRateMapDescriptor {}
);

unsafe impl CopyingHelper for MTLRasterizationRateMapDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLRasterizationRateMapDescriptor {}
);

impl MTLRasterizationRateMapDescriptor {
    extern_methods!(
        /// Convenience descriptor creation without layers.
        #[unsafe(method(rasterizationRateMapDescriptorWithScreenSize:))]
        #[unsafe(method_family = none)]
        pub unsafe fn rasterization_rate_map_descriptor_with_screen_size(
            screen_size: MTLSize,
        ) -> Retained<MTLRasterizationRateMapDescriptor>;

        /// Convenience descriptor creation for a single layer.
        #[unsafe(method(rasterizationRateMapDescriptorWithScreenSize:layer:))]
        #[unsafe(method_family = none)]
        pub unsafe fn rasterization_rate_map_descriptor_with_screen_size_layer(
            screen_size: MTLSize,
            layer: &MTLRasterizationRateLayerDescriptor,
        ) -> Retained<MTLRasterizationRateMapDescriptor>;

        /// Returns the layer descriptor for the given index, if any.
        #[unsafe(method(layerAtIndex:))]
        #[unsafe(method_family = none)]
        pub unsafe fn layer_at_index(
            &self,
            layer_index: usize,
        ) -> Option<Retained<MTLRasterizationRateLayerDescriptor>>;

        /// Sets the layer descriptor for the given index.
        #[unsafe(method(setLayer:atIndex:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_layer_at_index(
            &self,
            layer: Option<&MTLRasterizationRateLayerDescriptor>,
            layer_index: usize,
        );

        /// Access the modifiable array of layers.
        #[unsafe(method(layers))]
        #[unsafe(method_family = none)]
        pub unsafe fn layers(&self) -> Retained<MTLRasterizationRateLayerArray>;

        /// The screen size in pixels of the region where variable rasterization is applied.
        #[unsafe(method(screenSize))]
        #[unsafe(method_family = none)]
        pub unsafe fn screen_size(&self) -> MTLSize;

        /// Setter for [`screen_size`][Self::screen_size].
        #[unsafe(method(setScreenSize:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_screen_size(&self, screen_size: MTLSize);

        /// Number of subsequent non-nil layers starting at index 0.
        #[unsafe(method(layerCount))]
        #[unsafe(method_family = none)]
        pub unsafe fn layer_count(&self) -> usize;
    );
}

/// Methods declared on superclass `NSObject`.
impl MTLRasterizationRateMapDescriptor {
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
impl MTLRasterizationRateMapDescriptor {
    /// Optional label for the descriptor.
    fn label(&self) -> Option<String> {
        let s: Option<Retained<NSString>> = unsafe { msg_send![self, label] };
        s.map(|s| s.to_string())
    }
    /// Setter for [`label`][Self::label].
    fn set_label(&self, label: Option<&str>) {
        unsafe {
            let _: () = msg_send![self, setLabel: label.map(NSString::from_str).as_deref()];
        }
    }
}
