use objc2::{
    extern_class, extern_conformance, extern_methods, msg_send,
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
        pub fn function_graph(&self) -> Option<Retained<MTLFunctionStitchingGraph>>;

        /// Setter for [`functionGraph`][Self::functionGraph].
        ///
        /// This is [copied][objc2_foundation::NSCopying::copy] when set.
        #[unsafe(method(setFunctionGraph:))]
        #[unsafe(method_family = none)]
        pub fn set_function_graph(
            &self,
            function_graph: Option<&MTLFunctionStitchingGraph>,
        );
    );

    /// Function descriptors that contribute to the stitching process.
    pub fn function_descriptors(&self) -> Option<Box<[Retained<MTL4FunctionDescriptor>]>> {
        let descriptors: Option<Retained<NSArray<MTL4FunctionDescriptor>>> =
            unsafe { msg_send![self, functionDescriptors] };
        descriptors.map(|descriptors| descriptors.to_vec().into_boxed_slice())
    }

    /// Sets the function descriptors with copy semantics.
    pub fn set_function_descriptors(
        &self,
        function_descriptors: Option<&[&MTL4FunctionDescriptor]>,
    ) {
        let function_descriptors = function_descriptors.map(NSArray::from_slice);
        unsafe {
            let _: () = msg_send![self, setFunctionDescriptors: function_descriptors.as_deref()];
        }
    }
}

/// Methods declared on superclass `NSObject`.
impl MTL4StitchedFunctionDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub fn new() -> Retained<Self>;
    );
}
