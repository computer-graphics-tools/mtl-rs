use objc2::{Message, extern_class, extern_methods, msg_send, rc::Retained, runtime::NSObject};
use objc2_foundation::NSString;

use crate::{
    MTLDataType, MTLTextureType,
    argument::{MTLArgumentType, MTLBindingAccess, MTLPointerType, MTLStructType},
};

extern_class!(
    /// Reflection object describing a function argument.
    ///
    /// Availability: macOS 10.11+, iOS 8.0+
    ///
    /// Deprecated in favor of `MTLBinding`-based reflection on macOS 13.0+ and iOS 16.0+.
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLArgument;
);

impl MTLArgument {
    extern_methods!(
        /// Argument type.
        #[unsafe(method(type))]
        #[unsafe(method_family = none)]
        pub fn argument_type(&self) -> MTLArgumentType;

        /// Access permissions for this argument.
        #[unsafe(method(access))]
        #[unsafe(method_family = none)]
        pub fn access(&self) -> MTLBindingAccess;

        /// Bind point index of the argument.
        #[unsafe(method(index))]
        #[unsafe(method_family = none)]
        pub fn index(&self) -> usize;

        /// True if the argument is active.
        #[unsafe(method(isActive))]
        #[unsafe(method_family = none)]
        pub fn is_active(&self) -> bool;

        // Buffer arguments
        /// Minimum alignment of the starting offset in the buffer.
        #[unsafe(method(bufferAlignment))]
        #[unsafe(method_family = none)]
        pub fn buffer_alignment(&self) -> usize;

        /// Size of the buffer data in bytes.
        #[unsafe(method(bufferDataSize))]
        #[unsafe(method_family = none)]
        pub fn buffer_data_size(&self) -> usize;

        /// Buffer data type (Float, Float4, Struct, ...).
        #[unsafe(method(bufferDataType))]
        #[unsafe(method_family = none)]
        pub fn buffer_data_type(&self) -> MTLDataType;

        /// Buffer struct type if the buffer holds a struct.
        #[unsafe(method(bufferStructType))]
        #[unsafe(method_family = none)]
        pub fn buffer_struct_type(&self) -> Option<Retained<MTLStructType>>;

        /// Buffer pointer type.
        #[unsafe(method(bufferPointerType))]
        #[unsafe(method_family = none)]
        pub fn buffer_pointer_type(&self) -> Option<Retained<MTLPointerType>>;

        // Threadgroup memory arguments
        /// Minimum alignment for the threadgroup memory.
        #[unsafe(method(threadgroupMemoryAlignment))]
        #[unsafe(method_family = none)]
        pub fn threadgroup_memory_alignment(&self) -> usize;

        /// Size of the threadgroup memory data in bytes.
        #[unsafe(method(threadgroupMemoryDataSize))]
        #[unsafe(method_family = none)]
        pub fn threadgroup_memory_data_size(&self) -> usize;

        // Texture arguments
        /// The texture type (1D, 2D, etc.).
        #[unsafe(method(textureType))]
        #[unsafe(method_family = none)]
        pub fn texture_type(&self) -> MTLTextureType;

        /// The texture data type (half, float, int, or uint).
        #[unsafe(method(textureDataType))]
        #[unsafe(method_family = none)]
        pub fn texture_data_type(&self) -> MTLDataType;

        /// True if the texture is a depth texture.
        #[unsafe(method(isDepthTexture))]
        #[unsafe(method_family = none)]
        pub fn is_depth_texture(&self) -> bool;

        /// Array length for array textures or arrays of resources.
        #[unsafe(method(arrayLength))]
        #[unsafe(method_family = none)]
        pub fn array_length(&self) -> usize;
    );
}

#[allow(unused)]
pub trait MTLArgumentExt: Message {
    fn name(&self) -> String;
}

impl MTLArgumentExt for MTLArgument {
    fn name(&self) -> String {
        let ns: Retained<NSString> = unsafe { msg_send![self, name] };
        ns.to_string()
    }
}
