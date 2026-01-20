use objc2::{Message, extern_protocol, msg_send, rc::Retained, runtime::ProtocolObject};
use objc2_foundation::{NSError, NSObjectProtocol, NSString, NSURL};

use crate::device::MTLDevice;

extern_protocol!(
    /// A container for the binary representation of code compiled for a Device.
    ///
    /// Availability: macOS 11.0+, iOS 14.0+
    ///
    /// Mirrors `MTLDynamicLibrary`.
    pub unsafe trait MTLDynamicLibrary: NSObjectProtocol + Send + Sync {
        /// The device this resource was created against. This resource can only be used with this device.
        #[unsafe(method(device))]
        #[unsafe(method_family = none)]
        fn device(&self) -> Retained<ProtocolObject<dyn MTLDevice>>;

        /// Writes the contents of the dynamic library to a file.
        ///
        /// On success, the file contains a representation of the source MTLLibrary used to create the MTLDynamicLibrary,
        /// as well as compiled code for the current device. Such files may be combined offline to include code for multiple devices.
        #[unsafe(method(serializeToURL:error:_))]
        #[unsafe(method_family = none)]
        fn serialize_to_url(&self, url: &NSURL) -> Result<(), Retained<NSError>>;
    }
);

#[allow(unused)]
pub trait MTLDynamicLibraryExt: MTLDynamicLibrary + Message {
    /// A string to help identify this object.
    fn label(&self) -> Option<String>;
    /// Setter for `label`.
    fn set_label(&self, label: Option<&str>);
    /// The install name of this dynamic library.
    fn install_name(&self) -> String;
}

impl MTLDynamicLibraryExt for ProtocolObject<dyn MTLDynamicLibrary> {
    fn label(&self) -> Option<String> {
        let label: Option<Retained<NSString>> = unsafe { msg_send![self, label] };
        label.map(|s| s.to_string())
    }

    fn set_label(&self, label: Option<&str>) {
        unsafe {
            let _: () = msg_send![self, setLabel: label.map(NSString::from_str).as_deref()];
        }
    }

    fn install_name(&self) -> String {
        let s: Retained<NSString> = unsafe { msg_send![self, installName] };
        s.to_string()
    }
}
