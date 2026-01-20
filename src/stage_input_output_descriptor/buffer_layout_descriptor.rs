use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
    runtime::NSObject,
};
use objc2_foundation::{CopyingHelper, NSCopying, NSObjectProtocol};

use super::MTLStepFunction;

extern_class!(
    /// Buffer layout descriptor
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLBufferLayoutDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for MTLBufferLayoutDescriptor {}
);

unsafe impl CopyingHelper for MTLBufferLayoutDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLBufferLayoutDescriptor {}
);

impl MTLBufferLayoutDescriptor {
    extern_methods!(
        #[unsafe(method(stride))]
        #[unsafe(method_family = none)]
        pub fn stride(&self) -> usize;

        /// Setter for [`stride`][Self::stride].
        #[unsafe(method(setStride:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_stride(&self, stride: usize);

        #[unsafe(method(stepFunction))]
        #[unsafe(method_family = none)]
        pub fn step_function(&self) -> MTLStepFunction;

        /// Setter for [`step_function`][Self::step_function].
        #[unsafe(method(setStepFunction:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_step_function(&self, step_function: MTLStepFunction);

        #[unsafe(method(stepRate))]
        #[unsafe(method_family = none)]
        pub fn step_rate(&self) -> usize;

        /// Setter for [`step_rate`][Self::step_rate].
        #[unsafe(method(setStepRate:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_step_rate(&self, step_rate: usize);
    );
}

/// Methods declared on superclass `NSObject`.
impl MTLBufferLayoutDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
