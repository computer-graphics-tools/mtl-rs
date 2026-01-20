use core::ptr::NonNull;

use block2::{Block, RcBlock};
use objc2::runtime::ProtocolObject;

use super::MTLCommandBuffer;

/// Rust-friendly wrapper for `MTLCommandBufferHandler`.
///
/// Apple's type: `typedef void (^MTLCommandBufferHandler)(id<MTLCommandBuffer>);`
pub struct MTLCommandBufferHandler(RcBlock<dyn Fn(NonNull<ProtocolObject<dyn MTLCommandBuffer>>)>);

impl MTLCommandBufferHandler {
    pub fn new<F>(handler: F) -> Self
    where
        F: Fn(&ProtocolObject<dyn MTLCommandBuffer>) + 'static,
    {
        Self(RcBlock::new(
            move |cb_ptr: NonNull<ProtocolObject<dyn MTLCommandBuffer>>| {
                let cb = unsafe { cb_ptr.as_ref() };
                handler(cb);
            },
        ))
    }
}

impl core::ops::Deref for MTLCommandBufferHandler {
    type Target = Block<dyn Fn(NonNull<ProtocolObject<dyn MTLCommandBuffer>>)>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
