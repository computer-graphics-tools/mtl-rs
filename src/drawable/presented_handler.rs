use core::{ops::Deref, ptr::NonNull};

use block2::{Block, RcBlock};
use objc2::runtime::ProtocolObject;

use crate::MTLDrawable;

/// The presented callback function protocol.
///
/// Be careful when you use delta between this `presented_time` and previous
/// frame's `presented_time` to animate next frame. If the frame was presented
/// using `present_after_minimum_duration` or `present_at_time`, the
/// `presented_time` might include delays to meet your specified present time.
/// If you want to measure how much frame you can achieve, use GPUStartTime in
/// the first command buffer of your frame rendering and GPUEndTime of your last
/// frame rendering to calculate the frame interval.
pub struct MTLDrawablePresentedHandler(RcBlock<dyn Fn(NonNull<ProtocolObject<dyn MTLDrawable>>)>);

impl MTLDrawablePresentedHandler {
    pub fn new<F>(handler: F) -> Self
    where
        F: Fn(&ProtocolObject<dyn MTLDrawable>) + 'static,
    {
        Self(RcBlock::new(
            move |drawable_nn: NonNull<ProtocolObject<dyn MTLDrawable>>| {
                let drawable = unsafe { drawable_nn.as_ref() };
                handler(drawable);
            },
        ))
    }
}

impl Deref for MTLDrawablePresentedHandler {
    type Target = Block<dyn Fn(NonNull<ProtocolObject<dyn MTLDrawable>>)>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
