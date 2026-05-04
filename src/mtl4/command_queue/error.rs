use objc2::encode::{Encode, Encoding, RefEncode};
use objc2_foundation::{NSErrorDomain};

/// Enumeration of kinds of errors that committing an array of command buffers instances can produce.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/metal/mtl4commandqueueerror?language=objc)
/// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTL4CommandQueueError(pub isize);
impl MTL4CommandQueueError {
    /// Indicates the absence of any problems.
    #[doc(alias = "MTL4CommandQueueErrorNone")]
    pub const NONE: Self = Self(0);
    /// Indicates the workload takes longer to execute than the system allows.
    #[doc(alias = "MTL4CommandQueueErrorTimeout")]
    pub const TIMEOUT: Self = Self(1);
    /// Indicates a process doesn’t have access to a GPU device.
    #[doc(alias = "MTL4CommandQueueErrorNotPermitted")]
    pub const NOT_PERMITTED: Self = Self(2);
    /// Indicates the GPU doesn’t have sufficient memory to execute a command buffer.
    #[doc(alias = "MTL4CommandQueueErrorOutOfMemory")]
    pub const OUT_OF_MEMORY: Self = Self(3);
    /// Indicates the physical removal of the GPU before the command buffer completed.
    #[doc(alias = "MTL4CommandQueueErrorDeviceRemoved")]
    pub const DEVICE_REMOVED: Self = Self(4);
    /// Indicates that the system revokes GPU access because it’s responsible for too many timeouts or hangs.
    #[doc(alias = "MTL4CommandQueueErrorAccessRevoked")]
    pub const ACCESS_REVOKED: Self = Self(5);
    /// Indicates an internal problem in the Metal framework.
    #[doc(alias = "MTL4CommandQueueErrorInternal")]
    pub const INTERNAL: Self = Self(6);
}

unsafe impl Encode for MTL4CommandQueueError {
    const ENCODING: Encoding = isize::ENCODING;
}

unsafe impl RefEncode for MTL4CommandQueueError {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

unsafe extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtl4commandqueueerrordomain?language=objc)
    pub static MTL4CommandQueueErrorDomain: &'static NSErrorDomain;
}
