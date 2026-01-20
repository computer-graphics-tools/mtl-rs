use core::ptr::NonNull;

use objc2::{extern_protocol, runtime::ProtocolObject};
use objc2_foundation::NSRange;

use crate::{MTLFunctionHandle, MTLResource, MTLResourceID};

extern_protocol!(
    /// Apple's documentation: `https://developer.apple.com/documentation/metal/mtlvisiblefunctiontable?language=objc`
    pub unsafe trait MTLVisibleFunctionTable: MTLResource {
        /// Handle of the GPU resource suitable for storing in an Argument Buffer
        #[unsafe(method(gpuResourceID))]
        #[unsafe(method_family = none)]
        unsafe fn gpu_resource_id(&self) -> MTLResourceID;

        #[unsafe(method(setFunction:atIndex:))]
        #[unsafe(method_family = none)]
        unsafe fn set_function_at_index(
            &self,
            function: Option<&ProtocolObject<dyn MTLFunctionHandle>>,
            index: usize,
        );

        /// Safety: `functions` must be a valid pointer.
        #[unsafe(method(setFunctions:withRange:))]
        #[unsafe(method_family = none)]
        unsafe fn set_functions_with_range(
            &self,
            functions: NonNull<*const ProtocolObject<dyn MTLFunctionHandle>>,
            range: NSRange,
        );
    }
);
