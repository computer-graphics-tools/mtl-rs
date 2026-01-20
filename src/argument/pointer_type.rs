use objc2::{extern_class, extern_methods, runtime::NSObject};

use crate::MTLDataType;

extern_class!(
    /// Reflection for a pointer type used by argument/pipeline reflection.
    ///
    /// Availability: macOS 10.13+, iOS 11.0+
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLPointerType;
);

impl MTLPointerType {
    extern_methods!(
        /// The element data type (e.g. Float, Float4, Struct, ...).
        #[unsafe(method(elementType))]
        #[unsafe(method_family = none)]
        pub fn element_type(&self) -> MTLDataType;

        /// Read/write access of the pointer target.
        #[unsafe(method(access))]
        #[unsafe(method_family = none)]
        pub fn access(&self) -> crate::argument::MTLBindingAccess;

        /// Minimum alignment for the element data.
        #[unsafe(method(alignment))]
        #[unsafe(method_family = none)]
        pub fn alignment(&self) -> usize;

        /// Size of the element type in bytes.
        #[unsafe(method(dataSize))]
        #[unsafe(method_family = none)]
        pub fn data_size(&self) -> usize;

        /// Whether the element type is an argument buffer.
        #[unsafe(method(elementIsArgumentBuffer))]
        #[unsafe(method_family = none)]
        pub fn element_is_argument_buffer(&self) -> bool;

        /// If the element is a struct, returns its reflection type.
        ///
        /// Availability: macOS 10.13+, iOS 11.0+
        #[unsafe(method(elementStructType))]
        #[unsafe(method_family = none)]
        pub fn element_struct_type(
            &self,
        ) -> Option<objc2::rc::Retained<crate::argument::MTLStructType>>;

        /// If the element is an array, returns its reflection type.
        ///
        /// Availability: macOS 10.13+, iOS 11.0+
        #[unsafe(method(elementArrayType))]
        #[unsafe(method_family = none)]
        pub fn element_array_type(
            &self,
        ) -> Option<objc2::rc::Retained<crate::argument::MTLArrayType>>;
    );
}
