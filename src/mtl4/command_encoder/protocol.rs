use objc2::{Message, extern_protocol, msg_send, rc::Retained, runtime::ProtocolObject};
use objc2_foundation::{NSObjectProtocol, NSString};

use super::types::MTL4VisibilityOptions;
use crate::*;

extern_protocol!(
    /// An encoder that writes GPU commands into a command buffer.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metal/mtl4commandencoder?language=objc)
    pub unsafe trait MTL4CommandEncoder: NSObjectProtocol {
        /// Returns the command buffer that is currently encoding commands.
        ///
        /// This property may return undefined results if you call it after calling ``endEncoding``.
        #[unsafe(method(commandBuffer))]
        #[unsafe(method_family = none)]
        unsafe fn command_buffer(&self) -> Option<Retained<ProtocolObject<dyn MTL4CommandBuffer>>>;

        /// Encodes a consumer barrier on work you commit to the same command queue.
        #[unsafe(method(barrierAfterQueueStages:beforeStages:visibilityOptions:))]
        #[unsafe(method_family = none)]
        unsafe fn barrier_after_queue_stages_before_stages_visibility_options(
            &self,
            after_queue_stages: MTLRenderStages,
            before_stages: MTLRenderStages,
            visibility_options: MTL4VisibilityOptions,
        );

        /// Encodes a producer barrier on work committed to the same command queue.
        #[unsafe(method(barrierAfterStages:beforeQueueStages:visibilityOptions:))]
        #[unsafe(method_family = none)]
        unsafe fn barrier_after_stages_before_queue_stages_visibility_options(
            &self,
            after_stages: MTLRenderStages,
            before_queue_stages: MTLRenderStages,
            visibility_options: MTL4VisibilityOptions,
        );

        /// Encodes an intra-pass barrier.
        #[unsafe(method(barrierAfterEncoderStages:beforeEncoderStages:visibilityOptions:))]
        #[unsafe(method_family = none)]
        unsafe fn barrier_after_encoder_stages_before_encoder_stages_visibility_options(
            &self,
            after_encoder_stages: MTLRenderStages,
            before_encoder_stages: MTLRenderStages,
            visibility_options: MTL4VisibilityOptions,
        );

        /// Encodes a command to update a GPU fence.
        #[unsafe(method(updateFence:afterEncoderStages:))]
        #[unsafe(method_family = none)]
        unsafe fn update_fence_after_encoder_stages(
            &self,
            fence: &ProtocolObject<dyn MTLFence>,
            after_encoder_stages: MTLRenderStages,
        );

        /// Encodes a command to wait on a GPU fence.
        #[unsafe(method(waitForFence:beforeEncoderStages:))]
        #[unsafe(method_family = none)]
        unsafe fn wait_for_fence_before_encoder_stages(
            &self,
            fence: &ProtocolObject<dyn MTLFence>,
            before_encoder_stages: MTLRenderStages,
        );

        /// Pops the latest debug group string from this encoder's stack of debug groups.
        #[unsafe(method(popDebugGroup))]
        #[unsafe(method_family = none)]
        unsafe fn pop_debug_group(&self);

        /// Declares that all command generation from this encoder is complete.
        #[unsafe(method(endEncoding))]
        #[unsafe(method_family = none)]
        unsafe fn end_encoding(&self);
    }
);

#[allow(unused)]
pub trait MTL4CommandEncoderExt: MTL4CommandEncoder + Message {
    /// Provides an optional label to assign to the command encoder for debug purposes.
    fn label(&self) -> Option<String>;
    /// Setter for label.
    fn set_label(&self, label: Option<&str>);
    /// Inserts a debug signpost.
    fn insert_debug_signpost(&self, signpost: &str);
    /// Pushes a debug group.
    fn push_debug_group(&self, label: &str);
}

impl MTL4CommandEncoderExt for ProtocolObject<dyn MTL4CommandEncoder> {
    fn label(&self) -> Option<String> {
        let s: Option<Retained<NSString>> = unsafe { msg_send![self, label] };
        s.map(|v| v.to_string())
    }

    fn set_label(&self, label: Option<&str>) {
        unsafe {
            let _: () = msg_send![self, setLabel: label.map(NSString::from_str).as_deref()];
        }
    }

    fn insert_debug_signpost(&self, signpost: &str) {
        unsafe {
            let _: () = msg_send![self, insertDebugSignpost: &*NSString::from_str(signpost)];
        }
    }

    fn push_debug_group(&self, label: &str) {
        unsafe {
            let _: () = msg_send![self, pushDebugGroup: &*NSString::from_str(label)];
        }
    }
}
