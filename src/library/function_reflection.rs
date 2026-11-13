use objc2::{
    extern_class, extern_conformance, msg_send,
    rc::Retained,
    runtime::{NSObject, ProtocolObject},
};
use objc2_foundation::{NSArray, NSObjectProtocol, NSString};

use crate::MTLBinding;

extern_class!(
    /// Reflection info for a function in a Metal library.
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLFunctionReflection;
);

// SAFETY (Send + Sync): [Apple lists this class's conformances](https://developer.apple.com/documentation/metal/mtlfunctionreflection):
//
// > `Sendable`
//
// This permits transferring and concurrently sharing instances of this reference type.
unsafe impl Send for MTLFunctionReflection {}
unsafe impl Sync for MTLFunctionReflection {}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLFunctionReflection {}
);

impl MTLFunctionReflection {
    /// The function's input and output bindings.
    pub fn bindings(&self) -> Box<[Retained<ProtocolObject<dyn MTLBinding>>]> {
        let bindings: Retained<NSArray<ProtocolObject<dyn MTLBinding>>> = unsafe { msg_send![self, bindings] };
        bindings.to_vec().into_boxed_slice()
    }

    /// The function's user annotation, when one is present.
    pub fn user_annotation(&self) -> Option<String> {
        let annotation: Option<Retained<NSString>> = unsafe { msg_send![self, userAnnotation] };
        annotation.map(|annotation| annotation.to_string())
    }
}
