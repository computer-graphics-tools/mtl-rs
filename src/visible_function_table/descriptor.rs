use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
    runtime::NSObject,
};
use objc2_foundation::{CopyingHelper, NSCopying, NSObjectProtocol};

extern_class!(
    /// Apple's documentation: `https://developer.apple.com/documentation/metal/mtlvisiblefunctiontabledescriptor?language=objc`
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLVisibleFunctionTableDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for MTLVisibleFunctionTableDescriptor {}
);

unsafe impl CopyingHelper for MTLVisibleFunctionTableDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLVisibleFunctionTableDescriptor {}
);

impl MTLVisibleFunctionTableDescriptor {
    extern_methods!(
        /// Create an autoreleased visible function table descriptor
        #[unsafe(method(visibleFunctionTableDescriptor))]
        #[unsafe(method_family = none)]
        pub unsafe fn visible_function_table_descriptor()
        -> Retained<MTLVisibleFunctionTableDescriptor>;

        /// The number of functions in the table.
        #[unsafe(method(functionCount))]
        #[unsafe(method_family = none)]
        pub fn function_count(&self) -> usize;

        /// Setter for [`function_count`][Self::function_count].
        #[unsafe(method(setFunctionCount:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_function_count(&self, function_count: usize);
    );
}

/// Methods declared on superclass `NSObject`.
impl MTLVisibleFunctionTableDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
