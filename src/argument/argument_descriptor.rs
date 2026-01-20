use objc2::rc::Retained;
use objc2::runtime::NSObject;
use objc2::{extern_class, extern_conformance, extern_methods};
use objc2_foundation::{CopyingHelper, NSCopying, NSObjectProtocol};

use crate::{MTLBindingAccess, MTLDataType, MTLTextureType};

extern_class!(
    /// Represents a member of an argument buffer.
    ///
    /// See also Apple's documentation for MTLArgumentDescriptor.
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLArgumentDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for MTLArgumentDescriptor {}
);

unsafe impl CopyingHelper for MTLArgumentDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLArgumentDescriptor {}
);

impl MTLArgumentDescriptor {
    extern_methods!(
        /// Create an autoreleased default argument descriptor.
        #[unsafe(method(argumentDescriptor))]
        pub fn new() -> Retained<Self>;

        /// For constants, the data type. Otherwise, MTLDataTypeTexture, MTLDataTypeSampler, or MTLDataTypePointer.
        #[unsafe(method(dataType))]
        #[unsafe(method_family = none)]
        pub fn data_type(&self) -> MTLDataType;

        /// Sets the data type of the argument.
        #[unsafe(method(setDataType:))]
        pub fn set_data_type(&self, value: MTLDataType);

        /// The binding point index of the argument.
        #[unsafe(method(index))]
        #[unsafe(method_family = none)]
        pub fn index(&self) -> usize;

        /// Sets the binding point index of the argument.
        #[unsafe(method(setIndex:))]
        pub fn set_index(&self, value: usize);

        /// The length of an array of constants, textures, or samplers, or 0 for non-array arguments.
        #[unsafe(method(arrayLength))]
        #[unsafe(method_family = none)]
        pub fn array_length(&self) -> usize;

        /// Sets the array length of the argument.
        #[unsafe(method(setArrayLength:))]
        pub fn set_array_length(&self, value: usize);

        /// Access flags for the argument.
        #[unsafe(method(access))]
        #[unsafe(method_family = none)]
        pub fn access(&self) -> MTLBindingAccess;

        /// Sets the access flags for the argument.
        #[unsafe(method(setAccess:))]
        pub fn set_access(&self, value: MTLBindingAccess);

        /// For texture arguments, the texture type.
        #[unsafe(method(textureType))]
        #[unsafe(method_family = none)]
        pub fn texture_type(&self) -> MTLTextureType;

        /// Sets the texture type for the argument.
        #[unsafe(method(setTextureType:))]
        pub fn set_texture_type(&self, value: MTLTextureType);

        /// If set, forces the constant block to be aligned to the given alignment.
        /// Should only be set on the first constant of the block and is only valid if a corresponding
        /// explicit "alignas" is applied to the constant in the metal shader language.
        #[unsafe(method(constantBlockAlignment))]
        #[unsafe(method_family = none)]
        pub fn constant_block_alignment(&self) -> usize;

        /// Sets the constant block alignment.
        #[unsafe(method(setConstantBlockAlignment:))]
        pub fn set_constant_block_alignment(&self, value: usize);
    );
}
