use objc2::{Message, extern_protocol, msg_send, rc::Retained, runtime::ProtocolObject};
use objc2_foundation::{NSObjectProtocol, NSString, NSURL};

extern_protocol!(
    /// Location information for a function log (from `MTLFunctionLogDebugLocation`).
    ///
    /// Availability: macOS 11.0+, iOS 14.0+
    pub unsafe trait MTLFunctionLogDebugLocation: NSObjectProtocol {
        #[unsafe(method(URL))]
        #[unsafe(method_family = none)]
        fn url(&self) -> Option<Retained<NSURL>>;

        #[unsafe(method(line))]
        #[unsafe(method_family = none)]
        fn line(&self) -> usize;

        #[unsafe(method(column))]
        #[unsafe(method_family = none)]
        fn column(&self) -> usize;
    }
);

#[allow(unused)]
pub trait MTLFunctionLogDebugLocationExt: MTLFunctionLogDebugLocation + Message {
    fn function_name(&self) -> Option<String>;
}

impl MTLFunctionLogDebugLocationExt for ProtocolObject<dyn MTLFunctionLogDebugLocation> {
    fn function_name(&self) -> Option<String> {
        let s: Option<Retained<NSString>> = unsafe { msg_send![self, functionName] };
        s.map(|v| v.to_string())
    }
}
