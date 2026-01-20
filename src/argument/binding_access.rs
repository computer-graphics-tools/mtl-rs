use objc2::{Encode, Encoding, RefEncode};

/// Read/write permissions for resource bindings.
///
/// Mirrors `MTLBindingAccess` from Metal.
#[repr(u64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum MTLBindingAccess {
    /// Read-only access.
    ReadOnly = 0,
    /// Read and write access.
    ReadWrite = 1,
    /// Write-only access.
    WriteOnly = 2,
}

unsafe impl Encode for MTLBindingAccess {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for MTLBindingAccess {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// Deprecated alias for `MTLBindingAccess`.
///
/// In Metal headers, `MTLArgumentAccess` is deprecated in favor of `MTLBindingAccess`.
pub type MTLArgumentAccess = MTLBindingAccess;
