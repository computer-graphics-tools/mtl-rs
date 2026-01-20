use std::ops::Deref;

use block2::{Block, RcBlock};
use objc2::{rc::Retained, runtime::ProtocolObject};
use objc2_foundation::NSError;

use super::MTLFunction;

/// A completion handler invoked when a function creation finishes.
///
/// Signature mirrors Metal's `void (^MTLNewLibraryCompletionHandler)(id<MTLFunction> function, NSError *error)`.
pub struct LibraryFunctionCompletionHandler(
    RcBlock<dyn Fn(*mut ProtocolObject<dyn MTLFunction>, *mut NSError)>,
);

impl LibraryFunctionCompletionHandler {
    pub fn new<F>(handler: F) -> Self
    where
        F: Fn(Option<Retained<ProtocolObject<dyn MTLFunction>>>, *mut NSError) + 'static,
    {
        Self(RcBlock::new(
            move |function_ptr: *mut ProtocolObject<dyn MTLFunction>, error: *mut NSError| {
                let function = unsafe { Retained::from_raw(function_ptr) };
                handler(function, error);
            },
        ))
    }
}

impl Deref for LibraryFunctionCompletionHandler {
    type Target = Block<dyn Fn(*mut ProtocolObject<dyn MTLFunction>, *mut NSError)>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
