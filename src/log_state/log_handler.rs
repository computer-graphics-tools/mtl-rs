use core::ptr::NonNull;
use std::ops::Deref;

use block2::{Block, RcBlock};
use objc2_foundation::NSString;

use crate::MTLLogLevel;

/// A handler invoked for GPU log messages.
pub struct MTLLogHandler(
    RcBlock<dyn Fn(*mut NSString, *mut NSString, MTLLogLevel, NonNull<NSString>)>,
);

pub struct LogMessage {
    pub category: Option<String>,
    pub subsystem: Option<String>,
    pub log_level: MTLLogLevel,
    pub message: String,
}

impl MTLLogHandler {
    pub fn new<F>(handler: F) -> Self
    where
        F: Fn(LogMessage) + 'static,
    {
        Self(RcBlock::new(
            move |category_ptr: *mut NSString,
                  subsystem_ptr: *mut NSString,
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
}

impl Deref for MTLLogHandler {
    type Target = Block<dyn Fn(*mut NSString, *mut NSString, MTLLogLevel, NonNull<NSString>)>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
