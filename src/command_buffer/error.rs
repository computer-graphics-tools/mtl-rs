use objc2::{Encode, Encoding, RefEncode};
use objc2_foundation::{NSErrorDomain, NSErrorUserInfoKey};

unsafe extern "C" {
    /// Error domain for NSError objects produced by MTLCommandBuffer.
    static MTLCommandBufferErrorDomain: &'static NSErrorDomain;

    /// Key whose value contains encoder execution details in a command-buffer error.
    static MTLCommandBufferEncoderInfoErrorKey: &'static NSErrorUserInfoKey;
}

#[inline]
pub fn command_buffer_error_domain() -> String {
    unsafe { MTLCommandBufferErrorDomain }.to_string()
}

/// Returns the user-info key for command-buffer encoder execution details.
#[inline]
pub fn command_buffer_encoder_info_error_key() -> String {
    unsafe { MTLCommandBufferEncoderInfoErrorKey }.to_string()
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
    #[deprecated(note = "this error cannot occur on Apple Silicon")]
    DeviceRemoved = 11,
    StackOverflow = 12,
}

impl MTLCommandBufferError {
    /// Deprecated name for [`Self::AccessRevoked`].
    #[allow(non_upper_case_globals)]
    #[deprecated(note = "use AccessRevoked")]
    pub const Blacklisted: Self = Self::AccessRevoked;
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
