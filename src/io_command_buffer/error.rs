use objc2::{Encode, Encoding, RefEncode};
use objc2_foundation::NSErrorDomain;

/// IO error codes (ported from `MTLIOError`).
#[repr(i64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum MTLIOError {
    UrlInvalid = 1,
    Internal = 2,
}

unsafe impl Encode for MTLIOError {
    const ENCODING: Encoding = i64::ENCODING;
}

unsafe impl RefEncode for MTLIOError {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

unsafe extern "C" {
    static MTLIOErrorDomain: &'static NSErrorDomain;
}

/// Returns the NSError domain for I/O errors emitted by Metal.
pub fn io_error_domain() -> &'static NSErrorDomain {
    unsafe { MTLIOErrorDomain }
}
