use objc2::{
    extern_class, extern_conformance, extern_methods, msg_send,
    rc::{Allocated, Retained},
    runtime::NSObject,
};
use objc2_foundation::{CopyingHelper, NSCopying, NSObjectProtocol, NSString};

extern_class!(
    /// Specifies the parameters for `ResidencySet` creation.
    ///
    /// Apple's documentation: `https://developer.apple.com/documentation/metal/mtlresidencysetdescriptor?language=objc`
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLResidencySetDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for MTLResidencySetDescriptor {}
);

unsafe impl CopyingHelper for MTLResidencySetDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLResidencySetDescriptor {}
);

impl MTLResidencySetDescriptor {
    extern_methods!(
        /// If non-zero, defines the number of allocations for which to initialize the internal arrays. Defaults to zero.
        #[unsafe(method(initialCapacity))]
        #[unsafe(method_family = none)]
        pub fn initial_capacity(&self) -> usize;

        /// Setter for [`initial_capacity`][Self::initial_capacity].
        #[unsafe(method(setInitialCapacity:))]
        #[unsafe(method_family = none)]
        pub fn set_initial_capacity(&self, initial_capacity: usize);
    );
}

/// Methods declared on superclass `NSObject`.
impl MTLResidencySetDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

#[allow(unused)]
impl MTLResidencySetDescriptor {
    /// Optional label for the residency set.
    fn label(&self) -> Option<String> {
        let s: Option<Retained<NSString>> = unsafe { msg_send![self, label] };
        s.map(|s| s.to_string())
    }

    /// Setter for the [`label`][Self::label].
    fn set_label(&self, label: Option<&str>) {
        unsafe {
            let _: () = msg_send![self, setLabel: label.map(NSString::from_str).as_deref()];
        }
    }
}
