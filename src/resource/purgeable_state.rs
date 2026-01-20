use objc2::{Encode, Encoding, RefEncode};

/// Options for `set_purgeable_state` call (from `MTLPurgeableState`).
#[repr(u64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum MTLPurgeableState {
    /// The purgeability state is not changed.
    KeepCurrent = 1,
    /// The contents of this resource may not be discarded.
    NonVolatile = 2,
    /// The contents of this resource may be discarded.
    Volatile = 3,
    /// The contents of this resource are discarded.
    Empty = 4,
}

unsafe impl Encode for MTLPurgeableState {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for MTLPurgeableState {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
