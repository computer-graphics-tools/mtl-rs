use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
};
use objc2_foundation::{CopyingHelper, NSCopying, NSObject, NSObjectProtocol};

use crate::*;

extern_class!(
    /// Base class for Metal 4 acceleration structure descriptors.
    ///
    /// Don't use this class directly. Use one of its subclasses instead.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metal/mtl4accelerationstructuredescriptor?language=objc)
    #[unsafe(super(MTLAccelerationStructureDescriptor, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTL4AccelerationStructureDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for MTL4AccelerationStructureDescriptor {}
);

unsafe impl CopyingHelper for MTL4AccelerationStructureDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTL4AccelerationStructureDescriptor {}
);

impl MTL4AccelerationStructureDescriptor {
    extern_methods!();
}

/// Methods declared on superclass `NSObject`.
impl MTL4AccelerationStructureDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
