use objc2::{Message, extern_protocol, msg_send, rc::Retained, runtime::ProtocolObject};
use objc2_foundation::{NSObjectProtocol, NSString};

extern_protocol!(
    /// A descriptor for a single counter.
    pub unsafe trait MTLCounter: NSObjectProtocol + Send + Sync {}
);

#[allow(unused)]
pub trait MTLCounterExt: MTLCounter + Message {
    fn name(&self) -> String;
}

impl MTLCounterExt for ProtocolObject<dyn MTLCounter> {
    fn name(&self) -> String {
        let ns: Retained<NSString> = unsafe { msg_send![self, name] };
        ns.to_string()
    }
}
