use bitflags::bitflags;
use objc2::{Encode, Encoding, RefEncode};

bitflags! {
    /// MTLTextureUsage declares how the texture will be used over its lifetime (bitwise OR for multiple uses).
    ///
    /// Discussion: This information may be used by the driver to make optimization decisions.
    ///
    /// Availability: API_AVAILABLE(macos(10.11), ios(9.0))
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
    pub struct MTLTextureUsage: usize {
        /// Unknown usage
        const UNKNOWN = 0x0000;
        /// Readable from shader stages
        const SHADER_READ = 0x0001;
        /// Writable from shader stages
        const SHADER_WRITE = 0x0002;
        /// Usable as a render target
        const RENDER_TARGET = 0x0004;
        /// Supports creating views with different but compatible pixel formats
        const PIXEL_FORMAT_VIEW = 0x0010;
        /// Supports shader atomic operations
        ///
        /// Availability: API_AVAILABLE(macos(14.0), ios(17.0))
        const SHADER_ATOMIC = 0x0020;
    }
}

unsafe impl Encode for MTLTextureUsage {
    const ENCODING: Encoding = usize::ENCODING;
}

unsafe impl RefEncode for MTLTextureUsage {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
