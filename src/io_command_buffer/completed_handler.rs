use core::ptr::NonNull;

use block2::{Block, RcBlock};
use objc2::runtime::ProtocolObject;

use super::MTLIOCommandBuffer;

/// Rust-friendly wrapper for Metal's `MTLIOCommandBufferHandler` block.
///
/// Metal may invoke this callback on an implementation-selected thread, so the
/// constructor requires captured state to be `Send + Sync`. The command-buffer
/// reference passed to the callback is borrowed and is valid only for the
/// duration of that invocation.
pub struct MTLIOCommandBufferHandler(RcBlock<dyn Fn(NonNull<ProtocolObject<dyn MTLIOCommandBuffer>>)>);

impl MTLIOCommandBufferHandler {
    /// Creates an I/O-command-buffer completion callback with thread-safe
    /// captured state.
    ///
    /// [Apple's declaration](https://developer.apple.com/documentation/metal/mtliocommandbufferhandler):
    ///
    /// > `typealias MTLIOCommandBufferHandler = @Sendable (any MTLIOCommandBuffer) -> Void`
    pub fn new<F>(handler: F) -> Self
    where
        F: Fn(&ProtocolObject<dyn MTLIOCommandBuffer>) + Send + Sync + 'static,
    {
        Self(RcBlock::new(move |ptr: NonNull<ProtocolObject<dyn MTLIOCommandBuffer>>| {
            let cb = unsafe { ptr.as_ref() };
            handler(cb);
        }))
    }

    pub(super) fn as_block(&self) -> &Block<dyn Fn(NonNull<ProtocolObject<dyn MTLIOCommandBuffer>>)> {
        &self.0
    }
}

/// Backwards-compatible name for [`MTLIOCommandBufferHandler`].
pub type MTLIOCommandBufferCompletedHandler = MTLIOCommandBufferHandler;
