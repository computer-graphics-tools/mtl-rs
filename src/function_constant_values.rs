use core::{ffi::c_void, ops::Range, ptr::NonNull};

use objc2::{
    extern_class, extern_conformance, extern_methods, msg_send,
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
        /// For the duration of this call, `value` must point to a properly
        /// aligned, readable value whose representation matches `type`.
        /// `index` must also be a valid function-constant index.
        #[unsafe(method(setConstantValue:type:atIndex:))]
        #[unsafe(method_family = none)]
        pub fn set_constant_value_type_at_index(
            &self,
            value: NonNull<c_void>,
            r#type: MTLDataType,
            index: usize,
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

impl MTLFunctionConstantValues {
    /// Set a range of constant values by index range.
    ///
    /// For the duration of this call, `values` must point to at least
    /// `range.len()` properly aligned, contiguous, readable values whose
    /// representations match `type`. The range must contain valid
    /// function-constant indices.
    pub fn set_constant_values_type_with_range(
        &self,
        values: NonNull<c_void>,
        r#type: MTLDataType,
        range: Range<usize>,
    ) {
        let range = NSRange::from(range);
        unsafe {
            msg_send![
                self,
                setConstantValues: values,
                type: r#type,
                withRange: range,
            ]
        }
    }

    /// Set a single constant value by name.
    ///
    /// For the duration of this call, `value` must point to a properly
    /// aligned, readable value whose representation matches `type`.
    pub fn set_constant_value_type_with_name(
        &self,
        value: NonNull<c_void>,
        r#type: MTLDataType,
        name: &str,
    ) {
        unsafe {
            msg_send![
                self,
                setConstantValue: value,
                type: r#type,
                withName: &*NSString::from_str(name),
            ]
        }
    }
}
