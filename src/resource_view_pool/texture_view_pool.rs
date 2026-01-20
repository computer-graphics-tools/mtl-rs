use objc2::{extern_protocol, runtime::ProtocolObject};

use super::MTLResourceViewPool;
use crate::{MTLBuffer, MTLResourceID, MTLTexture, MTLTextureDescriptor};

extern_protocol!(
    /// A pool of lightweight texture views.
    pub unsafe trait MTLTextureViewPool: MTLResourceViewPool {
        /// Copies a default texture view to a slot in this texture view pool at an index provided.
        #[unsafe(method(setTextureView:atIndex:))]
        #[unsafe(method_family = none)]
        unsafe fn set_texture_view_at_index(
            &self,
            texture: &ProtocolObject<dyn MTLTexture>,
            index: usize,
        ) -> MTLResourceID;

        /// Creates a new lightweight texture view.
        #[unsafe(method(setTextureView:descriptor:atIndex:))]
        #[unsafe(method_family = none)]
        unsafe fn set_texture_view_with_descriptor_at_index(
            &self,
            texture: &ProtocolObject<dyn MTLTexture>,
            descriptor: &MTLTextureDescriptor,
            index: usize,
        ) -> MTLResourceID;

        /// Creates a new lightweight texture view of a buffer.
        #[unsafe(method(setTextureViewFromBuffer:descriptor:offset:bytesPerRow:atIndex:))]
        #[unsafe(method_family = none)]
        unsafe fn set_texture_view_from_buffer(
            &self,
            buffer: &ProtocolObject<dyn MTLBuffer>,
            descriptor: &MTLTextureDescriptor,
            offset: usize,
            bytes_per_row: usize,
            index: usize,
        ) -> MTLResourceID;
    }
);
