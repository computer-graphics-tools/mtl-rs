use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
    runtime::NSObject,
};
use objc2_foundation::{CopyingHelper, NSCopying, NSObjectProtocol};

use super::MTLMutability;

extern_class!(
    /// Apple's docs: `https://developer.apple.com/documentation/metal/mtlpipelinebufferdescriptor?language=objc`
    ///
    /// Availability: macOS 10.13+, iOS 11.0+
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLPipelineBufferDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for MTLPipelineBufferDescriptor {}
);

unsafe impl CopyingHelper for MTLPipelineBufferDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLPipelineBufferDescriptor {}
);

impl MTLPipelineBufferDescriptor {
    extern_methods!(
        /// Buffer mutability. Defaults to Mutability::Default: mutable for standard buffers, immutable for argument buffers
        #[unsafe(method(mutability))]
        #[unsafe(method_family = none)]
        pub fn mutability(&self) -> MTLMutability;

        /// Setter for [`mutability`][Self::mutability].
        #[unsafe(method(setMutability:))]
        #[unsafe(method_family = none)]
        pub fn set_mutability(&self, mutability: MTLMutability);
    );
}

/// Methods declared on superclass `NSObject`.
impl MTLPipelineBufferDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
