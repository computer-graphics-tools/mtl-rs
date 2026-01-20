use objc2::{Encode, Encoding, RefEncode};

/// Options to create a stitched library (from `MTLStitchedLibraryOptions`).
///
/// Availability: macOS 15.0+, iOS 18.0+
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLStitchedLibraryOptions(pub usize);
bitflags::bitflags! {
    impl MTLStitchedLibraryOptions: usize {
        const None = 0;
        const FailOnBinaryArchiveMiss = 1<<0;
        const StoreLibraryInMetalPipelinesScript = 1<<1;
    }
}

unsafe impl Encode for MTLStitchedLibraryOptions {
    const ENCODING: Encoding = usize::ENCODING;
}

unsafe impl RefEncode for MTLStitchedLibraryOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
