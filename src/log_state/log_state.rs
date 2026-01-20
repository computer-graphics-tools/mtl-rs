use objc2::{Message, extern_protocol, msg_send};
use objc2_foundation::NSObjectProtocol;

use crate::MTLLogHandler;

extern_protocol!(
    /// Log state for handling GPU log messages.
    ///
    /// Availability: macOS 15.0+, iOS 18.0+
    pub unsafe trait MTLLogState: NSObjectProtocol + Send + Sync {}
);

#[allow(unused)]
pub trait MTLLogStateExt: MTLLogState + Message {
    /// Add a log handler block.
    fn add_log_handler(&self, handler: &MTLLogHandler);
}

impl<T> MTLLogStateExt for T
where
    T: MTLLogState + Message,
{
    fn add_log_handler(&self, handler: &MTLLogHandler) {
        unsafe {
            let _: () = msg_send![self, addLogHandler: &**handler];
        }
    }
}
