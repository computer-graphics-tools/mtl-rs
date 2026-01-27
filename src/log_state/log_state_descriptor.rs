use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
    runtime::NSObject,
};
use objc2_foundation::{CopyingHelper, NSCopying, NSObjectProtocol};

use crate::MTLLogLevel;

extern_class!(
    /// Descriptor for creating a `LogState`.
    ///
    /// Availability: macOS 15.0+, iOS 18.0+
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLLogStateDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for MTLLogStateDescriptor {}
);

unsafe impl CopyingHelper for MTLLogStateDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLLogStateDescriptor {}
);

impl MTLLogStateDescriptor {
    extern_methods!(
        /// Minimum level of logs that will be printed.
        #[unsafe(method(level))]
        #[unsafe(method_family = none)]
        pub fn level(&self) -> MTLLogLevel;

        #[unsafe(method(setLevel:))]
        #[unsafe(method_family = none)]
        pub fn set_level(&self, level: MTLLogLevel);

        /// Size of the GPU buffer for shader logging (minimum 1KB).
        #[unsafe(method(bufferSize))]
        #[unsafe(method_family = none)]
        pub fn buffer_size(&self) -> isize;

        #[unsafe(method(setBufferSize:))]
        #[unsafe(method_family = none)]
        pub fn set_buffer_size(&self, buffer_size: isize);
    );
}

impl MTLLogStateDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub fn new() -> Retained<Self>;
    );
}
