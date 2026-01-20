use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
    runtime::NSObject,
};
use objc2_foundation::{CopyingHelper, NSCopying, NSObjectProtocol};

use super::{MTLCompareFunction, MTLStencilOperation};

extern_class!(
    /// Descriptor for stencil state properties.
    ///
    /// Availability: macOS 10.11+, iOS 8.0+
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLStencilDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for MTLStencilDescriptor {}
);

unsafe impl CopyingHelper for MTLStencilDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLStencilDescriptor {}
);

impl MTLStencilDescriptor {
    extern_methods!(
        /// Comparison function for stencil tests.
        #[unsafe(method(stencilCompareFunction))]
        #[unsafe(method_family = none)]
        pub fn stencil_compare_function(&self) -> MTLCompareFunction;

        #[unsafe(method(setStencilCompareFunction:))]
        #[unsafe(method_family = none)]
        pub fn set_stencil_compare_function(&self, value: MTLCompareFunction);

        /// Operation applied when the stencil test fails.
        #[unsafe(method(stencilFailureOperation))]
        #[unsafe(method_family = none)]
        pub fn stencil_failure_operation(&self) -> MTLStencilOperation;

        #[unsafe(method(setStencilFailureOperation:))]
        #[unsafe(method_family = none)]
        pub fn set_stencil_failure_operation(&self, value: MTLStencilOperation);

        /// Operation applied when depth test fails but stencil passed.
        #[unsafe(method(depthFailureOperation))]
        #[unsafe(method_family = none)]
        pub fn depth_failure_operation(&self) -> MTLStencilOperation;

        #[unsafe(method(setDepthFailureOperation:))]
        #[unsafe(method_family = none)]
        pub fn set_depth_failure_operation(&self, value: MTLStencilOperation);

        /// Operation applied when both stencil and depth tests pass.
        #[unsafe(method(depthStencilPassOperation))]
        #[unsafe(method_family = none)]
        pub fn depth_stencil_pass_operation(&self) -> MTLStencilOperation;

        #[unsafe(method(setDepthStencilPassOperation:))]
        #[unsafe(method_family = none)]
        pub fn set_depth_stencil_pass_operation(&self, value: MTLStencilOperation);

        /// Bitmask applied to both the stencil reference value and the stored stencil value when reading.
        #[unsafe(method(readMask))]
        #[unsafe(method_family = none)]
        pub fn read_mask(&self) -> u32;

        #[unsafe(method(setReadMask:))]
        #[unsafe(method_family = none)]
        pub fn set_read_mask(&self, value: u32);

        /// Bitmask applied to values written to the stencil buffer.
        #[unsafe(method(writeMask))]
        #[unsafe(method_family = none)]
        pub fn write_mask(&self) -> u32;

        #[unsafe(method(setWriteMask:))]
        #[unsafe(method_family = none)]
        pub fn set_write_mask(&self, value: u32);
    );
}

impl MTLStencilDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub fn new() -> Retained<Self>;
    );
}
