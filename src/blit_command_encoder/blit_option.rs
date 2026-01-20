use objc2::{Encode, Encoding, RefEncode};

/// Controls the blit operation
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLBlitOption(pub u64);

bitflags::bitflags! {
    impl MTLBlitOption: u64 {
        const None = 0;
        const DepthFromDepthStencil = 1<<0;
        const StencilFromDepthStencil = 1<<1;
        const RowLinearPVRTC = 1<<2;
    }
}

unsafe impl Encode for MTLBlitOption {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for MTLBlitOption {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
