use objc2::{
    extern_class, extern_conformance, extern_methods, extern_protocol,
    rc::{Allocated, Retained},
    runtime::NSObject,
};
use objc2_foundation::{CopyingHelper, NSCopying, NSObjectProtocol};

extern_protocol!(
    /// Node used in a graph for stitching.
    ///
    /// Availability: macOS 12.0+, iOS 15.0+
    pub unsafe trait MTLFunctionStitchingNode: NSObjectProtocol + NSCopying {}
);

extern_class!(
    /// Indexed input node of the produced stitched function.
    ///
    /// Availability: macOS 12.0+, iOS 15.0+
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLFunctionStitchingInputNode;
);

extern_conformance!(
    unsafe impl MTLFunctionStitchingNode for MTLFunctionStitchingInputNode {}
);

extern_conformance!(
    unsafe impl NSCopying for MTLFunctionStitchingInputNode {}
);

unsafe impl CopyingHelper for MTLFunctionStitchingInputNode {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLFunctionStitchingInputNode {}
);

impl MTLFunctionStitchingInputNode {
    extern_methods!(
        #[unsafe(method(argumentIndex))]
        #[unsafe(method_family = none)]
        pub fn argument_index(&self) -> usize;

        /// Setter for [`argument_index`][Self::argument_index].
        #[unsafe(method(setArgumentIndex:))]
        #[unsafe(method_family = none)]
        pub fn set_argument_index(&self, argument_index: usize);

        #[unsafe(method(initWithArgumentIndex:))]
        #[unsafe(method_family = init)]
        pub fn init_with_argument_index(
            this: Allocated<Self>,
            argument: usize,
        ) -> Retained<Self>;
    );
}
