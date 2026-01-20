use objc2::{extern_protocol, rc::Retained, runtime::ProtocolObject};
use objc2_foundation::{NSObjectProtocol, NSString};

use crate::{MTLCommandQueue, MTLDevice};

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

        /// Scope label.
        #[unsafe(method(label))]
        #[unsafe(method_family = none)]
        fn label(&self) -> Option<Retained<NSString>>;

        /// Setter for `label`.
        #[unsafe(method(setLabel:))]
        #[unsafe(method_family = none)]
        unsafe fn set_label(&self, label: Option<&NSString>);

        /// Associated device: this scope will capture Metal commands from the associated device.
        #[unsafe(method(device))]
        #[unsafe(method_family = none)]
        unsafe fn device(&self) -> Retained<ProtocolObject<dyn MTLDevice>>;

        /// If set, this scope will only capture Metal commands from the associated command queue.
        /// Defaults to `None` (all command queues from the associated device are captured).
        #[unsafe(method(commandQueue))]
        #[unsafe(method_family = none)]
        unsafe fn command_queue(&self) -> Option<Retained<ProtocolObject<dyn MTLCommandQueue>>>;

        /// If set, this scope will only capture Metal commands from the associated Metal 4 command queue.
        /// Defaults to `None` (all command queues from the associated device are captured).
        #[unsafe(method(mtl4CommandQueue))]
        #[unsafe(method_family = none)]
        unsafe fn mtl4_command_queue(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn crate::MTL4CommandQueue>>>;
    }
);
