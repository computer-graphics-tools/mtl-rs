use objc2::{
    Message, extern_class, extern_conformance, extern_methods, msg_send,
    rc::{Allocated, Retained},
    runtime::NSObject,
};
use objc2_foundation::{CopyingHelper, NSCopying, NSObjectProtocol, NSString};

use super::{MTLCompareFunction, MTLStencilDescriptor};

extern_class!(
    /// Descriptor for creating a `DepthStencilState`.
    ///
    /// Availability: macOS 10.11+, iOS 8.0+
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLDepthStencilDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for MTLDepthStencilDescriptor {}
);

unsafe impl CopyingHelper for MTLDepthStencilDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLDepthStencilDescriptor {}
);

impl MTLDepthStencilDescriptor {
    extern_methods!(
        /// Depth compare function. Defaults to `Always`, which effectively skips the depth test.
        #[unsafe(method(depthCompareFunction))]
        #[unsafe(method_family = none)]
        pub fn depth_compare_function(&self) -> MTLCompareFunction;

        #[unsafe(method(setDepthCompareFunction:))]
        #[unsafe(method_family = none)]
        pub fn set_depth_compare_function(&self, value: MTLCompareFunction);

        /// Whether depth writes are performed. Defaults to `false`.
        #[unsafe(method(isDepthWriteEnabled))]
        #[unsafe(method_family = none)]
        pub fn is_depth_write_enabled(&self) -> bool;

        #[unsafe(method(setDepthWriteEnabled:))]
        #[unsafe(method_family = none)]
        pub fn set_depth_write_enabled(&self, value: bool);

        /// Separate stencil state for front face.
        #[unsafe(method(frontFaceStencil))]
        #[unsafe(method_family = none)]
        pub fn front_face_stencil(&self) -> Retained<MTLStencilDescriptor>;

        #[unsafe(method(setFrontFaceStencil:))]
        #[unsafe(method_family = none)]
        pub fn set_front_face_stencil(&self, value: Option<&MTLStencilDescriptor>);

        /// Separate stencil state for back face.
        #[unsafe(method(backFaceStencil))]
        #[unsafe(method_family = none)]
        pub fn back_face_stencil(&self) -> Retained<MTLStencilDescriptor>;

        #[unsafe(method(setBackFaceStencil:))]
        #[unsafe(method_family = none)]
        pub fn set_back_face_stencil(&self, value: Option<&MTLStencilDescriptor>);
    );
}

impl MTLDepthStencilDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub fn new() -> Retained<Self>;
    );
}

#[allow(unused)]
pub trait MTLDepthStencilDescriptorExt: Message {
    fn label(&self) -> Option<String>;
    fn set_label(&self, label: Option<&str>);
}

impl MTLDepthStencilDescriptorExt for MTLDepthStencilDescriptor {
    fn label(&self) -> Option<String> {
        let label: Option<Retained<NSString>> = unsafe { msg_send![self, label] };
        label.map(|s| s.to_string())
    }

    fn set_label(&self, label: Option<&str>) {
        unsafe {
            let _: () = msg_send![self, setLabel: label.map(NSString::from_str).as_deref()];
        }
    }
}
