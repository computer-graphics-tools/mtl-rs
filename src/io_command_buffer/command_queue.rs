use objc2::{Message, extern_protocol, msg_send, rc::Retained, runtime::ProtocolObject};
use objc2_foundation::{NSObjectProtocol, NSString};

use super::MTLIOCommandBuffer;

extern_protocol!(
    /// Represents a queue that schedules IO command buffers.
    ///
    /// Availability: macOS 13.0+, iOS 16.0+
    pub unsafe trait MTLIOCommandQueue: NSObjectProtocol + Send + Sync {
        /// Insert a barrier to order prior and subsequent command buffers.
        #[unsafe(method(enqueueBarrier))]
        #[unsafe(method_family = none)]
        unsafe fn enqueue_barrier(&self);

        /// Vend an autoreleased command buffer for encoding IO commands.
        #[unsafe(method(commandBuffer))]
        #[unsafe(method_family = none)]
        unsafe fn command_buffer(&self) -> Retained<ProtocolObject<dyn MTLIOCommandBuffer>>;

        /// Vend an autoreleased command buffer that does not retain referenced objects.
        ///
        /// For correct execution, the application must retain objects referenced by commands.
        #[unsafe(method(commandBufferWithUnretainedReferences))]
        #[unsafe(method_family = none)]
        unsafe fn command_buffer_with_unretained_references(
            &self,
        ) -> Retained<ProtocolObject<dyn MTLIOCommandBuffer>>;
    }
);

#[allow(unused)]
pub trait MTLIOCommandQueueExt: MTLIOCommandQueue + Message {
    fn label(&self) -> Option<String>;
    fn set_label(&self, label: Option<&str>);
}

impl MTLIOCommandQueueExt for ProtocolObject<dyn MTLIOCommandQueue> {
    fn label(&self) -> Option<String> {
        let s: Option<Retained<NSString>> = unsafe { msg_send![self, label] };
        s.map(|s| s.to_string())
    }

    fn set_label(&self, label: Option<&str>) {
        unsafe {
            let _: () = msg_send![self, setLabel: label.map(NSString::from_str).as_deref()];
        }
    }
}
