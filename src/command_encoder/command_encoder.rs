use objc2::{Message, extern_protocol, msg_send, rc::Retained, runtime::ProtocolObject};
use objc2_foundation::{NSObjectProtocol, NSString};

use crate::{MTLComputeCommandEncoder, MTLDevice, MTLRenderCommandEncoder, MTLRenderStages};

extern_protocol!(
    /// MTLCommandEncoder is the common interface for objects that write commands into MTLCommandBuffers.
    ///
    /// Availability: macOS 10.11+, iOS 8.0+
    pub unsafe trait MTLCommandEncoder: NSObjectProtocol {
        /// The device this resource was created against.
        #[unsafe(method(device))]
        #[unsafe(method_family = none)]
        fn device(&self) -> Retained<ProtocolObject<dyn MTLDevice>>;

        /// Declare that all command generation from this encoder is complete, and detach from the MTLCommandBuffer.
        #[unsafe(method(endEncoding))]
        #[unsafe(method_family = none)]
        fn end_encoding(&self);

        /// Encodes a consumer barrier on work you commit to the same command queue.
        ///
        /// Encode a barrier that guarantees that any subsequent work you encode in the current command encoder that corresponds
        /// to the `beforeStages` stages doesn't proceed until Metal completes all work prior to the current command encoder
        /// corresponding to the `afterQueueStages` stages, completes.
        ///
        /// Metal can reorder the exact point where it applies the barrier, so use this method for synchronizing between different passes.
        /// If you need to synchronize work within a pass that you encode with an instance of a subclass of `MTLCommandEncoder`,
        /// use memory barriers instead. For subclasses of `MTL4CommandEncoder`, use encoder barriers.
        ///
        /// You can specify `afterQueueStages` and `beforeStages` that contain `MTLStages` unrelated to the current command encoder.
        ///
        /// Availability: macOS 26.0+, iOS 26.0+
        #[unsafe(method(barrierAfterQueueStages:beforeStages:))]
        #[unsafe(method_family = none)]
        fn barrier_after_queue_stages_before_stages(
            &self,
            after_queue_stages: MTLRenderStages,
            before_stages: MTLRenderStages,
        );
    }
);

#[allow(unused)]
pub trait MTLCommandEncoderExt: MTLCommandEncoder + Message {
    /// The device this resource was created against.
    fn device(&self) -> Retained<ProtocolObject<dyn MTLDevice>>;
    /// A string to help identify this object.
    fn label(&self) -> Option<String>;
    /// Sets a string to help identify this object.
    fn set_label(&self, label: Option<&str>);
    /// Inserts a debug string into the command buffer. This does not change any API behavior, but can be useful when debugging.
    fn insert_debug_signpost(&self, string: &str);
    /// Push a new named string onto a stack of string labels.
    fn push_debug_group(&self, string: &str);
    /// Pop the latest named string off of the stack.
    fn pop_debug_group(&self);
}

impl MTLCommandEncoderExt for ProtocolObject<dyn MTLCommandEncoder> {
    fn device(&self) -> Retained<ProtocolObject<dyn MTLDevice>> {
        unsafe { msg_send![self, device] }
    }

    fn label(&self) -> Option<String> {
        let label: Option<Retained<NSString>> = unsafe { msg_send![self, label] };
        label.map(|s| s.to_string())
    }

    fn set_label(&self, label: Option<&str>) {
        unsafe {
            let _: () = msg_send![self, setLabel: label.map(NSString::from_str).as_deref()];
        }
    }

    fn insert_debug_signpost(&self, string: &str) {
        unsafe {
            let _: () = msg_send![self, insertDebugSignpost: &*NSString::from_str(string)];
        }
    }

    fn push_debug_group(&self, string: &str) {
        unsafe {
            let _: () = msg_send![self, pushDebugGroup: &*NSString::from_str(string)];
        }
    }

    fn pop_debug_group(&self) {
        unsafe {
            let _: () = msg_send![self, popDebugGroup];
        }
    }
}

impl MTLCommandEncoderExt for ProtocolObject<dyn MTLComputeCommandEncoder> {
    fn device(&self) -> Retained<ProtocolObject<dyn MTLDevice>> {
        unsafe { msg_send![self, device] }
    }

    fn label(&self) -> Option<String> {
        let label: Option<Retained<NSString>> = unsafe { msg_send![self, label] };
        label.map(|s| s.to_string())
    }

    fn set_label(&self, label: Option<&str>) {
        unsafe {
            let _: () = msg_send![self, setLabel: label.map(NSString::from_str).as_deref()];
        }
    }

    fn insert_debug_signpost(&self, string: &str) {
        unsafe {
            let _: () = msg_send![self, insertDebugSignpost: &*NSString::from_str(string)];
        }
    }

    fn push_debug_group(&self, string: &str) {
        unsafe {
            let _: () = msg_send![self, pushDebugGroup: &*NSString::from_str(string)];
        }
    }

    fn pop_debug_group(&self) {
        unsafe {
            let _: () = msg_send![self, popDebugGroup];
        }
    }
}

impl MTLCommandEncoderExt for ProtocolObject<dyn MTLRenderCommandEncoder> {
    fn device(&self) -> Retained<ProtocolObject<dyn MTLDevice>> {
        unsafe { msg_send![self, device] }
    }

    fn label(&self) -> Option<String> {
        let label: Option<Retained<NSString>> = unsafe { msg_send![self, label] };
        label.map(|s| s.to_string())
    }

    fn set_label(&self, label: Option<&str>) {
        unsafe {
            let _: () = msg_send![self, setLabel: label.map(NSString::from_str).as_deref()];
        }
    }

    fn insert_debug_signpost(&self, string: &str) {
        unsafe {
            let _: () = msg_send![self, insertDebugSignpost: &*NSString::from_str(string)];
        }
    }

    fn push_debug_group(&self, string: &str) {
        unsafe {
            let _: () = msg_send![self, pushDebugGroup: &*NSString::from_str(string)];
        }
    }

    fn pop_debug_group(&self) {
        unsafe {
            let _: () = msg_send![self, popDebugGroup];
        }
    }
}
