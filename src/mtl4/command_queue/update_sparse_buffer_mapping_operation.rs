use core::ops::Range;

use objc2::encode::{Encode, Encoding, RefEncode};
use objc2_foundation::NSRange;

use crate::*;

/// Groups together arguments for an operation to update a sparse buffer mapping.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MTL4UpdateSparseBufferMappingOperation {
    pub mode: MTLSparseTextureMappingMode,
    buffer_range: NSRange,
    pub heap_offset: usize,
}

impl MTL4UpdateSparseBufferMappingOperation {
    /// Constructs an update operation. `buffer_range` is in bytes.
    pub fn new(
        mode: MTLSparseTextureMappingMode,
        buffer_range: Range<usize>,
        heap_offset: usize,
    ) -> Self {
        Self {
            mode,
            buffer_range: NSRange::from(buffer_range),
            heap_offset,
        }
    }

    /// Byte range within the buffer affected by this operation.
    pub fn buffer_range(&self) -> Range<usize> {
        self.buffer_range.into()
    }
}

unsafe impl Encode for MTL4UpdateSparseBufferMappingOperation {
    const ENCODING: Encoding =
        Encoding::Struct("?", &[<MTLSparseTextureMappingMode>::ENCODING, <NSRange>::ENCODING, <usize>::ENCODING]);
}

unsafe impl RefEncode for MTL4UpdateSparseBufferMappingOperation {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
