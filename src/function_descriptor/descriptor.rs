use objc2::{
    extern_class, extern_conformance, extern_methods, msg_send,
    rc::{Allocated, Retained},
    runtime::{NSObject, ProtocolObject},
};
use objc2_foundation::{CopyingHelper, NSArray, NSCopying, NSObjectProtocol, NSString};

use crate::{MTLBinaryArchive, MTLFunctionConstantValues, MTLFunctionOptions};

extern_class!(
    /// Descriptor for locating and specializing a visible function.
    ///
    /// Availability: macOS 11.0+, iOS 14.0+
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLFunctionDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for MTLFunctionDescriptor {}
);

unsafe impl CopyingHelper for MTLFunctionDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLFunctionDescriptor {}
);

impl MTLFunctionDescriptor {
    extern_methods!(
        /// Create an autoreleased function descriptor.
        #[unsafe(method(functionDescriptor))]
        #[unsafe(method_family = none)]
        pub fn function_descriptor() -> Retained<MTLFunctionDescriptor>;

        /// Options for creating the Function.
        #[unsafe(method(options))]
        #[unsafe(method_family = none)]
        pub fn options(&self) -> MTLFunctionOptions;

        /// Setter for [`options`][Self::options].
        #[unsafe(method(setOptions:))]
        #[unsafe(method_family = none)]
        pub fn set_options(&self, options: MTLFunctionOptions);

        /// The set of constant values assigned to the function constants. Compilation fails if you do not provide valid
        /// constant values for all required function constants.
        #[unsafe(method(constantValues))]
        #[unsafe(method_family = none)]
        pub fn constant_values(&self) -> Option<Retained<MTLFunctionConstantValues>>;

        #[unsafe(method(setConstantValues:))]
        #[unsafe(method_family = none)]
        pub fn set_constant_values(&self, values: Option<&MTLFunctionConstantValues>);
    );
}

/// Methods declared on superclass `NSObject`.
impl MTLFunctionDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub fn new() -> Retained<Self>;
    );
}

#[allow(unused)]
impl MTLFunctionDescriptor {
    /// The name of the visible function to find.
    fn name(&self) -> Option<String> {
        let s: Option<Retained<NSString>> = unsafe { msg_send![self, name] };
        s.map(|v| v.to_string())
    }

    /// Setter for [`name`][Self::name].
    fn set_name(&self, name: Option<&str>) {
        unsafe {
            let _: () = msg_send![self, setName: name.map(NSString::from_str).as_deref()];
        }
    }

    /// An optional new name for a visible function to allow reuse with different specializations.
    fn specialized_name(&self) -> Option<String> {
        let s: Option<Retained<NSString>> = unsafe { msg_send![self, specializedName] };
        s.map(|v| v.to_string())
    }

    /// Setter for [`specialized_name`][Self::specialized_name].
    fn set_specialized_name(&self, specialized_name: Option<&str>) {
        unsafe {
            let _: () = msg_send![
                self,
                setSpecializedName: specialized_name.map(NSString::from_str).as_deref()
            ];
        }
    }

    /// Binary archives to be searched for precompiled functions during the compilation of this function.
    fn binary_archives(&self) -> Option<Box<[Retained<ProtocolObject<dyn MTLBinaryArchive>>]>> {
        let array: Option<Retained<NSArray<ProtocolObject<dyn MTLBinaryArchive>>>> =
            unsafe { msg_send![self, binaryArchives] };
        array.map(|a| a.to_vec().into_boxed_slice())
    }

    /// Setter for [`binary_archives`][Self::binary_archives].
    pub fn set_binary_archives(&self, archives: Option<&[&ProtocolObject<dyn MTLBinaryArchive>]>) {
        let archives = archives.map(|archives| NSArray::from_slice(archives));
        unsafe {
            let _: () = msg_send![self, setBinaryArchives: archives.as_deref()];
        }
    }
}
