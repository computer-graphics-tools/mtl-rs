use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
    runtime::{NSObject, ProtocolObject},
};
use objc2_foundation::{CopyingHelper, NSCopying, NSObjectProtocol};

use crate::{MTLCommandBufferErrorOption, log_state::MTLLogState};

extern_class!(
    /// An object that you use to configure new Metal command buffer objects.
    ///
    /// Availability: macOS 11.0+, iOS 14.0+
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLCommandBufferDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for MTLCommandBufferDescriptor {}
);

unsafe impl CopyingHelper for MTLCommandBufferDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLCommandBufferDescriptor {}
);

impl MTLCommandBufferDescriptor {
    extern_methods!(
        /// If true, the created command buffer holds strong references to objects needed for it to execute. If false, the created command buffer does not hold strong references to objects needed for it to execute.
        #[unsafe(method(retainedReferences))]
        #[unsafe(method_family = none)]
        pub fn retained_references(&self) -> bool;

        #[unsafe(method(setRetainedReferences:))]
        #[unsafe(method_family = none)]
        pub fn set_retained_references(&self, retained: bool);

        /// A set of options to influence the error reporting of the created command buffer.
        #[unsafe(method(errorOptions))]
        #[unsafe(method_family = none)]
        pub fn error_options(&self) -> MTLCommandBufferErrorOption;

        #[unsafe(method(setErrorOptions:))]
        #[unsafe(method_family = none)]
        pub fn set_error_options(&self, opts: MTLCommandBufferErrorOption);

        /// Contains information related to shader logging.
        ///
        /// Availability: macOS 15.0+, iOS 18.0+
        #[unsafe(method(logState))]
        #[unsafe(method_family = none)]
        pub fn log_state(&self) -> Option<Retained<ProtocolObject<dyn MTLLogState>>>;

        #[unsafe(method(setLogState:))]
        #[unsafe(method_family = none)]
        pub fn set_log_state(&self, log_state: Option<&ProtocolObject<dyn MTLLogState>>);
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub fn new() -> Retained<Self>;
    );
}
