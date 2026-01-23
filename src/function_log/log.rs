use objc2::{Message, extern_protocol, msg_send, rc::Retained, runtime::ProtocolObject};
use objc2_foundation::{NSObjectProtocol, NSString};

use crate::{MTLFunction, MTLFunctionLogDebugLocation, MTLFunctionLogType};

extern_protocol!(
    /// Function log information (from `MTLFunctionLog`).
    ///
    /// Availability: macOS 11.0+, iOS 14.0+
    pub unsafe trait MTLFunctionLog: NSObjectProtocol {
        #[unsafe(method(r#type))]
        #[unsafe(method_family = none)]
        fn r#type(&self) -> MTLFunctionLogType;

        #[unsafe(method(function))]
        #[unsafe(method_family = none)]
        fn function(&self) -> Option<Retained<ProtocolObject<dyn MTLFunction>>>;

        #[unsafe(method(debugLocation))]
        #[unsafe(method_family = none)]
        fn debug_location(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn MTLFunctionLogDebugLocation>>>;
    }
);

#[allow(unused)]
pub trait MTLFunctionLogExt: MTLFunctionLog + Message {
    fn encoder_label(&self) -> Option<String>;
}

impl MTLFunctionLogExt for ProtocolObject<dyn MTLFunctionLog> {
    fn encoder_label(&self) -> Option<String> {
        let s: Option<Retained<NSString>> = unsafe { msg_send![self, encoderLabel] };
        s.map(|v| v.to_string())
    }
}
