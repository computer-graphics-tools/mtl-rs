use objc2::extern_protocol;
use objc2_foundation::NSObjectProtocol;

extern_protocol!(
    /// Container of logs emitted by Metal (bridged from `MTLLogContainer`).
    ///
    /// Availability: macOS 11.0+, iOS 14.0+
    pub unsafe trait MTLLogContainer: NSObjectProtocol {}
);
