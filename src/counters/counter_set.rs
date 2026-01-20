use objc2::{Message, extern_protocol, msg_send, rc::Retained, runtime::ProtocolObject};
use objc2_foundation::{NSArray, NSObjectProtocol, NSString};

use super::MTLCounter;

extern_protocol!(
    /// A collection of counters that the device can capture in a single pass.
    pub unsafe trait MTLCounterSet: NSObjectProtocol + Send + Sync {}
);

#[allow(unused)]
pub trait MTLCounterSetExt: MTLCounterSet + Message {
    fn name(&self) -> String;
    /// The counters array contains all the counters that will be written
    /// when a counter sample is collected. Counters that do not appear in this array
    /// will not be written to the resolved buffer when the samples are resolved.
    fn counters(&self) -> Box<[Retained<ProtocolObject<dyn MTLCounter>>]>;
}

impl MTLCounterSetExt for ProtocolObject<dyn MTLCounterSet> {
    fn name(&self) -> String {
        let ns: Retained<NSString> = unsafe { msg_send![self, name] };
        ns.to_string()
    }

    fn counters(&self) -> Box<[Retained<ProtocolObject<dyn MTLCounter>>]> {
        let arr: Retained<NSArray<ProtocolObject<dyn MTLCounter>>> =
            unsafe { msg_send![self, counters] };
        arr.to_vec().into_boxed_slice()
    }
}
