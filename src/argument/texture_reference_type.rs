use objc2::{extern_class, extern_methods, runtime::NSObject};

use crate::MTLDataType;

extern_class!(
    /// Reflection for a texture reference type.
    ///
    /// Availability: macOS 10.13+, iOS 11.0+
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLTextureReferenceType;
);

impl MTLTextureReferenceType {
    extern_methods!(
        /// The underlying data type for the texture (half, float, int, or uint).
        #[unsafe(method(textureDataType))]
        #[unsafe(method_family = none)]
        pub fn texture_data_type(&self) -> MTLDataType;

        /// The texture type (1D, 2D, etc.).
        #[unsafe(method(textureType))]
        #[unsafe(method_family = none)]
        pub fn texture_type(&self) -> crate::MTLTextureType;

        /// Read/write access for the texture.
        #[unsafe(method(access))]
        #[unsafe(method_family = none)]
        pub fn access(&self) -> crate::argument::MTLBindingAccess;

        /// Whether this is a depth texture.
        #[unsafe(method(isDepthTexture))]
        #[unsafe(method_family = none)]
        pub fn is_depth_texture(&self) -> bool;
    );
}
