use core::{ffi::c_void, ptr::NonNull};

use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
    runtime::NSObject,
};
use objc2_foundation::{CopyingHelper, NSCopying, NSObjectProtocol, NSRange, NSString};

use crate::MTLDataType;

extern_class!(
    /// Values for Metal function constants (bridged from `MTLFunctionConstantValues`).
    ///
    /// Availability: macOS 10.12+, iOS 10.0+
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLFunctionConstantValues;
);

extern_conformance!(
    unsafe impl NSCopying for MTLFunctionConstantValues {}
);

unsafe impl CopyingHelper for MTLFunctionConstantValues {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLFunctionConstantValues {}
);

impl MTLFunctionConstantValues {
    extern_methods!(
        /// Set a single constant value by index.
        ///
        /// Safety: `value` must be a valid pointer to a value of the specified `type`.
        #[unsafe(method(setConstantValue:type:atIndex:))]
        #[unsafe(method_family = none)]
        pub fn set_constant_value_type_at_index(
            &self,
            value: NonNull<c_void>,
            r#type: MTLDataType,
            index: usize,
        );

        /// Set a range of constant values by index range.
        ///
        /// Safety: `values` must be a valid pointer to an array of values of the specified `type` large enough to cover `range`.
        #[unsafe(method(setConstantValues:type:withRange:))]
        #[unsafe(method_family = none)]
        pub fn set_constant_values_type_with_range(
            &self,
            values: NonNull<c_void>,
            r#type: MTLDataType,
            range: NSRange,
        );

        /// Set a single constant value by name.
        ///
        /// Safety: `value` must be a valid pointer to a value of the specified `type`.
        #[unsafe(method(setConstantValue:type:withName:))]
        #[unsafe(method_family = none)]
        pub fn set_constant_value_type_with_name(
            &self,
            value: NonNull<c_void>,
            r#type: MTLDataType,
            name: &NSString,
        );

        /// Reset all function constant values.
        #[unsafe(method(reset))]
        #[unsafe(method_family = none)]
        pub fn reset(&self);
    );
}

/// Methods declared on superclass `NSObject`.
impl MTLFunctionConstantValues {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub fn new() -> Retained<Self>;
    );
}
