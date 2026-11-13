use objc2::{Message, extern_protocol, msg_send, rc::Retained, runtime::ProtocolObject};
use objc2_foundation::{NSObjectProtocol, NSString};

extern_protocol!(
    /// Bridged protocol for `MTLEvent`.
    ///
    /// Availability: macOS 10.14+, iOS 12.0+
    ///
    /// # Thread safety
    ///
    /// [Apple's declaration](https://developer.apple.com/documentation/metal/mtlevent):
    ///
    /// > `protocol MTLEvent : NSObjectProtocol, Sendable`
    ///
    /// The `Send` and `Sync` bounds rely on this guarantee, also inherited by `MTLSharedEvent`.
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

    /// Sets the optional debug label.
    fn set_label(
        &self,
        label: Option<&str>,
    );
}

impl<T: MTLEvent + Message + ?Sized> MTLEventExt for T {
    fn label(&self) -> Option<String> {
        let label: Option<Retained<NSString>> = unsafe { msg_send![self, label] };
        label.map(|s| s.to_string())
    }

    fn set_label(
        &self,
        label: Option<&str>,
    ) {
        unsafe {
            let _: () = msg_send![self, setLabel: label.map(NSString::from_str).as_deref()];
        }
    }
}
