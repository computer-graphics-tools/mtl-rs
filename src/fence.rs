use objc2::{Message, extern_protocol, msg_send, rc::Retained, runtime::ProtocolObject};
use objc2_foundation::{NSObjectProtocol, NSString};

use crate::MTLDevice;

extern_protocol!(
    /// Apple's documentation: `https://developer.apple.com/documentation/metal/mtlfence?language=objc`
    ///
    /// # Thread safety
    ///
    /// [Apple's declaration](https://developer.apple.com/documentation/metal/mtlfence):
    ///
    /// > `protocol MTLFence : NSObjectProtocol, Sendable`
    ///
    /// The `Send` and `Sync` bounds rely on this guarantee.
    pub unsafe trait MTLFence: NSObjectProtocol + Send + Sync {
        #[unsafe(method(device))]
        #[unsafe(method_family = none)]
        fn device(&self) -> Retained<ProtocolObject<dyn MTLDevice>>;
    }
);

/// Rust-native label access for a Metal fence.
pub trait MTLFenceExt: MTLFence + Message {
    /// A string to help identify this object.
    fn label(&self) -> Option<String>
    where
        Self: Sized,
    {
        let label: Option<Retained<NSString>> = unsafe { msg_send![self, label] };
        label.map(|label| label.to_string())
    }

    /// Sets the optional label, copying it into the fence.
    fn set_label(
        &self,
        label: Option<&str>,
    ) where
        Self: Sized,
    {
        let label = label.map(NSString::from_str);
        unsafe {
            let _: () = msg_send![self, setLabel: label.as_deref()];
        }
    }
}

impl<T: MTLFence + Message> MTLFenceExt for T {}
