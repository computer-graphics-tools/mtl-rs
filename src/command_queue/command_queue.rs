use objc2::{Message, extern_protocol, msg_send, rc::Retained, runtime::ProtocolObject};
use objc2_foundation::{NSObjectProtocol, NSString};

use crate::{MTLCommandBuffer, MTLCommandBufferDescriptor, MTLDevice};

extern_protocol!(
    /// A serial queue of command buffers to be executed by the device.
    pub unsafe trait MTLCommandQueue: NSObjectProtocol + Send + Sync {
        /// The device this queue will submit to.
        #[unsafe(method(device))]
        #[unsafe(method_family = none)]
        fn device(&self) -> Retained<ProtocolObject<dyn MTLDevice>>;

        /// Returns a new command buffer used to encode work into this queue that
        /// maintains strong references to resources used within the command buffer.
        #[unsafe(method(commandBuffer))]
        #[unsafe(method_family = none)]
        fn command_buffer(&self) -> Option<Retained<ProtocolObject<dyn MTLCommandBuffer>>>;

        /// Returns a new command buffer used to encode work into this queue.
        #[unsafe(method(commandBufferWithDescriptor:))]
        #[unsafe(method_family = none)]
        unsafe fn command_buffer_with_descriptor(
            &self,
            descriptor: &MTLCommandBufferDescriptor,
        ) -> Option<Retained<ProtocolObject<dyn MTLCommandBuffer>>>;

        /// Returns a new command buffer that does not maintain strong references to resources used within it.
        #[unsafe(method(commandBufferWithUnretainedReferences))]
        #[unsafe(method_family = none)]
        unsafe fn command_buffer_with_unretained_references(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn MTLCommandBuffer>>>;

        /// Inform Xcode about when debug capture should start and stop.
        #[deprecated]
        #[unsafe(method(insertDebugCaptureBoundary))]
        #[unsafe(method_family = none)]
        unsafe fn insert_debug_capture_boundary(&self);
    }
);

#[allow(unused)]
pub trait MTLCommandQueueExt: MTLCommandQueue + Message {
    fn label(&self) -> Option<String>;
    fn set_label(&self, label: Option<&str>);
}

impl MTLCommandQueueExt for ProtocolObject<dyn MTLCommandQueue> {
    fn label(&self) -> Option<String> {
        let label: Option<Retained<NSString>> = unsafe { msg_send![self, label] };
        label.map(|s| s.to_string())
    }

    fn set_label(&self, label: Option<&str>) {
        unsafe {
            let _: () = msg_send![self, setLabel: label.map(NSString::from_str).as_deref()];
        }
    }
}
