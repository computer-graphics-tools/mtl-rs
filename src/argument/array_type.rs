use objc2::{extern_class, extern_methods, rc::Retained, runtime::NSObject};

use crate::MTLDataType;

extern_class!(
    /// Reflection for an array type.
    ///
    /// Availability: macOS 10.11+, iOS 8.0+
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLArrayType;
);

impl MTLArrayType {
    extern_methods!(
        /// Element data type.
        #[unsafe(method(elementType))]
        #[unsafe(method_family = none)]
        pub fn element_type(&self) -> MTLDataType;

        /// Array length.
        #[unsafe(method(arrayLength))]
        #[unsafe(method_family = none)]
        pub fn array_length(&self) -> usize;

        /// Stride between elements in bytes.
        #[unsafe(method(stride))]
        #[unsafe(method_family = none)]
        pub fn stride(&self) -> usize;

        /// Argument index stride for argument buffers.
        ///
        /// Availability: macOS 10.13+, iOS 11.0+
        #[unsafe(method(argumentIndexStride))]
        #[unsafe(method_family = none)]
        pub fn argument_index_stride(&self) -> usize;

        /// If elements are structs, returns the element struct type.
        #[unsafe(method(elementStructType))]
        #[unsafe(method_family = none)]
        pub fn element_struct_type(&self) -> Option<Retained<crate::argument::MTLStructType>>;

        /// If elements are arrays, returns the element array type.
        #[unsafe(method(elementArrayType))]
        #[unsafe(method_family = none)]
        pub fn element_array_type(&self) -> Option<Retained<MTLArrayType>>;

        /// If elements are textures, returns the element texture reference type.
        ///
        /// Availability: macOS 10.13+, iOS 11.0+
        #[unsafe(method(elementTextureReferenceType))]
        #[unsafe(method_family = none)]
        pub fn element_texture_reference_type(
            &self,
        ) -> Option<Retained<crate::argument::MTLTextureReferenceType>>;

        /// If elements are pointers, returns the element pointer type.
        ///
        /// Availability: macOS 10.13+, iOS 11.0+
        #[unsafe(method(elementPointerType))]
        #[unsafe(method_family = none)]
        pub fn element_pointer_type(&self) -> Option<Retained<crate::argument::MTLPointerType>>;

        /// If elements are tensors, returns the element tensor reference type.
        ///
        /// Availability: macOS 26.0+, iOS 26.0+
        #[unsafe(method(elementTensorReferenceType))]
        #[unsafe(method_family = none)]
        pub fn element_tensor_reference_type(
            &self,
        ) -> Option<Retained<crate::MTLTensorReferenceType>>;
    );
}
