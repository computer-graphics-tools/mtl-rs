use objc2::{Message, extern_protocol, msg_send, rc::Retained, runtime::ProtocolObject};
use objc2_foundation::{NSObjectProtocol, NSString};

use crate::{MTLDataType, MTLTensorDataType, MTLTensorExtents, MTLTextureType};

extern_protocol!(
    /// Describes a resource binding.
    ///
    /// Availability: macOS 13.0+, iOS 16.0+
    pub unsafe trait MTLBinding: NSObjectProtocol {
        /// Type of the binding.
        #[unsafe(method(type))]
        #[unsafe(method_family = none)]
        fn binding_type(&self) -> crate::argument::MTLBindingType;

        /// Access flags of the binding.
        #[unsafe(method(access))]
        #[unsafe(method_family = none)]
        fn access(&self) -> crate::argument::MTLBindingAccess;

        /// Bind point index of the binding.
        #[unsafe(method(index))]
        #[unsafe(method_family = none)]
        fn index(&self) -> usize;

        /// Whether the binding is used.
        #[unsafe(method(isUsed))]
        #[unsafe(method_family = none)]
        fn is_used(&self) -> bool;

        /// Whether this reflection entry is an argument as opposed to a function constant.
        #[unsafe(method(isArgument))]
        #[unsafe(method_family = none)]
        fn is_argument(&self) -> bool;
    }
);

#[allow(unused)]
pub trait MTLBindingExt: MTLBinding + Message {
    fn name(&self) -> String;
}

impl MTLBindingExt for ProtocolObject<dyn MTLBinding> {
    fn name(&self) -> String {
        let ns: Retained<NSString> = unsafe { msg_send![self, name] };
        ns.to_string()
    }
}

extern_protocol!(
    /// Binding for buffers.
    ///
    /// Availability: macOS 13.0+, iOS 16.0+
    pub unsafe trait MTLBufferBinding: MTLBinding {
        #[unsafe(method(bufferAlignment))]
        #[unsafe(method_family = none)]
        fn buffer_alignment(&self) -> usize;

        #[unsafe(method(bufferDataSize))]
        #[unsafe(method_family = none)]
        fn buffer_data_size(&self) -> usize;

        #[unsafe(method(bufferDataType))]
        #[unsafe(method_family = none)]
        fn buffer_data_type(&self) -> MTLDataType;

        #[unsafe(method(bufferStructType))]
        #[unsafe(method_family = none)]
        fn buffer_struct_type(&self) -> Option<Retained<crate::argument::MTLStructType>>;

        #[unsafe(method(bufferPointerType))]
        #[unsafe(method_family = none)]
        fn buffer_pointer_type(&self) -> Option<Retained<crate::argument::MTLPointerType>>;
    }
);

extern_protocol!(
    /// Binding for threadgroup memory.
    ///
    /// Availability: macOS 13.0+, iOS 16.0+
    pub unsafe trait MTLThreadgroupBinding: MTLBinding {
        #[unsafe(method(threadgroupMemoryAlignment))]
        #[unsafe(method_family = none)]
        fn threadgroup_memory_alignment(&self) -> usize;

        #[unsafe(method(threadgroupMemoryDataSize))]
        #[unsafe(method_family = none)]
        fn threadgroup_memory_data_size(&self) -> usize;
    }
);

extern_protocol!(
    /// Binding for textures.
    ///
    /// Availability: macOS 13.0+, iOS 16.0+
    pub unsafe trait MTLTextureBinding: MTLBinding {
        #[unsafe(method(textureType))]
        #[unsafe(method_family = none)]
        fn texture_type(&self) -> MTLTextureType;

        #[unsafe(method(textureDataType))]
        #[unsafe(method_family = none)]
        fn texture_data_type(&self) -> MTLDataType;

        #[unsafe(method(isDepthTexture))]
        #[unsafe(method_family = none)]
        fn is_depth_texture(&self) -> bool;

        #[unsafe(method(arrayLength))]
        #[unsafe(method_family = none)]
        fn array_length(&self) -> usize;
    }
);

extern_protocol!(
    /// Binding for object payloads.
    ///
    /// Availability: macOS 13.0+, iOS 16.0+
    pub unsafe trait MTLObjectPayloadBinding: MTLBinding {
        #[unsafe(method(objectPayloadAlignment))]
        #[unsafe(method_family = none)]
        fn object_payload_alignment(&self) -> usize;

        #[unsafe(method(objectPayloadDataSize))]
        #[unsafe(method_family = none)]
        fn object_payload_data_size(&self) -> usize;
    }
);

extern_protocol!(
    /// Binding for a tensor resource bound to a graphics, compute, or ML function.
    ///
    /// Availability: macOS 26.0+, iOS 26.0+
    pub unsafe trait MTLTensorBinding: MTLBinding {
        /// The underlying data format of this tensor.
        #[unsafe(method(tensorDataType))]
        #[unsafe(method_family = none)]
        fn tensor_data_type(&self) -> MTLTensorDataType;

        /// The data format used for indexing into the tensor.
        #[unsafe(method(indexType))]
        #[unsafe(method_family = none)]
        fn index_type(&self) -> MTLDataType;

        /// The array of sizes, in elements, one for each dimension of this tensor.
        ///
        /// For shader-bound tensors with dynamic extents, the rank corresponds to
        /// the shader-specified rank and each extent value is -1.
        #[unsafe(method(dimensions))]
        #[unsafe(method_family = none)]
        fn dimensions(&self) -> Option<Retained<MTLTensorExtents>>;
    }
);
