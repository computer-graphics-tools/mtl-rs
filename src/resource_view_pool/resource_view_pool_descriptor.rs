use objc2::{
    extern_class, extern_conformance, extern_methods, msg_send,
    rc::{Allocated, Retained},
    runtime::NSObject,
};
use objc2_foundation::{CopyingHelper, NSCopying, NSObjectProtocol, NSString};

extern_class!(
    /// Parameters for creating a resource view pool.
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLResourceViewPoolDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for MTLResourceViewPoolDescriptor {}
);

unsafe impl CopyingHelper for MTLResourceViewPoolDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLResourceViewPoolDescriptor {}
);

impl MTLResourceViewPoolDescriptor {
    extern_methods!(
        /// Number of resource views with which Metal creates the resource view pool.
        #[unsafe(method(resourceViewCount))]
        #[unsafe(method_family = none)]
        pub fn resource_view_count(&self) -> usize;

        /// Setter for [`resource_view_count`][Self::resource_view_count].
        #[unsafe(method(setResourceViewCount:))]
        #[unsafe(method_family = none)]
        pub fn set_resource_view_count(&self, resource_view_count: usize);
    );

    pub fn label(&self) -> Option<String> {
        let label: Option<Retained<NSString>> = unsafe { msg_send![self, label] };
        label.map(|label| label.to_string())
    }

    pub fn set_label(&self, label: Option<&str>) {
        unsafe {
            let _: () = msg_send![self, setLabel: label.map(NSString::from_str).as_deref()];
        }
    }
}

/// Methods declared on superclass `NSObject`.
impl MTLResourceViewPoolDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub fn new() -> Retained<Self>;
    );
}
