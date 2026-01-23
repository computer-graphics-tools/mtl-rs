use objc2::{Message, extern_protocol, msg_send, rc::Retained, runtime::ProtocolObject};
use objc2_foundation::{NSArray, NSError, NSObjectProtocol, NSString};

use crate::{
    LibraryFunctionCompletionHandler, MTLDevice, MTLFunction, MTLFunctionConstantValues,
    MTLFunctionDescriptor, MTLFunctionReflection, MTLIntersectionFunctionDescriptor,
    MTLLibraryType,
};

extern_protocol!(
    /// Metal library interface.
    pub unsafe trait MTLLibrary: NSObjectProtocol + Send + Sync {
        /// The device this resource was created against.
        #[unsafe(method(device))]
        #[unsafe(method_family = none)]
        fn device(&self) -> Retained<ProtocolObject<dyn MTLDevice>>;

        #[unsafe(method(type))]
        #[unsafe(method_family = none)]
        fn r#type(&self) -> MTLLibraryType;

        /// Synchronously creates a new function object from a descriptor.
        #[unsafe(method(newFunctionWithDescriptor:error:))]
        #[unsafe(method_family = new)]
        fn new_function_with_descriptor_error(
            &self,
            descriptor: &MTLFunctionDescriptor,
            error: *mut *mut NSError,
        ) -> Option<Retained<ProtocolObject<dyn MTLFunction>>>;
    }
);

#[allow(unused)]
/// Rust-friendly helpers for `MTLLibrary`.
pub trait MTLLibraryExt: MTLLibrary + Message {
    /// Optional label.
    fn label(&self) -> Option<String>;

    /// Setter for [`label`][Self::label].
    fn set_label(&self, label: Option<&str>);

    /// Synchronously creates a new function object by name.
    fn new_function_with_name(
        &self,
        function_name: &str,
    ) -> Option<Retained<ProtocolObject<dyn MTLFunction>>>;

    /// Returns a function obtained by applying constant values to the named function (synchronous).
    fn new_function_with_name_constant_values_error(
        &self,
        name: &str,
        constant_values: &MTLFunctionConstantValues,
        error: *mut *mut NSError,
    ) -> Option<Retained<ProtocolObject<dyn MTLFunction>>>;

    /// Asynchronously creates a function by applying constant values to the named function.
    fn new_function_with_name_constant_values_completion_handler(
        &self,
        name: &str,
        constant_values: &MTLFunctionConstantValues,
        completion_handler: LibraryFunctionCompletionHandler,
    );

    /// Returns a reflection object for a function by name.
    fn reflection_for_function_with_name(
        &self,
        function_name: &str,
    ) -> Option<Retained<MTLFunctionReflection>>;

    /// Returns the list of function names contained in the library.
    fn function_names(&self) -> Box<[String]>;

    /// The install name for this library, if any.
    fn install_name(&self) -> Option<String>;

    /// Asynchronously creates a new function object from a descriptor.
    fn new_function_with_descriptor_completion_handler(
        &self,
        descriptor: &MTLFunctionDescriptor,
        completion_handler: LibraryFunctionCompletionHandler,
    );

    /// Asynchronously creates a new intersection function object.
    fn new_intersection_function_with_descriptor_completion_handler(
        &self,
        descriptor: &MTLIntersectionFunctionDescriptor,
        completion_handler: LibraryFunctionCompletionHandler,
    );

    /// Synchronously creates a new intersection function object.
    fn new_intersection_function_with_descriptor_error(
        &self,
        descriptor: &MTLIntersectionFunctionDescriptor,
        error: *mut *mut NSError,
    ) -> Option<Retained<ProtocolObject<dyn MTLFunction>>>;
}

impl MTLLibraryExt for ProtocolObject<dyn MTLLibrary> {
    fn label(&self) -> Option<String> {
        let label: Option<Retained<NSString>> = unsafe { msg_send![self, label] };
        label.map(|label| label.to_string())
    }

    fn set_label(&self, label: Option<&str>) {
        unsafe {
            let _: () = msg_send![self, setLabel: label.map(NSString::from_str).as_deref()];
        }
    }

    fn new_function_with_name(
        &self,
        function_name: &str,
    ) -> Option<Retained<ProtocolObject<dyn MTLFunction>>> {
        let ns_name = NSString::from_str(function_name);
        unsafe { msg_send![self, newFunctionWithName: &*ns_name] }
    }

    /// Returns a function obtained by applying constant values to the named function (synchronous).
    fn new_function_with_name_constant_values_error(
        &self,
        name: &str,
        constant_values: &MTLFunctionConstantValues,
        error: *mut *mut NSError,
    ) -> Option<Retained<ProtocolObject<dyn MTLFunction>>> {
        let ns_name = NSString::from_str(name);
        unsafe {
            msg_send![
                self,
                newFunctionWithName: &*ns_name,
                constantValues: constant_values,
                error: error
            ]
        }
    }

    /// Asynchronously creates a function by applying constant values to the named function.
    fn new_function_with_name_constant_values_completion_handler(
        &self,
        name: &str,
        constant_values: &MTLFunctionConstantValues,
        completion_handler: LibraryFunctionCompletionHandler,
    ) {
        let ns_name = NSString::from_str(name);
        unsafe {
            let _: () = msg_send![
                self,
                newFunctionWithName: &*ns_name,
                constantValues: constant_values,
                completionHandler: &*completion_handler
            ];
        }
    }

    fn reflection_for_function_with_name(
        &self,
        function_name: &str,
    ) -> Option<Retained<MTLFunctionReflection>> {
        let ns_name = NSString::from_str(function_name);
        unsafe { msg_send![self, reflectionForFunctionWithName: &*ns_name] }
    }

    fn function_names(&self) -> Box<[String]> {
        let names: Retained<NSArray<NSString>> = unsafe { msg_send![self, functionNames] };
        unsafe { names.to_vec_unchecked() }
            .into_iter()
            .map(|ns| ns.to_string())
            .collect::<Vec<String>>()
            .into_boxed_slice()
    }

    fn install_name(&self) -> Option<String> {
        let name: Option<Retained<NSString>> = unsafe { msg_send![self, installName] };
        name.map(|s| s.to_string())
    }

    fn new_function_with_descriptor_completion_handler(
        &self,
        descriptor: &MTLFunctionDescriptor,
        completion_handler: LibraryFunctionCompletionHandler,
    ) {
        unsafe {
            let _: () = msg_send![
                self,
                newFunctionWithDescriptor: descriptor,
                completionHandler: &*completion_handler
            ];
        }
    }

    fn new_intersection_function_with_descriptor_completion_handler(
        &self,
        descriptor: &MTLIntersectionFunctionDescriptor,
        completion_handler: LibraryFunctionCompletionHandler,
    ) {
        unsafe {
            let _: () = msg_send![
                self,
                newIntersectionFunctionWithDescriptor: descriptor,
                completionHandler: &*completion_handler
            ];
        }
    }

    fn new_intersection_function_with_descriptor_error(
        &self,
        descriptor: &MTLIntersectionFunctionDescriptor,
        error: *mut *mut NSError,
    ) -> Option<Retained<ProtocolObject<dyn MTLFunction>>> {
        unsafe {
            msg_send![self, newIntersectionFunctionWithDescriptor: descriptor, error: error]
        }
    }
}
