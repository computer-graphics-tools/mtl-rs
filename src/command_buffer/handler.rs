use core::ptr::NonNull;

use block2::{Block, RcBlock};
use objc2::runtime::ProtocolObject;

use super::MTLCommandBuffer;

/// Rust-friendly wrapper for `MTLCommandBufferHandler`.
///
/// Apple's type: `typedef void (^MTLCommandBufferHandler)(id<MTLCommandBuffer>);`
pub struct MTLCommandBufferHandler(RcBlock<dyn Fn(NonNull<ProtocolObject<dyn MTLCommandBuffer>>)>);

impl MTLCommandBufferHandler {
    /// Creates a command-buffer callback.
    ///
    /// Metal may invoke the callback on an implementation-defined thread, so
    /// captured state must be safe to transfer and share between threads.
    ///
    /// [Apple's declaration](https://developer.apple.com/documentation/metal/mtlcommandbufferhandler):
    ///
    /// > `typealias MTLCommandBufferHandler = @Sendable (any MTLCommandBuffer) -> Void`
    pub fn new<F>(handler: F) -> Self
    where
        F: Fn(&ProtocolObject<dyn MTLCommandBuffer>) + Send + Sync + 'static,
    {
        Self(RcBlock::new(move |cb_ptr: NonNull<ProtocolObject<dyn MTLCommandBuffer>>| {
            let cb = unsafe { cb_ptr.as_ref() };
            handler(cb);
        }))
    }

    pub(super) fn as_block(&self) -> &Block<dyn Fn(NonNull<ProtocolObject<dyn MTLCommandBuffer>>)> {
        &self.0
    }
}
