use objc2::{Encode, Encoding, RefEncode};

/// Type of mapping operation for sparse texture (from `MTLSparseTextureMappingMode`).
#[repr(u64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum MTLSparseTextureMappingMode {
    Map = 0,
    Unmap = 1,
}

unsafe impl Encode for MTLSparseTextureMappingMode {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for MTLSparseTextureMappingMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
