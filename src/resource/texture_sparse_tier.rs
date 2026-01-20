use objc2::{Encode, Encoding, RefEncode};

/// Enumerates the different support levels for sparse textures (from `MTLTextureSparseTier`).
#[repr(i64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum MTLTextureSparseTier {
    /// Indicates that the texture is not sparse.
    None = 0,
    /// Indicates support for sparse textures tier 1.
    ///
    /// Tier 1 sparse textures allow the following:
    /// - Partial memory backing at sparse tile granularity.
    /// - Defined behavior for accessing an unbacked texture region.
    /// - Shader feedback on texture access to determine memory backing.
    ///
    /// An unbacked texture region indicates a region within the texture that doesn't
    /// have memory backing at a given point in time. Accessing an unbacked texture
    /// region produces the following results:
    /// - Reading returns zero (transparent black) for pixel formats with an alpha (A) channel.
    /// - Reading returns zero in RGB and one in alpha (A) channels (opaque black) otherwise.
    /// - Writing produces no result.
    Tier1 = 1,
    /// Indicates support for sparse textures tier 2.
    ///
    /// In addition to the guarantees tier 1 sparse textures provide,
    /// tier 2 sparse textures allow the following:
    /// - Obtain per-tile activity counters.
    Tier2 = 2,
}

unsafe impl Encode for MTLTextureSparseTier {
    const ENCODING: Encoding = i64::ENCODING;
}

unsafe impl RefEncode for MTLTextureSparseTier {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
