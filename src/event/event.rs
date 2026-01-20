use objc2::{Message, extern_protocol, msg_send, rc::Retained, runtime::ProtocolObject};
use objc2_foundation::{NSObjectProtocol, NSString};

extern_protocol!(
    /// Bridged protocol for `MTLEvent`.
    ///
    /// Availability: macOS 10.14+, iOS 12.0+
    pub unsafe trait MTLEvent: NSObjectProtocol + Send + Sync {
        /// The device this event can be used with. Will be nil when the event is shared across devices.
        #[unsafe(method(device))]
        #[unsafe(method_family = none)]
        fn device(&self) -> Option<Retained<ProtocolObject<dyn crate::MTLDevice>>>;
    }
);

#[allow(unused)]
pub trait MTLEventExt: MTLEvent + Message {
    /// Optional label.
    fn label(&self) -> Option<String>;
}

impl MTLEventExt for ProtocolObject<dyn MTLEvent> {
    fn label(&self) -> Option<String> {
        let label: Option<Retained<NSString>> = unsafe { msg_send![self, label] };
        label.map(|s| s.to_string())
    }
}
