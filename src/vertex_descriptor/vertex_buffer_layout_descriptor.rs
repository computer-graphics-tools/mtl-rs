use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
    runtime::NSObject,
};
use objc2_foundation::{CopyingHelper, NSCopying, NSObjectProtocol};

use super::vertex_step_function::MTLVertexStepFunction;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlvertexbufferlayoutdescriptor?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLVertexBufferLayoutDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for MTLVertexBufferLayoutDescriptor {}
);

unsafe impl CopyingHelper for MTLVertexBufferLayoutDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLVertexBufferLayoutDescriptor {}
);

impl MTLVertexBufferLayoutDescriptor {
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
        pub fn step_function(&self) -> MTLVertexStepFunction;

        /// Setter for [`stepFunction`][Self::stepFunction].
        #[unsafe(method(setStepFunction:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_step_function(&self, step_function: MTLVertexStepFunction);

        #[unsafe(method(stepRate))]
        #[unsafe(method_family = none)]
        pub fn step_rate(&self) -> usize;

        /// Setter for [`stepRate`][Self::stepRate].
        #[unsafe(method(setStepRate:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_step_rate(&self, step_rate: usize);
    );
}

/// Methods declared on superclass `NSObject`.
impl MTLVertexBufferLayoutDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub fn new() -> Retained<Self>;
    );
}
