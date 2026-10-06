use objc2::{
    extern_class, extern_conformance, extern_methods, msg_send,
    rc::{Allocated, Retained},
    runtime::NSObject,
};
use objc2_foundation::NSObjectProtocol;

use super::MTLAttributeDescriptor;

extern_class!(
    /// Array of attribute descriptors
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLAttributeDescriptorArray;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLAttributeDescriptorArray {}
);

impl MTLAttributeDescriptorArray {
    /// Returns the attribute descriptor at `index`.
    pub fn get(
        &self,
        index: usize,
    ) -> Retained<MTLAttributeDescriptor> {
        unsafe { msg_send![self, objectAtIndexedSubscript: index] }
    }
    /// Sets the attribute descriptor at `index`; `None` resets it to default values.
    pub fn set(
        &self,
        index: usize,
        attribute: Option<&MTLAttributeDescriptor>,
    ) {
        unsafe {
            let _: () = msg_send![self, setObject: attribute, atIndexedSubscript: index];
        }
    }
}

/// Methods declared on superclass `NSObject`.
impl MTLAttributeDescriptorArray {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub fn new() -> Retained<Self>;
    );
}
