use objc2::{Encode, Encoding, RefEncode};

/// Overall kind of entry point (from `MTLFunctionType`).
#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MTLFunctionType {
    Vertex = 1,
    Fragment = 2,
    Kernel = 3,
    Visible = 5,
    Intersection = 6,
    Mesh = 7,
    Object = 8,
}

unsafe impl Encode for MTLFunctionType {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for MTLFunctionType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
