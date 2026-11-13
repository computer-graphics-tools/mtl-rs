use objc2::{Encode, Encoding, RefEncode};

/// The kind of tessellation patch produced by a post-tessellation vertex function.
///
/// Availability: macOS 10.12+, iOS 10.0+
#[repr(usize)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MTLPatchType {
    None = 0,
    Triangle = 1,
    Quad = 2,
}

unsafe impl Encode for MTLPatchType {
    const ENCODING: Encoding = usize::ENCODING;
}

unsafe impl RefEncode for MTLPatchType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
