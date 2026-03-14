use objc2::{
    extern_class, extern_conformance, extern_methods, msg_send,
    rc::{Allocated, Retained},
    runtime::{NSObject, ProtocolObject},
};
use objc2_foundation::{CopyingHelper, NSArray, NSCopying, NSObjectProtocol};

use super::{MTLFunctionStitchingGraph, MTLStitchedLibraryOptions};
use crate::{MTLBinaryArchive, library::MTLFunction};

extern_class!(
    /// Container for the graphs and functions needed to create stitched functions.
    ///
    /// Availability: macOS 12.0+, iOS 15.0+
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLStitchedLibraryDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for MTLStitchedLibraryDescriptor {}
);

unsafe impl CopyingHelper for MTLStitchedLibraryDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLStitchedLibraryDescriptor {}
);

impl MTLStitchedLibraryDescriptor {
    extern_methods!(
        /// Options for creating the stitched library.
        ///
        /// Availability: macOS 15.0+, iOS 18.0+
        #[unsafe(method(options))]
        #[unsafe(method_family = none)]
        pub fn options(&self) -> MTLStitchedLibraryOptions;

        /// Setter for [`options`][Self::options].
        #[unsafe(method(setOptions:))]
        #[unsafe(method_family = none)]
        pub fn set_options(
            &self,
            options: MTLStitchedLibraryOptions,
        );
    );

    pub fn function_graphs(&self) -> Box<[Retained<MTLFunctionStitchingGraph>]> {
        let function_graphs: Retained<NSArray<MTLFunctionStitchingGraph>> = unsafe { msg_send![self, functionGraphs] };
        function_graphs.to_vec().into_boxed_slice()
    }

    pub fn set_function_graphs(
        &self,
        function_graphs: &[&MTLFunctionStitchingGraph],
    ) {
        let function_graphs = NSArray::from_slice(function_graphs);
        unsafe {
            let _: () = msg_send![self, setFunctionGraphs: &*function_graphs];
        }
    }

    /// Functions referenced by the graphs.
    pub fn functions(&self) -> Box<[Retained<ProtocolObject<dyn MTLFunction>>]> {
        let functions: Retained<NSArray<ProtocolObject<dyn MTLFunction>>> = unsafe { msg_send![self, functions] };
        functions.to_vec().into_boxed_slice()
    }

    pub fn set_functions(
        &self,
        functions: &[&ProtocolObject<dyn MTLFunction>],
    ) {
        let functions = NSArray::from_slice(functions);
        unsafe {
            let _: () = msg_send![self, setFunctions: &*functions];
        }
    }

    /// Binary archives to be searched for precompiled stitched libraries during compilation.
    ///
    /// Availability: macOS 15.0+, iOS 18.0+
    pub fn binary_archives(&self) -> Box<[Retained<ProtocolObject<dyn MTLBinaryArchive>>]> {
        let binary_archives: Retained<NSArray<ProtocolObject<dyn MTLBinaryArchive>>> =
            unsafe { msg_send![self, binaryArchives] };
        binary_archives.to_vec().into_boxed_slice()
    }

    /// Availability: macOS 15.0+, iOS 18.0+
    pub fn set_binary_archives(
        &self,
        binary_archives: &[&ProtocolObject<dyn MTLBinaryArchive>],
    ) {
        let binary_archives = NSArray::from_slice(binary_archives);
        unsafe {
            let _: () = msg_send![self, setBinaryArchives: &*binary_archives];
        }
    }
}

/// Methods declared on superclass `NSObject`.
impl MTLStitchedLibraryDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub fn new() -> Retained<Self>;
    );
}
