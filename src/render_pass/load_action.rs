use objc2::{Encode, Encoding, RefEncode};

/// Attachment load action (from `MTLLoadAction`).
#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MTLLoadAction {
    DontCare = 0,
    Load = 1,
    Clear = 2,
}

unsafe impl Encode for MTLLoadAction {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for MTLLoadAction {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
