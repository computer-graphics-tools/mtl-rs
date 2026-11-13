use core::ptr::NonNull;

use block2::{Block, RcBlock};
use objc2::runtime::ProtocolObject;

use crate::MTLDrawable;

type DrawablePresentedBlock = dyn Fn(NonNull<ProtocolObject<dyn MTLDrawable>>);

/// The presented callback function protocol.
///
/// Be careful when you use delta between this `presented_time` and previous
/// frame's `presented_time` to animate next frame. If the frame was presented
/// using `present_after_minimum_duration` or `present_at_time`, the
/// `presented_time` might include delays to meet your specified present time.
/// If you want to measure how much frame you can achieve, use GPUStartTime in
/// the first command buffer of your frame rendering and GPUEndTime of your last
/// frame rendering to calculate the frame interval.
pub struct MTLDrawablePresentedHandler(RcBlock<DrawablePresentedBlock>);

impl MTLDrawablePresentedHandler {
    /// Creates a presented callback whose captured state can safely be sent
    /// to and shared with Metal's callback thread.
    ///
    /// [Apple's declaration](https://developer.apple.com/documentation/metal/mtldrawablepresentedhandler):
    ///
    /// > `typealias MTLDrawablePresentedHandler = @Sendable (any MTLDrawable) -> Void`
    pub fn new<F>(handler: F) -> Self
    where
        F: Fn(&ProtocolObject<dyn MTLDrawable>) + Send + Sync + 'static,
    {
        Self(RcBlock::new(move |drawable_nn: NonNull<ProtocolObject<dyn MTLDrawable>>| {
            let drawable = unsafe { drawable_nn.as_ref() };
            handler(drawable);
        }))
    }

    pub(super) fn as_block(&self) -> &Block<DrawablePresentedBlock> {
        &self.0
    }
}
