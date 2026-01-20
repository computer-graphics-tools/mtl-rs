use objc2::{Encode, Encoding, RefEncode};

/// Enumerates the different support levels for sparse buffers (from `MTLBufferSparseTier`).
#[repr(i64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum MTLBufferSparseTier {
    /// Indicates that the buffer is not sparse.
    None = 0,
    /// Indicates support for sparse buffers tier 1.
    ///
    /// Tier 1 sparse buffers allow the following:
    /// - Partial memory backing at sparse page granularity.
    /// - Defined behavior for accessing an unbacked buffer range.
    ///
    /// An unbacked buffer range indicates a range within the buffer that doesn't
    /// have memory backing at a given point in time. Accessing an unbacked buffer
    /// range of a sparse buffer produces the following results:
    /// - Reading return zero.
    /// - Writing produces no result.
    Tier1 = 1,
}

unsafe impl Encode for MTLBufferSparseTier {
    const ENCODING: Encoding = i64::ENCODING;
}

unsafe impl RefEncode for MTLBufferSparseTier {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
