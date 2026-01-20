use objc2::{Encode, Encoding, RefEncode};
use objc2_foundation::NSErrorDomain;

unsafe extern "C" {
    /// Error domain for NSError objects produced by MTLCommandBuffer.
    pub static MTLCommandBufferErrorDomain: &'static NSErrorDomain;
}

#[inline]
pub fn command_buffer_error_domain() -> &'static NSErrorDomain {
    unsafe { MTLCommandBufferErrorDomain }
}

/// Error codes that can be found in MTLCommandBuffer.error.
///
/// Availability: macOS 10.11+, iOS 8.0+
#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[allow(unused)]
pub enum MTLCommandBufferError {
    None = 0,
    Internal = 1,
    Timeout = 2,
    PageFault = 3,
    AccessRevoked = 4,
    NotPermitted = 7,
    OutOfMemory = 8,
    InvalidResource = 9,
    Memoryless = 10,
    DeviceRemoved = 11,
    StackOverflow = 12,
}

unsafe impl Encode for MTLCommandBufferError {
    const ENCODING: Encoding = u64::ENCODING;
}
unsafe impl RefEncode for MTLCommandBufferError {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub struct MTLCommandBufferErrorOption: u64 { const None = 0; const EncoderExecutionStatus = 1<<0; }
}
unsafe impl Encode for MTLCommandBufferErrorOption {
    const ENCODING: Encoding = u64::ENCODING;
}
unsafe impl RefEncode for MTLCommandBufferErrorOption {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
