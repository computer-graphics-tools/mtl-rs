use objc2::{
    encode::{Encode, Encoding, RefEncode},
    extern_class, extern_conformance, extern_methods, msg_send,
    rc::{Allocated, Retained},
};
use objc2_foundation::{CopyingHelper, NSCopying, NSObject, NSObjectProtocol, NSString};

use crate::*;

/// Options for configuring the creation of binary functions.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/metal/mtl4binaryfunctionoptions?language=objc)
// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTL4BinaryFunctionOptions(pub usize);
bitflags::bitflags! {
    impl MTL4BinaryFunctionOptions: usize {
        /// Represents the default value: no options.
        #[doc(alias = "MTL4BinaryFunctionOptionNone")]
        const None = 0;
        /// Compiles the function to have its function handles return a constant MTLResourceID across
        /// all pipeline states. The function needs to be linked to the pipeline that will use this function.
        #[doc(alias = "MTL4BinaryFunctionOptionPipelineIndependent")]
        const PipelineIndependent = 1<<1;
    }
}

unsafe impl Encode for MTL4BinaryFunctionOptions {
    const ENCODING: Encoding = usize::ENCODING;
}

unsafe impl RefEncode for MTL4BinaryFunctionOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// Base interface for other function-derived interfaces.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metal/mtl4binaryfunctiondescriptor?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTL4BinaryFunctionDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for MTL4BinaryFunctionDescriptor {}
);

unsafe impl CopyingHelper for MTL4BinaryFunctionDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTL4BinaryFunctionDescriptor {}
);

impl MTL4BinaryFunctionDescriptor {
    extern_methods!(
        /// Provides the function descriptor corresponding to the function to compile into a binary function.
        #[unsafe(method(functionDescriptor))]
        #[unsafe(method_family = none)]
        pub unsafe fn function_descriptor(&self) -> Retained<MTL4FunctionDescriptor>;

        /// Setter for [`functionDescriptor`][Self::functionDescriptor].
        ///
        /// This is [copied][objc2_foundation::NSCopying::copy] when set.
        #[unsafe(method(setFunctionDescriptor:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_function_descriptor(&self, function_descriptor: &MTL4FunctionDescriptor);

        /// Configure the options to use at binary function creation time.
        #[unsafe(method(options))]
        #[unsafe(method_family = none)]
        pub unsafe fn options(&self) -> MTL4BinaryFunctionOptions;

        /// Setter for [`options`][Self::options].
        #[unsafe(method(setOptions:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_options(&self, options: MTL4BinaryFunctionOptions);
    );
}

/// Methods declared on superclass `NSObject`.
impl MTL4BinaryFunctionDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

impl MTL4BinaryFunctionDescriptor {
    /// Associates a string that uniquely identifies a binary function.
    pub fn name(&self) -> String {
        let s: Retained<NSString> = unsafe { msg_send![self, name] };
        s.to_string()
    }

    /// Setter for name.
    pub fn set_name(&self, name: &str) {
        unsafe {
            let _: () = msg_send![self, setName: &*NSString::from_str(name)];
        }
    }
}
