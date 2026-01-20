use core::{ops::Deref, ptr::NonNull};

use block2::{Block, RcBlock};
use objc2::runtime::ProtocolObject;

use super::MTLIOCommandBuffer;

pub struct MTLIOCommandBufferCompletedHandler(
    RcBlock<dyn Fn(NonNull<ProtocolObject<dyn MTLIOCommandBuffer>>)>,
);

impl MTLIOCommandBufferCompletedHandler {
    pub fn new<F>(handler: F) -> Self
    where
        F: Fn(&ProtocolObject<dyn MTLIOCommandBuffer>) + 'static,
    {
        Self(RcBlock::new(
            move |ptr: NonNull<ProtocolObject<dyn MTLIOCommandBuffer>>| {
                let cb = unsafe { ptr.as_ref() };
                handler(cb);
            },
        ))
    }
}

impl Deref for MTLIOCommandBufferCompletedHandler {
    type Target = Block<dyn Fn(NonNull<ProtocolObject<dyn MTLIOCommandBuffer>>)>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
