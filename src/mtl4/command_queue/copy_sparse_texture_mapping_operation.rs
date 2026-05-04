use objc2::encode::{Encode, Encoding, RefEncode};

use crate::*;

/// Groups together arguments for an operation to copy a sparse texture mapping.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MTL4CopySparseTextureMappingOperation {
    pub source_region: MTLRegion,
    pub source_level: usize,
    pub source_slice: usize,
    pub destination_origin: MTLOrigin,
    pub destination_level: usize,
    pub destination_slice: usize,
}

unsafe impl Encode for MTL4CopySparseTextureMappingOperation {
    const ENCODING: Encoding = Encoding::Struct(
        "?",
        &[
            <MTLRegion>::ENCODING,
            <usize>::ENCODING,
            <usize>::ENCODING,
            <MTLOrigin>::ENCODING,
            <usize>::ENCODING,
            <usize>::ENCODING,
        ],
    );
}

unsafe impl RefEncode for MTL4CopySparseTextureMappingOperation {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
