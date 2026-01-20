use super::MTLFunctionDescriptor;
use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
    runtime::NSObject,
};
use objc2_foundation::{CopyingHelper, NSCopying, NSObjectProtocol};

extern_class!(
    /// Descriptor for an intersection function (subclass of `FunctionDescriptor`).
    #[unsafe(super(MTLFunctionDescriptor, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLIntersectionFunctionDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for MTLIntersectionFunctionDescriptor {}
);

unsafe impl CopyingHelper for MTLIntersectionFunctionDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLIntersectionFunctionDescriptor {}
);

impl MTLIntersectionFunctionDescriptor {
    extern_methods!();
}

/// Methods declared on superclass `NSObject`.
impl MTLIntersectionFunctionDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
