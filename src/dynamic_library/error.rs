use objc2::{Encode, Encoding, RefEncode};
use objc2_foundation::NSErrorDomain;

/// Dynamic library error codes (ported from `MTLDynamicLibraryError`).
///
/// See Apple's documentation for `MTLDynamicLibraryError`.
#[repr(u64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum MTLDynamicLibraryError {
    None = 0,
    InvalidFile = 1,
    CompilationFailure = 2,
    UnresolvedInstallName = 3,
    DependencyLoadFailure = 4,
    Unsupported = 5,
}

unsafe impl Encode for MTLDynamicLibraryError {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for MTLDynamicLibraryError {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

unsafe extern "C" {
    static MTLDynamicLibraryDomain: &'static NSErrorDomain;
}

/// Returns the NSError domain for dynamic library errors emitted by Metal.
#[inline]
pub fn dynamic_library_error_domain() -> &'static NSErrorDomain {
    unsafe { MTLDynamicLibraryDomain }
}
