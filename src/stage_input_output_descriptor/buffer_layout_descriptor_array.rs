use objc2::{
    extern_class, extern_conformance, extern_methods, msg_send,
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
    /// Returns the buffer layout descriptor at `index`.
    pub fn get(
        &self,
        index: usize,
    ) -> Retained<MTLBufferLayoutDescriptor> {
        unsafe { msg_send![self, objectAtIndexedSubscript: index] }
    }
    /// Sets the buffer layout descriptor at `index`; `None` resets it to default values.
    pub fn set(
        &self,
        index: usize,
        layout: Option<&MTLBufferLayoutDescriptor>,
    ) {
        unsafe {
            let _: () = msg_send![self, setObject: layout, atIndexedSubscript: index];
        }
    }
}

/// Methods declared on superclass `NSObject`.
impl MTLBufferLayoutDescriptorArray {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub fn new() -> Retained<Self>;
    );
}
