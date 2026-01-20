use objc2::{Encode, Encoding, RefEncode};

/// Location of the GPU on macOS (ported from `MTLDeviceLocation`).
#[repr(u64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum MTLDeviceLocation {
    BuiltIn = 0,
    Slot = 1,
    External = 2,
    Unspecified = u64::MAX,
}

unsafe impl Encode for MTLDeviceLocation {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for MTLDeviceLocation {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
