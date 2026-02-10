use objc2::{Message, extern_protocol, msg_send, rc::Retained, runtime::ProtocolObject};
use objc2_foundation::{NSObjectProtocol, NSString};

use crate::{MTL4CommandQueue, MTLCommandQueue, MTLDevice};

extern_protocol!(
    /// Capture scope to bracket GPU work for debugging captures.
    ///
    /// Availability: macOS 10.13+, iOS 11.0+
    ///
    /// Remarks: Only `MTLCommandBuffer`s created after `beginScope` and committed before `endScope` are captured.
    pub unsafe trait MTLCaptureScope: NSObjectProtocol {
        /// Marks the begin of the capture scope. This should be invoked repeatedly per frame.
        #[unsafe(method(beginScope))]
        #[unsafe(method_family = none)]
        fn begin_scope(&self);

        /// Marks the end of the capture scope. This should be invoked repeatedly per frame.
        #[unsafe(method(endScope))]
        #[unsafe(method_family = none)]
        fn end_scope(&self);

        /// Associated device: this scope will capture Metal commands from the associated device.
        #[unsafe(method(device))]
        #[unsafe(method_family = none)]
        fn device(&self) -> Retained<ProtocolObject<dyn MTLDevice>>;

        /// If set, this scope will only capture Metal commands from the associated command queue.
        /// Defaults to `None` (all command queues from the associated device are captured).
        #[unsafe(method(commandQueue))]
        #[unsafe(method_family = none)]
        fn command_queue(&self) -> Option<Retained<ProtocolObject<dyn MTLCommandQueue>>>;

        /// If set, this scope will only capture Metal commands from the associated Metal 4 command queue.
        /// Defaults to `None` (all command queues from the associated device are captured).
        #[unsafe(method(mtl4CommandQueue))]
        #[unsafe(method_family = none)]
        fn mtl4_command_queue(&self) -> Option<Retained<ProtocolObject<dyn MTL4CommandQueue>>>;
    }
);

pub trait MTLCaptureScopeExt: MTLCaptureScope + Message {
    fn label(&self) -> Option<String>
    where
        Self: Sized,
    {
        let label: Option<Retained<NSString>> = unsafe { msg_send![self, label] };
        label.map(|label| label.to_string())
    }

    fn set_label(&self, label: Option<&str>)
    where
        Self: Sized,
    {
        unsafe {
            let _: () = msg_send![self, setLabel: label.map(NSString::from_str).as_deref()];
        }
    }
}

impl<T: MTLCaptureScope + Message> MTLCaptureScopeExt for T {}
