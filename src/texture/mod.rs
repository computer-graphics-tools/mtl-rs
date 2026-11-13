mod shared_texture_handle;
mod texture;
mod texture_compression_type;
mod texture_descriptor;
mod texture_swizzle;
mod texture_swizzle_channels;
mod texture_type;
mod texture_usage;
mod texture_view_descriptor;

pub use shared_texture_handle::MTLSharedTextureHandle;
pub use texture::{MTLTexture, TextureExt};
pub use texture_compression_type::MTLTextureCompressionType;
pub use texture_descriptor::MTLTextureDescriptor;
pub use texture_swizzle::MTLTextureSwizzle;
pub use texture_swizzle_channels::MTLTextureSwizzleChannels;
pub use texture_type::MTLTextureType;
pub use texture_usage::MTLTextureUsage;
pub use texture_view_descriptor::MTLTextureViewDescriptor;

#[cfg(test)]
mod tests {
    use core::mem::{align_of, size_of};

    use super::*;

    #[test]
    fn swizzle_channels_match_the_metal_abi_and_helpers() {
        const SWIZZLE: MTLTextureSwizzleChannels = MTLTextureSwizzleChannels::new(
            MTLTextureSwizzle::Blue,
            MTLTextureSwizzle::Green,
            MTLTextureSwizzle::Red,
            MTLTextureSwizzle::One,
        );

        assert_eq!(size_of::<MTLTextureSwizzleChannels>(), 4);
        assert_eq!(align_of::<MTLTextureSwizzleChannels>(), 1);
        assert_eq!(SWIZZLE.blue, MTLTextureSwizzle::Red);
        assert_eq!(MTLTextureSwizzleChannels::default(), MTLTextureSwizzleChannels::DEFAULT);
    }
}
