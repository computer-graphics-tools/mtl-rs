use core::ffi::c_float;
use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
    runtime::NSObject,
};
use objc2_foundation::{CopyingHelper, NSCopying, NSObjectProtocol, NSString};

use super::{
    MTLSamplerAddressMode, MTLSamplerBorderColor, MTLSamplerMinMagFilter, MTLSamplerMipFilter,
    MTLSamplerReductionMode,
};

extern_class!(
    /// A mutable descriptor used to configure a sampler.
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLSamplerDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for MTLSamplerDescriptor {}
);

unsafe impl CopyingHelper for MTLSamplerDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLSamplerDescriptor {}
);

impl MTLSamplerDescriptor {
    extern_methods!(
        #[unsafe(method(minFilter))]
        #[unsafe(method_family = none)]
        pub fn min_filter(&self) -> MTLSamplerMinMagFilter;

        #[unsafe(method(setMinFilter:))]
        #[unsafe(method_family = none)]
        pub fn set_min_filter(&self, min_filter: MTLSamplerMinMagFilter);

        #[unsafe(method(magFilter))]
        #[unsafe(method_family = none)]
        pub fn mag_filter(&self) -> MTLSamplerMinMagFilter;

        #[unsafe(method(setMagFilter:))]
        #[unsafe(method_family = none)]
        pub fn set_mag_filter(&self, mag_filter: MTLSamplerMinMagFilter);

        #[unsafe(method(mipFilter))]
        #[unsafe(method_family = none)]
        pub fn mip_filter(&self) -> MTLSamplerMipFilter;

        #[unsafe(method(setMipFilter:))]
        #[unsafe(method_family = none)]
        pub fn set_mip_filter(&self, mip_filter: MTLSamplerMipFilter);

        #[unsafe(method(maxAnisotropy))]
        #[unsafe(method_family = none)]
        pub fn max_anisotropy(&self) -> usize;

        #[unsafe(method(setMaxAnisotropy:))]
        #[unsafe(method_family = none)]
        pub fn set_max_anisotropy(&self, max_anisotropy: usize);

        #[unsafe(method(sAddressMode))]
        #[unsafe(method_family = none)]
        pub fn s_address_mode(&self) -> MTLSamplerAddressMode;

        #[unsafe(method(setSAddressMode:))]
        #[unsafe(method_family = none)]
        pub fn set_s_address_mode(&self, mode: MTLSamplerAddressMode);

        #[unsafe(method(tAddressMode))]
        #[unsafe(method_family = none)]
        pub fn t_address_mode(&self) -> MTLSamplerAddressMode;

        #[unsafe(method(setTAddressMode:))]
        #[unsafe(method_family = none)]
        pub fn set_t_address_mode(&self, mode: MTLSamplerAddressMode);

        #[unsafe(method(rAddressMode))]
        #[unsafe(method_family = none)]
        pub fn r_address_mode(&self) -> MTLSamplerAddressMode;

        #[unsafe(method(setRAddressMode:))]
        #[unsafe(method_family = none)]
        pub fn set_r_address_mode(&self, mode: MTLSamplerAddressMode);

        #[unsafe(method(borderColor))]
        #[unsafe(method_family = none)]
        pub fn border_color(&self) -> MTLSamplerBorderColor;

        #[unsafe(method(setBorderColor:))]
        #[unsafe(method_family = none)]
        pub fn set_border_color(&self, color: MTLSamplerBorderColor);

        #[unsafe(method(reductionMode))]
        #[unsafe(method_family = none)]
        pub unsafe fn reduction_mode(&self) -> MTLSamplerReductionMode;

        #[unsafe(method(setReductionMode:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_reduction_mode(&self, mode: MTLSamplerReductionMode);

        #[unsafe(method(normalizedCoordinates))]
        #[unsafe(method_family = none)]
        pub fn normalized_coordinates(&self) -> bool;

        #[unsafe(method(setNormalizedCoordinates:))]
        #[unsafe(method_family = none)]
        pub fn set_normalized_coordinates(&self, normalized: bool);

        #[unsafe(method(lodMinClamp))]
        #[unsafe(method_family = none)]
        pub fn lod_min_clamp(&self) -> c_float;

        #[unsafe(method(setLodMinClamp:))]
        #[unsafe(method_family = none)]
        pub fn set_lod_min_clamp(&self, v: c_float);

        #[unsafe(method(lodMaxClamp))]
        #[unsafe(method_family = none)]
        pub fn lod_max_clamp(&self) -> c_float;

        #[unsafe(method(setLodMaxClamp:))]
        #[unsafe(method_family = none)]
        pub fn set_lod_max_clamp(&self, v: c_float);

        #[unsafe(method(lodAverage))]
        #[unsafe(method_family = none)]
        pub fn lod_average(&self) -> bool;

        #[unsafe(method(setLodAverage:))]
        #[unsafe(method_family = none)]
        pub fn set_lod_average(&self, v: bool);

        #[unsafe(method(lodBias))]
        #[unsafe(method_family = none)]
        pub unsafe fn lod_bias(&self) -> c_float;

        #[unsafe(method(setLodBias:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_lod_bias(&self, v: c_float);

        #[unsafe(method(supportArgumentBuffers))]
        #[unsafe(method_family = none)]
        pub fn support_argument_buffers(&self) -> bool;

        #[unsafe(method(setSupportArgumentBuffers:))]
        #[unsafe(method_family = none)]
        pub fn set_support_argument_buffers(&self, v: bool);

        #[unsafe(method(label))]
        #[unsafe(method_family = none)]
        pub fn label(&self) -> Option<Retained<NSString>>;

        #[unsafe(method(setLabel:))]
        #[unsafe(method_family = none)]
        pub fn set_label(&self, label: Option<&NSString>);
    );
}

/// Methods declared on superclass `NSObject`.
impl MTLSamplerDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub fn new() -> Retained<Self>;
    );
}
