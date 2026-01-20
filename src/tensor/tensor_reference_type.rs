use objc2::{extern_class, extern_methods, rc::Retained, runtime::NSObject};

use crate::{MTLBindingAccess, MTLDataType, MTLTensorDataType, MTLTensorExtents};

extern_class!(
    /// An object that represents a tensor in the shading language in a struct or array.
    ///
    /// Availability: macOS 26.0+, iOS 26.0+
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLTensorReferenceType;
);

impl MTLTensorReferenceType {
    extern_methods!(
        /// The underlying data format of the tensor.
        #[unsafe(method(tensorDataType))]
        #[unsafe(method_family = none)]
        pub unsafe fn tensor_data_type(&self) -> MTLTensorDataType;

        /// The data format used for indexing into the tensor.
        #[unsafe(method(indexType))]
        #[unsafe(method_family = none)]
        pub unsafe fn index_type(&self) -> MTLDataType;

        /// The array of sizes, in elements, one for each dimension of this tensor.
        ///
        /// For shader-bound tensors, the rank of `dimensions` corresponds to the
        /// rank the shader function specifies, and each extent value is -1.
        #[unsafe(method(dimensions))]
        #[unsafe(method_family = none)]
        pub unsafe fn dimensions(&self) -> Option<Retained<MTLTensorExtents>>;

        /// A value that represents the read/write permissions of the tensor.
        #[unsafe(method(access))]
        #[unsafe(method_family = none)]
        pub unsafe fn access(&self) -> MTLBindingAccess;
    );
}
