use objc2::{Encode, Encoding, RefEncode};

/// Alignment mode when converting pixel regions to sparse tile regions (from `MTLSparseTextureRegionAlignmentMode`).
#[repr(u64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum MTLSparseTextureRegionAlignmentMode {
    Outward = 0,
    Inward = 1,
}

unsafe impl Encode for MTLSparseTextureRegionAlignmentMode {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for MTLSparseTextureRegionAlignmentMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
