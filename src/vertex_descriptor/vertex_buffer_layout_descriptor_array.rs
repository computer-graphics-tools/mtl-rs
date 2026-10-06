use objc2::{
    extern_class, extern_conformance, extern_methods, msg_send,
    rc::{Allocated, Retained},
    runtime::NSObject,
};
use objc2_foundation::NSObjectProtocol;

use super::vertex_buffer_layout_descriptor::MTLVertexBufferLayoutDescriptor;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlvertexbufferlayoutdescriptorarray?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLVertexBufferLayoutDescriptorArray;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLVertexBufferLayoutDescriptorArray {}
);

impl MTLVertexBufferLayoutDescriptorArray {
    /// Returns the vertex buffer layout descriptor at `index`.
    pub fn get(
        &self,
        index: usize,
    ) -> Retained<MTLVertexBufferLayoutDescriptor> {
        unsafe { msg_send![self, objectAtIndexedSubscript: index] }
    }
    /// Sets the vertex buffer layout descriptor at `index`; `None` resets it to default values.
    pub fn set(
        &self,
        index: usize,
        layout: Option<&MTLVertexBufferLayoutDescriptor>,
    ) {
        unsafe {
            let _: () = msg_send![self, setObject: layout, atIndexedSubscript: index];
        }
    }
}

/// Methods declared on superclass `NSObject`.
impl MTLVertexBufferLayoutDescriptorArray {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub fn new() -> Retained<Self>;
    );
}
