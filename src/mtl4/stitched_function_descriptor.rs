use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
};
use objc2_foundation::{CopyingHelper, NSArray, NSCopying, NSObject, NSObjectProtocol};

use crate::{MTL4FunctionDescriptor, MTLFunctionStitchingGraph};

extern_class!(
    /// Groups together properties that describe a shader function suitable for stitching.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metal/mtl4stitchedfunctiondescriptor?language=objc)
    #[unsafe(super(MTL4FunctionDescriptor, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTL4StitchedFunctionDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for MTL4StitchedFunctionDescriptor {}
);

unsafe impl CopyingHelper for MTL4StitchedFunctionDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTL4StitchedFunctionDescriptor {}
);

impl MTL4StitchedFunctionDescriptor {
    extern_methods!(
        /// Sets the graph representing how to stitch functions together.
        #[unsafe(method(functionGraph))]
        #[unsafe(method_family = none)]
        pub unsafe fn function_graph(&self) -> Option<Retained<MTLFunctionStitchingGraph>>;

        /// Setter for [`functionGraph`][Self::functionGraph].
        ///
        /// This is [copied][objc2_foundation::NSCopying::copy] when set.
        #[unsafe(method(setFunctionGraph:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_function_graph(&self, function_graph: Option<&MTLFunctionStitchingGraph>);

        /// Configures an array of function descriptors with references to functions that contribute to the stitching process.
        #[unsafe(method(functionDescriptors))]
        #[unsafe(method_family = none)]
        pub unsafe fn function_descriptors(
            &self,
        ) -> Option<Retained<NSArray<MTL4FunctionDescriptor>>>;

        /// Setter for [`functionDescriptors`][Self::functionDescriptors].
        ///
        /// This is [copied][objc2_foundation::NSCopying::copy] when set.
        #[unsafe(method(setFunctionDescriptors:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_function_descriptors(
            &self,
            function_descriptors: Option<&NSArray<MTL4FunctionDescriptor>>,
        );
    );
}

/// Methods declared on superclass `NSObject`.
impl MTL4StitchedFunctionDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
