use bitflags::bitflags;
use objc2::{Encode, Encoding, RefEncode};

bitflags! {
    /// Pipeline creation options (ported from `MTLPipelineOption`).
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
    pub struct MTLPipelineOption: usize {
        const BINDING_INFO = 1 << 0;
        const BUFFER_TYPE_INFO = 1 << 1;
        const FAIL_ON_BINARY_ARCHIVE_MISS = 1 << 2;
    }
}

unsafe impl Encode for MTLPipelineOption {
    const ENCODING: Encoding = usize::ENCODING;
}

unsafe impl RefEncode for MTLPipelineOption {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
