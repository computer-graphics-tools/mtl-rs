use core::ptr::NonNull;

use block2::{Block, RcBlock};
use objc2_foundation::NSString;

use crate::MTLLogLevel;

/// A handler invoked for GPU log messages.
pub struct MTLLogHandler(RcBlock<dyn Fn(*mut NSString, *mut NSString, MTLLogLevel, NonNull<NSString>)>);

/// A GPU log message with Rust-owned strings.
pub struct LogMessage {
    pub category: Option<String>,
    pub subsystem: Option<String>,
    pub log_level: MTLLogLevel,
    pub message: String,
}

impl MTLLogHandler {
    /// Creates a callback whose Foundation strings are copied into Rust-owned
    /// strings before invoking `handler`.
    ///
    /// [Apple's declaration](https://developer.apple.com/documentation/metal/mtllogstate/addloghandler(_:)):
    ///
    /// > `func addLogHandler(_ block: @escaping @Sendable (String?, String?, MTLLogLevel, String) -> Void)`
    pub fn new<F>(handler: F) -> Self
    where
        F: Fn(LogMessage) + Send + Sync + 'static,
    {
        Self(RcBlock::new(
            move |subsystem_ptr: *mut NSString,
                  category_ptr: *mut NSString,
                  level: MTLLogLevel,
                  message_nn: NonNull<NSString>| {
                let category = unsafe { category_ptr.as_ref().map(|s| s.to_string()) };
                let subsystem = unsafe { subsystem_ptr.as_ref().map(|s| s.to_string()) };
                let message = unsafe { message_nn.as_ref().to_string() };
                handler(LogMessage {
                    category,
                    subsystem,
                    log_level: level,
                    message,
                });
            },
        ))
    }

    pub(super) fn as_block(&self) -> &Block<dyn Fn(*mut NSString, *mut NSString, MTLLogLevel, NonNull<NSString>)> {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use core::ptr::NonNull;
    use std::sync::{Arc, Mutex};

    use objc2_foundation::NSString;

    use super::MTLLogHandler;
    use crate::MTLLogLevel;

    #[test]
    fn preserves_subsystem_and_category_argument_order() {
        let received = Arc::new(Mutex::new(None));
        let received_by_handler = Arc::clone(&received);
        let handler = MTLLogHandler::new(move |message| {
            *received_by_handler.lock().unwrap() = Some(message);
        });
        let subsystem = NSString::from_str("rendering");
        let category = NSString::from_str("validation");
        let message = NSString::from_str("failed");

        handler.as_block().call((
            (&*subsystem as *const NSString).cast_mut(),
            (&*category as *const NSString).cast_mut(),
            MTLLogLevel::Error,
            NonNull::from(&*message),
        ));

        let received = received.lock().unwrap().take().unwrap();
        assert_eq!(received.subsystem.as_deref(), Some("rendering"));
        assert_eq!(received.category.as_deref(), Some("validation"));
        assert_eq!(received.log_level, MTLLogLevel::Error);
        assert_eq!(received.message, "failed");
    }
}
