use objc2::{extern_class, extern_conformance, extern_methods, rc::Retained, runtime::NSObject};
use objc2_foundation::{CopyingHelper, NSCopying, NSObjectProtocol};

extern_class!(
    /// Intersection function table descriptor
    ///
    /// Availability: macOS 11.0+, iOS 14.0+, tvOS 16.0+
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLIntersectionFunctionTableDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for MTLIntersectionFunctionTableDescriptor {}
);

unsafe impl CopyingHelper for MTLIntersectionFunctionTableDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLIntersectionFunctionTableDescriptor {}
);

impl MTLIntersectionFunctionTableDescriptor {
    extern_methods!(
        /// Create a descriptor
        ///
        /// Availability: macOS 11.0+, iOS 14.0+, tvOS 16.0+
        #[unsafe(method(intersectionFunctionTableDescriptor))]
        #[unsafe(method_family = none)]
        pub fn intersection_function_table_descriptor()
        -> Retained<MTLIntersectionFunctionTableDescriptor>;

        /// The number of functions in the table.
        #[unsafe(method(functionCount))]
        #[unsafe(method_family = none)]
        pub fn function_count(&self) -> usize;

        /// Setter for [`function_count`][Self::function_count].
        #[unsafe(method(setFunctionCount:))]
        #[unsafe(method_family = none)]
        pub fn set_function_count(&self, function_count: usize);
    );
}
