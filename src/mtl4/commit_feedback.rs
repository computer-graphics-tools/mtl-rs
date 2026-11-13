use core::ptr::NonNull;

use block2::{Block, RcBlock};
use objc2::{Message, extern_protocol, msg_send, rc::Retained, runtime::ProtocolObject};
use objc2_foundation::{NSError, NSObjectProtocol};

use crate::*;

/// Defines the block signature for a callback Metal invokes to provide your app feedback after completing a workload.
///
/// You register a commit feedback block with Metal by providing an instance of ``MTL4CommitOptions`` to
/// the command queue's commit method, ``MTL4CommandQueue/commit:count:options:``. The commit options instance
/// references your commit feedback handler after you add it via its ``MTL4CommitOptions/addFeedbackHandler:``
/// method.
///
/// - Parameter commitFeedback: a commit feedback instance containing information about the workload.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/metal/mtl4commitfeedbackhandler?language=objc)
pub struct MTL4CommitFeedbackHandler(RcBlock<dyn Fn(NonNull<ProtocolObject<dyn MTL4CommitFeedback>>)>);

impl MTL4CommitFeedbackHandler {
    /// Creates a callback whose captures can safely be used from Metal's
    /// feedback queue.
    ///
    /// [Apple's declaration](https://developer.apple.com/documentation/metal/mtl4commitfeedbackhandler):
    ///
    /// > `typealias MTL4CommitFeedbackHandler = @Sendable (any MTL4CommitFeedback) -> Void`
    pub fn new<F>(handler: F) -> Self
    where
        F: Fn(&ProtocolObject<dyn MTL4CommitFeedback>) + Send + Sync + 'static,
    {
        Self(RcBlock::new(move |feedback: NonNull<ProtocolObject<dyn MTL4CommitFeedback>>| {
            handler(unsafe { feedback.as_ref() });
        }))
    }

    pub(super) fn as_block(&self) -> &Block<dyn Fn(NonNull<ProtocolObject<dyn MTL4CommitFeedback>>)> {
        &self.0
    }
}

extern_protocol!(
    /// Describes an object containing debug information from Metal to your app after completing a workload.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metal/mtl4commitfeedback?language=objc)
    pub unsafe trait MTL4CommitFeedback: NSObjectProtocol {
        /// The host time, in seconds, when the GPU starts execution of the committed command buffers.
        #[unsafe(method(GPUStartTime))]
        #[unsafe(method_family = none)]
        fn gpu_start_time(&self) -> f64;

        /// The host time, in seconds, when the GPU finishes execution of the committed command buffers.
        #[unsafe(method(GPUEndTime))]
        #[unsafe(method_family = none)]
        fn gpu_end_time(&self) -> f64;
    }
);

pub trait MTL4CommitFeedbackExt: MTL4CommitFeedback + Message {
    /// A Rust-owned description of an error encountered by the GPU.
    fn error(&self) -> Option<MetalError>
    where
        Self: Sized,
    {
        let error: Option<Retained<NSError>> = unsafe { msg_send![self, error] };
        error.map(MetalError::from_nserror)
    }
}

impl<T: MTL4CommitFeedback + Message> MTL4CommitFeedbackExt for T {}
