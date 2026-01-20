use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, DefaultRetained, Retained},
    runtime::{NSObject, ProtocolObject},
};
use objc2_foundation::{
    CopyingHelper, NSArray, NSCopying, NSDictionary, NSObjectProtocol, NSString,
};

use crate::library::MTLFunction;

extern_class!(
    /// Functions to be linked into a pipeline.
    ///
    /// Availability: macOS 11.0+, iOS 14.0+
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLLinkedFunctions;
);

extern_conformance!(
    unsafe impl NSCopying for MTLLinkedFunctions {}
);

unsafe impl CopyingHelper for MTLLinkedFunctions {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLLinkedFunctions {}
);

impl MTLLinkedFunctions {
    extern_methods!(
        /// Create an autoreleased LinkedFunctions object.
        ///
        /// Availability: macOS 11.0+, iOS 14.0+
        #[unsafe(method(linkedFunctions))]
        #[unsafe(method_family = none)]
        pub fn linked_functions() -> Retained<MTLLinkedFunctions>;

        /// Array of functions to be AIR linked.
        #[unsafe(method(functions))]
        #[unsafe(method_family = none)]
        pub fn functions(&self) -> Option<Retained<NSArray<ProtocolObject<dyn MTLFunction>>>>;

        /// Setter for [`functions`][Self::functions]. Copied when set.
        #[unsafe(method(setFunctions:))]
        #[unsafe(method_family = none)]
        pub fn set_functions(&self, functions: Option<&NSArray<ProtocolObject<dyn MTLFunction>>>);

        /// Array of functions compiled to binary to be linked.
        ///
        /// Availability: macOS 11.0+, iOS 14.0+, tvOS 16.0+
        #[unsafe(method(binaryFunctions))]
        #[unsafe(method_family = none)]
        pub fn binary_functions(
            &self,
        ) -> Option<Retained<NSArray<ProtocolObject<dyn MTLFunction>>>>;

        /// Setter for [`binary_functions`][Self::binary_functions]. Copied when set.
        #[unsafe(method(setBinaryFunctions:))]
        #[unsafe(method_family = none)]
        pub fn set_binary_functions(
            &self,
            binary_functions: Option<&NSArray<ProtocolObject<dyn MTLFunction>>>,
        );

        /// Groups of functions, keyed by callsite name.
        #[unsafe(method(groups))]
        #[unsafe(method_family = none)]
        pub fn groups(
            &self,
        ) -> Option<Retained<NSDictionary<NSString, NSArray<ProtocolObject<dyn MTLFunction>>>>>;

        /// Setter for [`groups`][Self::groups]. Copied when set.
        #[unsafe(method(setGroups:))]
        #[unsafe(method_family = none)]
        pub fn set_groups(
            &self,
            groups: Option<&NSDictionary<NSString, NSArray<ProtocolObject<dyn MTLFunction>>>>,
        );

        /// Private functions to be AIR linked but not exported as function handles.
        ///
        /// Availability: macOS 12.0+, iOS 15.0+
        #[unsafe(method(privateFunctions))]
        #[unsafe(method_family = none)]
        pub fn private_functions(
            &self,
        ) -> Option<Retained<NSArray<ProtocolObject<dyn MTLFunction>>>>;

        /// Setter for [`private_functions`][Self::private_functions]. Copied when set.
        #[unsafe(method(setPrivateFunctions:))]
        #[unsafe(method_family = none)]
        pub fn set_private_functions(
            &self,
            private_functions: Option<&NSArray<ProtocolObject<dyn MTLFunction>>>,
        );
    );
}

impl MTLLinkedFunctions {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub fn new() -> Retained<Self>;
    );
}

impl DefaultRetained for MTLLinkedFunctions {
    #[inline]
    fn default_retained() -> Retained<Self> {
        Self::new()
    }
}
