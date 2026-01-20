use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
    runtime::NSObject,
};
use objc2_foundation::{CopyingHelper, NSCopying, NSObjectProtocol};

use crate::MTLAccelerationStructureUsage;

extern_class!(
    /// Base class for acceleration structure descriptors. Do not use this class directly. Use
    /// one of the derived classes instead.
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLAccelerationStructureDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for MTLAccelerationStructureDescriptor {}
);

unsafe impl CopyingHelper for MTLAccelerationStructureDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLAccelerationStructureDescriptor {}
);

impl MTLAccelerationStructureDescriptor {
    extern_methods!(
        #[unsafe(method(usage))]
        #[unsafe(method_family = none)]
        pub unsafe fn usage(&self) -> MTLAccelerationStructureUsage;

        /// Setter for [`usage`][Self::usage].
        #[unsafe(method(setUsage:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_usage(&self, usage: MTLAccelerationStructureUsage);
    );
}

/// Methods declared on superclass `NSObject`.
impl MTLAccelerationStructureDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
