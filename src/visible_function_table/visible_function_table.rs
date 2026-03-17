use core::ops::Range;

use objc2::{Message, extern_protocol, msg_send, runtime::ProtocolObject};
use objc2_foundation::NSRange;

use crate::util::option_ref_ptr_cast_const;
use crate::{MTLFunctionHandle, MTLResource, MTLResourceID};

extern_protocol!(
    /// Apple's documentation: `https://developer.apple.com/documentation/metal/mtlvisiblefunctiontable?language=objc`
    pub unsafe trait MTLVisibleFunctionTable: MTLResource {
        /// Handle of the GPU resource suitable for storing in an Argument Buffer
        #[unsafe(method(gpuResourceID))]
        #[unsafe(method_family = none)]
        fn gpu_resource_id(&self) -> MTLResourceID;

        #[unsafe(method(setFunction:atIndex:))]
        #[unsafe(method_family = none)]
        fn set_function_at_index(
            &self,
            function: Option<&ProtocolObject<dyn MTLFunctionHandle>>,
            index: usize,
        );
    }
);

pub trait MTLVisibleFunctionTableExt: MTLVisibleFunctionTable + Message {
    /// Set an array of functions at the given index range.
    fn set_functions(
        &self,
        functions: &[Option<&ProtocolObject<dyn MTLFunctionHandle>>],
        range: Range<usize>,
    ) where
        Self: Sized,
    {
        let ptr = option_ref_ptr_cast_const(functions.as_ptr());
        unsafe { msg_send![self, setFunctions: ptr, withRange: NSRange::from(range)] }
    }
}

impl<T: MTLVisibleFunctionTable + Message> MTLVisibleFunctionTableExt for T {}
