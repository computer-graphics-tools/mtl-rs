use std::path::Path;

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
    }
);

#[allow(unused)]
pub trait MTLDynamicLibraryExt: MTLDynamicLibrary + Message {
    /// A string to help identify this object.
    fn label(&self) -> Option<String>;
    /// Setter for `label`.
    fn set_label(
        &self,
        label: Option<&str>,
    );
    /// The install name of this dynamic library.
    fn install_name(&self) -> String;
    /// Writes the contents of the dynamic library to a file path.
    fn serialize_to_path(
        &self,
        path: &Path,
    ) -> Result<(), Retained<NSError>>;
}

impl MTLDynamicLibraryExt for ProtocolObject<dyn MTLDynamicLibrary> {
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

    fn install_name(&self) -> String {
        let s: Retained<NSString> = unsafe { msg_send![self, installName] };
        s.to_string()
    }

    fn serialize_to_path(
        &self,
        path: &Path,
    ) -> Result<(), Retained<NSError>> {
        let url = NSURL::from_file_path(path).expect("path must be a valid file URL path");
        unsafe { msg_send![self, serializeToURL: &*url, error: _] }
    }
}
