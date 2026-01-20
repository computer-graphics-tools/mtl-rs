use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
    runtime::NSObject,
};
use objc2_foundation::NSObjectProtocol;

use super::MTLBufferLayoutDescriptor;

extern_class!(
    /// Array of buffer layout descriptors
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLBufferLayoutDescriptorArray;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLBufferLayoutDescriptorArray {}
);

impl MTLBufferLayoutDescriptorArray {
    extern_methods!(
        #[unsafe(method(objectAtIndexedSubscript:))]
        #[unsafe(method_family = none)]
        pub unsafe fn object_at_indexed_subscript(
            &self,
            index: usize,
        ) -> Retained<MTLBufferLayoutDescriptor>;

        #[unsafe(method(setObject:atIndexedSubscript:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_object_at_indexed_subscript(
            &self,
            buffer_desc: Option<&MTLBufferLayoutDescriptor>,
            index: usize,
        );
    );
}

/// Methods declared on superclass `NSObject`.
impl MTLBufferLayoutDescriptorArray {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
