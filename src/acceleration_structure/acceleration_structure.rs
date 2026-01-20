use objc2::extern_protocol;
use objc2_foundation::NSObjectProtocol;

use crate::{MTLResource, MTLResourceID};

extern_protocol!(
    /// Minimal wrapper for `MTLAccelerationStructure`.
    pub unsafe trait MTLAccelerationStructure: MTLResource + NSObjectProtocol {
        /// Size of the acceleration structure in bytes.
        #[unsafe(method(size))]
        #[unsafe(method_family = none)]
        unsafe fn size(&self) -> usize;

        /// Handle of the GPU resource suitable for storing in an Argument Buffer.
        #[unsafe(method(gpuResourceID))]
        #[unsafe(method_family = none)]
        unsafe fn gpu_resource_id(&self) -> MTLResourceID;
    }
);
