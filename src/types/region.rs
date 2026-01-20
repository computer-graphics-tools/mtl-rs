use crate::types::{MTLOrigin, MTLSize};
use objc2::{Encode, Encoding, RefEncode};

/// Identify a region in an image or texture.
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct MTLRegion {
    pub origin: MTLOrigin,
    pub size: MTLSize,
}

unsafe impl Encode for MTLRegion {
    const ENCODING: Encoding = Encoding::Struct(
        "{MTLRegion={MTLOrigin=QQQ}{MTLSize=QQQ}}",
        &[MTLOrigin::ENCODING, MTLSize::ENCODING],
    );
}

unsafe impl RefEncode for MTLRegion {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
