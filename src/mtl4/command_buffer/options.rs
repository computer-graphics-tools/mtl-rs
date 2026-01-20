use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
    runtime::ProtocolObject,
};
use objc2_foundation::{CopyingHelper, NSCopying, NSObject, NSObjectProtocol};

use crate::*;

extern_class!(
    /// Options to configure a command buffer before encoding work into it.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metal/mtl4commandbufferoptions?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTL4CommandBufferOptions;
);

extern_conformance!(
    unsafe impl NSCopying for MTL4CommandBufferOptions {}
);

unsafe impl CopyingHelper for MTL4CommandBufferOptions {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTL4CommandBufferOptions {}
);

impl MTL4CommandBufferOptions {
    extern_methods!(
        /// Contains information related to shader logging.
        ///
        /// To enable shader logging, call ``MTL4CommandBuffer/beginCommandBufferWithAllocator:options:`` with an instance
        /// of ``MTL4CommandBufferOptions`` that contains a non-`nil` ``MTLLogState`` instance in this property.
        ///
        /// Shader functions log messages until the command buffer ends.
        #[unsafe(method(logState))]
        #[unsafe(method_family = none)]
        pub unsafe fn log_state(&self) -> Option<Retained<ProtocolObject<dyn MTLLogState>>>;

        /// Setter for [`logState`][Self::logState].
        #[unsafe(method(setLogState:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_log_state(&self, log_state: Option<&ProtocolObject<dyn MTLLogState>>);
    );
}

/// Methods declared on superclass `NSObject`.
impl MTL4CommandBufferOptions {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
