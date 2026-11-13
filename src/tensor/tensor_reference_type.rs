use objc2::{extern_class, extern_methods, msg_send, rc::Retained, runtime::NSObject};
use objc2_foundation::NSArray;

use crate::{MTLBindingAccess, MTLDataType, MTLTensorAuxiliaryPlaneType, MTLTensorDataType, MTLTensorExtents, MTLType};

extern_class!(
    /// An object that represents a tensor in the shading language in a struct or array.
    ///
    /// Availability: macOS 26.0+, iOS 26.0+
    #[unsafe(super(MTLType, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLTensorReferenceType;
);

impl MTLTensorReferenceType {
    extern_methods!(
        /// The underlying data format of the tensor.
        #[unsafe(method(tensorDataType))]
        #[unsafe(method_family = none)]
        pub fn tensor_data_type(&self) -> MTLTensorDataType;

        /// The data format used for indexing into the tensor.
        #[unsafe(method(indexType))]
        #[unsafe(method_family = none)]
        pub fn index_type(&self) -> MTLDataType;

        /// The array of sizes, in elements, one for each dimension of this tensor.
        ///
        /// For shader-bound tensors, the rank of `dimensions` corresponds to the
        /// rank the shader function specifies, and each extent value is -1.
        #[unsafe(method(dimensions))]
        #[unsafe(method_family = none)]
        pub fn dimensions(&self) -> Option<Retained<MTLTensorExtents>>;

        /// A value that represents the read/write permissions of the tensor.
        #[unsafe(method(access))]
        #[unsafe(method_family = none)]
        pub fn access(&self) -> MTLBindingAccess;
    );

    /// The auxiliary planes that this tensor reference requires.
    ///
    /// Availability: macOS 27.0+, iOS 27.0+
    pub fn auxiliary_planes(&self) -> Box<[Retained<MTLTensorAuxiliaryPlaneType>]> {
        let planes: Retained<NSArray<MTLTensorAuxiliaryPlaneType>> = unsafe { msg_send![self, auxiliaryPlanes] };
        planes.to_vec().into_boxed_slice()
    }
}
