use objc2::extern_protocol;
use objc2_foundation::NSObjectProtocol;

extern_protocol!(
    /// Bridged protocol for `MTLRenderPipelineState`.
    pub unsafe trait MTLRenderPipelineState: NSObjectProtocol {}
);
