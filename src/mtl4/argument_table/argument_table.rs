use objc2::{Message, extern_protocol, msg_send, rc::Retained, runtime::ProtocolObject};
use objc2_foundation::{NSObjectProtocol, NSString};

use crate::*;

extern_protocol!(
    /// Provides a mechanism to manage and provide resource bindings for buffers, textures, sampler states and other Metal resources.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metal/mtl4argumenttable?language=objc)
    pub unsafe trait MTL4ArgumentTable: NSObjectProtocol {
        /// Binds a GPU address to a buffer binding slot.
        #[unsafe(method(setAddress:atIndex:))]
        #[unsafe(method_family = none)]
        fn set_address_at_index(&self, gpu_address: MTLGPUAddress, binding_index: usize);

        /// Binds a GPU address to a buffer binding slot, providing a dynamic vertex stride.
        #[unsafe(method(setAddress:attributeStride:atIndex:))]
        #[unsafe(method_family = none)]
        fn set_address_attribute_stride_at_index(
            &self,
            gpu_address: MTLGPUAddress,
            stride: usize,
            binding_index: usize,
        );

        /// Binds a resource to a buffer binding slot.
        #[unsafe(method(setResource:atBufferIndex:))]
        #[unsafe(method_family = none)]
        fn set_resource_at_buffer_index(
            &self,
            resource_id: MTLResourceID,
            binding_index: usize,
        );

        /// Binds a texture to a texture binding slot.
        #[unsafe(method(setTexture:atIndex:))]
        #[unsafe(method_family = none)]
        fn set_texture_at_index(&self, resource_id: MTLResourceID, binding_index: usize);

        /// Binds a sampler state to a sampler state binding slot.
        #[unsafe(method(setSamplerState:atIndex:))]
        #[unsafe(method_family = none)]
        fn set_sampler_state_at_index(
            &self,
            resource_id: MTLResourceID,
            binding_index: usize,
        );

        /// The device from which you created this argument table.
        #[unsafe(method(device))]
        #[unsafe(method_family = none)]
        fn device(&self) -> Retained<ProtocolObject<dyn MTLDevice>>;
    }
);

#[allow(unused)]
pub trait MTL4ArgumentTableExt: MTL4ArgumentTable + Message {
    /// Assigns an optional label with this argument table for debugging purposes.
    fn label(&self) -> Option<String>;

    /// Setter for [`label`][Self::label].
    fn set_label(&self, label: Option<&str>);
}

impl<T: MTL4ArgumentTable + Message> MTL4ArgumentTableExt for T {
    fn label(&self) -> Option<String> {
        let s: Option<Retained<NSString>> = unsafe { msg_send![self, label] };
        s.map(|v| v.to_string())
    }

    /// Setter for [`label`][Self::label].
    fn set_label(&self, label: Option<&str>) {
        unsafe {
            let _: () = msg_send![self, setLabel: label.map(NSString::from_str).as_deref()];
        }
    }
}
