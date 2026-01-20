use objc2::{Message, extern_class, extern_methods, msg_send, rc::Retained, runtime::NSObject};
use objc2_foundation::NSString;

use crate::{MTLArrayType, MTLDataType, MTLPointerType, MTLTextureReferenceType};

extern_class!(
    /// Reflection for a member of a struct type.
    ///
    /// Availability: macOS 10.11+, iOS 8.0+
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLStructMember;
);

impl MTLStructMember {
    extern_methods!(
        /// Byte offset of the member within the struct.
        #[unsafe(method(offset))]
        #[unsafe(method_family = none)]
        pub fn offset(&self) -> usize;

        /// Data type of the member.
        #[unsafe(method(dataType))]
        #[unsafe(method_family = none)]
        pub fn data_type(&self) -> MTLDataType;

        /// If the member is itself a struct, returns its reflection type.
        #[unsafe(method(structType))]
        #[unsafe(method_family = none)]
        pub fn struct_type(&self) -> Option<Retained<crate::argument::MTLStructType>>;

        /// If the member is an array, returns its reflection type.
        #[unsafe(method(arrayType))]
        #[unsafe(method_family = none)]
        pub fn array_type(&self) -> Option<Retained<MTLArrayType>>;

        /// If the member is a texture reference, returns its reflection type.
        #[unsafe(method(textureReferenceType))]
        #[unsafe(method_family = none)]
        pub fn texture_reference_type(&self) -> Option<Retained<MTLTextureReferenceType>>;

        /// If the member is a pointer, returns its reflection type.
        #[unsafe(method(pointerType))]
        #[unsafe(method_family = none)]
        pub fn pointer_type(&self) -> Option<Retained<MTLPointerType>>;

        /// If the member holds a tensor, returns its tensor reference type.
        ///
        /// Availability: macOS 26.0+, iOS 26.0+
        #[unsafe(method(tensorReferenceType))]
        #[unsafe(method_family = none)]
        pub fn tensor_reference_type(&self) -> Option<Retained<crate::MTLTensorReferenceType>>;

        /// Argument index used when the struct represents a function argument.
        #[unsafe(method(argumentIndex))]
        #[unsafe(method_family = none)]
        pub fn argument_index(&self) -> usize;
    );
}

#[allow(unused)]
pub trait MTLStructMemberExt: Message {
    fn name(&self) -> String;
}

impl MTLStructMemberExt for MTLStructMember {
    fn name(&self) -> String {
        let ns: Retained<NSString> = unsafe { msg_send![self, name] };
        ns.to_string()
    }
}
