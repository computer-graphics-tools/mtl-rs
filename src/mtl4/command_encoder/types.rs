use objc2::encode::{Encode, Encoding, RefEncode};
use objc2_foundation::NSUInteger;

/// Memory consistency options for synchronization commands.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/metal/mtl4visibilityoptions?language=objc)
/// NS_OPTIONS
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTL4VisibilityOptions(pub NSUInteger);
bitflags::bitflags! {
    impl MTL4VisibilityOptions: NSUInteger {
        /// Don't flush caches. When you use this option on a barrier, it turns it into an execution barrier.
        #[doc(alias = "MTL4VisibilityOptionNone")]
        const None = 0;
        /// Flushes caches to the GPU (device) memory coherence point.
        #[doc(alias = "MTL4VisibilityOptionDevice")]
        const Device = 1<<0;
        /// Flushes caches to ensure that aliased virtual addresses are memory consistent.
        ///
        /// On some systems this may be the GPU+CPU (system) memory coherence point
        /// and on other systems it may be the GPU (device) memory coherence point.
        #[doc(alias = "MTL4VisibilityOptionResourceAlias")]
        const ResourceAlias = 1<<1;
    }
}

unsafe impl Encode for MTL4VisibilityOptions {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for MTL4VisibilityOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
