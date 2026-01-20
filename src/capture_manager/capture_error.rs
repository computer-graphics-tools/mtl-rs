use objc2::{Encode, Encoding, RefEncode};
use objc2_foundation::NSErrorDomain;

/// Errors that capture APIs can return.
///
/// Availability: macOS 10.15+, iOS 13.0+
#[repr(i64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MTLCaptureError {
    /// Capturing is not supported, maybe the destination is not supported.
    NotSupported = 1,
    /// A capture is already in progress.
    AlreadyCapturing = 2,
    /// The MTLCaptureDescriptor contains invalid parameters.
    InvalidDescriptor = 3,
}

unsafe impl Encode for MTLCaptureError {
    const ENCODING: Encoding = i64::ENCODING;
}
unsafe impl RefEncode for MTLCaptureError {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

unsafe extern "C" {
    /// NSError domain for capture-related errors.
    static MTLCaptureErrorDomain: &'static NSErrorDomain;
}

#[inline]
pub fn capture_error_domain() -> &'static NSErrorDomain {
    unsafe { MTLCaptureErrorDomain }
}
