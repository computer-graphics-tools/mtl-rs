use objc2::extern_protocol;
use objc2_foundation::NSObjectProtocol;

extern_protocol!(
    /// Bridged protocol for `MTLFunction`.
    pub unsafe trait MTLFunction: NSObjectProtocol {}
);
