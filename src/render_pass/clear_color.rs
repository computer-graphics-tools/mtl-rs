use objc2::{Encode, Encoding, RefEncode};

/// Clear color used for render pass color attachments.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MTLClearColor {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
    pub alpha: f64,
}

unsafe impl Encode for MTLClearColor {
    const ENCODING: Encoding = Encoding::Struct(
        "{MTLClearColor=dddd}",
        &[f64::ENCODING, f64::ENCODING, f64::ENCODING, f64::ENCODING],
    );
}

unsafe impl RefEncode for MTLClearColor {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
