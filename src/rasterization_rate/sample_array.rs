use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
    runtime::NSObject,
};
use objc2_foundation::{NSNumber, NSObjectProtocol};

extern_class!(
    /// Helper object for convenient access to samples stored in an array.
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLRasterizationRateSampleArray;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLRasterizationRateSampleArray {}
);

impl MTLRasterizationRateSampleArray {
    extern_methods!(
        /// Retrieves the sample value at the specified index.
        #[unsafe(method(objectAtIndexedSubscript:))]
        #[unsafe(method_family = none)]
        pub unsafe fn object_at_indexed_subscript(&self, index: usize) -> Retained<NSNumber>;

        /// Stores a sample value at the specified index.
        #[unsafe(method(setObject:atIndexedSubscript:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_object_at_indexed_subscript(&self, value: &NSNumber, index: usize);
    );
}

/// Methods declared on superclass `NSObject`.
impl MTLRasterizationRateSampleArray {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
