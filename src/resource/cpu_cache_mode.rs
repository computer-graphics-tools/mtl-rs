use objc2::{Encode, Encoding, RefEncode};

/// Describes what CPU cache mode is used for the CPU's mapping of a resource (from `MTLCPUCacheMode`).
#[repr(u64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum MTLCPUCacheMode {
    /// The default cache mode for the system.
    DefaultCache = 0,
    /// Write-combined memory is optimized for resources that the CPU will write into, but never read.
    /// On some implementations, writes may bypass caches avoiding cache pollution, and reads perform very poorly.
    WriteCombined = 1,
}

unsafe impl Encode for MTLCPUCacheMode {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for MTLCPUCacheMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
