use objc2::extern_protocol;
use objc2_foundation::NSObjectProtocol;

extern_protocol!(
    /// Extendible protocol used to wrap scratch buffers for IO commands.
    ///
    /// Availability: macOS 13.0+, iOS 16.0+
    pub unsafe trait MTLIOScratchBuffer: NSObjectProtocol {}
);
