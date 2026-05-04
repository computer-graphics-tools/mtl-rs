use core::ops::Range;

use objc2::encode::{Encode, Encoding, RefEncode};
use objc2_foundation::NSRange;

/// Groups together arguments for an operation to copy a sparse buffer mapping.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MTL4CopySparseBufferMappingOperation {
    source_range: NSRange,
    pub destination_offset: usize,
}

impl MTL4CopySparseBufferMappingOperation {
    /// Constructs a copy operation. `source_range` is in bytes.
    pub fn new(
        source_range: Range<usize>,
        destination_offset: usize,
    ) -> Self {
        Self {
            source_range: NSRange::from(source_range),
            destination_offset,
        }
    }

    /// Byte range within the source buffer this operation copies from.
    pub fn source_range(&self) -> Range<usize> {
        self.source_range.into()
    }
}

unsafe impl Encode for MTL4CopySparseBufferMappingOperation {
    const ENCODING: Encoding = Encoding::Struct("?", &[<NSRange>::ENCODING, <usize>::ENCODING]);
}

unsafe impl RefEncode for MTL4CopySparseBufferMappingOperation {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
