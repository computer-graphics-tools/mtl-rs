use objc2::{Encode, Encoding, RefEncode};

/// Motion border mode (from `MTLMotionBorderMode`).
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MTLMotionBorderMode {
    /// Motion is stopped. (default)
    Clamp = 0,
    /// Object disappears.
    Vanish = 1,
}

unsafe impl Encode for MTLMotionBorderMode {
    const ENCODING: Encoding = u32::ENCODING;
}

unsafe impl RefEncode for MTLMotionBorderMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
