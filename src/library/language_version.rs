use objc2::{Encode, Encoding, RefEncode};

#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MLTLanguageVersion {
    Version1_1 = (1 << 16) + 1,
    Version1_2 = (1 << 16) + 2,
    Version2_0 = 2 << 16,
    Version2_1 = (2 << 16) + 1,
    Version2_2 = (2 << 16) + 2,
    Version2_3 = (2 << 16) + 3,
    Version2_4 = (2 << 16) + 4,
    Version3_0 = (3 << 16),
    Version3_1 = (3 << 16) + 1,
    Version3_2 = (3 << 16) + 2,
    Version4_0 = (4 << 16),
}

unsafe impl Encode for MLTLanguageVersion {
    const ENCODING: Encoding = u64::ENCODING;
}
unsafe impl RefEncode for MLTLanguageVersion {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
