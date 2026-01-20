use objc2::{Message, extern_protocol, msg_send, rc::Retained, runtime::ProtocolObject};
use objc2_foundation::{NSObjectProtocol, NSString};

extern_protocol!(
    /// Represents a file handle usable as a source for IO commands.
    ///
    /// Availability: macOS 13.0+, iOS 16.0+
    pub unsafe trait MTLIOFileHandle: NSObjectProtocol + Send + Sync {}
);

#[allow(unused)]
pub trait MTLIOFileHandleExt: MTLIOFileHandle + Message {
    fn label(&self) -> Option<String>;
    fn set_label(&self, label: Option<&str>);
}

impl MTLIOFileHandleExt for ProtocolObject<dyn MTLIOFileHandle> {
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
