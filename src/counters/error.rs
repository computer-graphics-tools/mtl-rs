use objc2::{Encode, Encoding, RefEncode};
use objc2_foundation::NSErrorDomain;

unsafe extern "C" {
    pub static MTLCounterErrorDomain: &'static NSErrorDomain;
}

#[inline]
pub fn counter_error_domain() -> &'static NSErrorDomain {
    unsafe { MTLCounterErrorDomain }
}

/// Errors when creating a counter sample buffer (from `MTLCounterSampleBufferError`).
#[repr(i64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[allow(unused)]
pub enum MTLCounterSampleBufferError {
    OutOfMemory = 0,
    Invalid = 1,
    Internal = 2,
}

unsafe impl Encode for MTLCounterSampleBufferError {
    const ENCODING: Encoding = i64::ENCODING;
}

unsafe impl RefEncode for MTLCounterSampleBufferError {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// Special resolved value written when a counter cannot be resolved.
#[allow(unused)]
pub const MTL_COUNTER_ERROR_VALUE: u64 = !0u64;

/// Special sample index value to indicate that a sample should not be taken.
#[allow(unused)]
pub const MTL_COUNTER_DONT_SAMPLE: usize = usize::MAX;


