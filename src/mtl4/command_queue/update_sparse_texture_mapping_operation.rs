use objc2::encode::{Encode, Encoding, RefEncode};

use crate::*;

/// Groups together arguments for an operation to update a sparse texture mapping.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MTL4UpdateSparseTextureMappingOperation {
    pub mode: MTLSparseTextureMappingMode,
    pub texture_region: MTLRegion,
    pub texture_level: usize,
    pub texture_slice: usize,
    pub heap_offset: usize,
}

unsafe impl Encode for MTL4UpdateSparseTextureMappingOperation {
    const ENCODING: Encoding = Encoding::Struct(
        "?",
        &[
            <MTLSparseTextureMappingMode>::ENCODING,
            <MTLRegion>::ENCODING,
            <usize>::ENCODING,
            <usize>::ENCODING,
            <usize>::ENCODING,
        ],
    );
}

unsafe impl RefEncode for MTL4UpdateSparseTextureMappingOperation {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
