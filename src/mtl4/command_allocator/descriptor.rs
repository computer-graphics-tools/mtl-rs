use objc2::{
    extern_class, extern_conformance, extern_methods, msg_send,
    rc::{Allocated, Retained},
};
use objc2_foundation::{CopyingHelper, NSCopying, NSObject, NSObjectProtocol, NSString};

extern_class!(
    /// Groups together parameters for creating a command allocator.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metal/mtl4commandallocatordescriptor?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTL4CommandAllocatorDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for MTL4CommandAllocatorDescriptor {}
);

unsafe impl CopyingHelper for MTL4CommandAllocatorDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTL4CommandAllocatorDescriptor {}
);

impl MTL4CommandAllocatorDescriptor {
    extern_methods!();
}

impl MTL4CommandAllocatorDescriptor {
    /// An optional label you can assign to the command allocator to aid debugging.
    pub fn label(&self) -> Option<String> {
        let s: Option<Retained<NSString>> = unsafe { msg_send![self, label] };
        s.map(|v| v.to_string())
    }

    /// Setter for [`label`][Self::label].
    pub fn set_label(&self, label: Option<&str>) {
        unsafe {
            let _: () = msg_send![self, setLabel: label.map(NSString::from_str).as_deref()];
        }
    }
}

/// Methods declared on superclass `NSObject`.
impl MTL4CommandAllocatorDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub fn new() -> Retained<Self>;
    );
}
