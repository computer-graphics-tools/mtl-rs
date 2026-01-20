use objc2::{Encode, Encoding, RefEncode};

/// Options for store action (from `MTLStoreActionOptions`).
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLStoreActionOptions(pub u64);

bitflags::bitflags! {
    impl MTLStoreActionOptions: u64 {
        const None = 0;
        const CustomSamplePositions = 1<<0;
    }
}

unsafe impl Encode for MTLStoreActionOptions {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for MTLStoreActionOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
