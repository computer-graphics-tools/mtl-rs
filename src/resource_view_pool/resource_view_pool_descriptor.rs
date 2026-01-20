use objc2::{
    extern_class, extern_conformance, extern_methods,
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

        /// Optional label for debugging purposes.
        #[unsafe(method(label))]
        #[unsafe(method_family = none)]
        pub fn label(&self) -> Option<Retained<NSString>>;

        /// Setter for [`label`][Self::label]. Copied when set.
        #[unsafe(method(setLabel:))]
        #[unsafe(method_family = none)]
        pub fn set_label(&self, label: Option<&NSString>);
    );
}

/// Methods declared on superclass `NSObject`.
impl MTLResourceViewPoolDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
