use objc2::{Encode, Encoding, RefEncode};

#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MTLLanguageVersion {
    #[deprecated(note = "use a newer language standard")]
    Version1_0 = 1 << 16,
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
    Version4_1 = (4 << 16) + 1,
}

/// The Metal shading-language version used to compile a library.
///
/// Compatibility alias for the crate's original misspelling of
/// [`MTLLanguageVersion`].
pub type MLTLanguageVersion = MTLLanguageVersion;

unsafe impl Encode for MTLLanguageVersion {
    const ENCODING: Encoding = u64::ENCODING;
}
unsafe impl RefEncode for MTLLanguageVersion {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
