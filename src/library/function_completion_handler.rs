use block2::{Block, RcBlock};
use objc2::{rc::Retained, runtime::ProtocolObject};
use objc2_foundation::NSError;

use super::MTLFunction;
use crate::MetalError;

/// A completion handler invoked when a function creation finishes.
///
/// Signature mirrors the function-creation completion blocks on `MTLLibrary`.
pub struct LibraryFunctionCompletionHandler(RcBlock<dyn Fn(*mut ProtocolObject<dyn MTLFunction>, *mut NSError)>);

impl LibraryFunctionCompletionHandler {
    /// Creates a callback with captures that can be transferred and shared with Metal's worker threads.
    ///
    /// [Apple's callback declaration](https://developer.apple.com/documentation/metal/mtllibrary/makefunction(name:constantvalues:completionhandler:)) includes:
    ///
    /// > `completionHandler: @escaping @Sendable`
    pub fn new<F>(handler: F) -> Self
    where
        F: Fn(Option<Retained<ProtocolObject<dyn MTLFunction>>>, Option<MetalError>) + Send + Sync + 'static,
    {
        Self(RcBlock::new(move |function_ptr: *mut ProtocolObject<dyn MTLFunction>, error: *mut NSError| {
            let function = unsafe { Retained::retain(function_ptr) };
            let error = unsafe { MetalError::from_unretained(error) };
            handler(function, error);
        }))
    }

    pub(super) fn as_block(&self) -> &Block<dyn Fn(*mut ProtocolObject<dyn MTLFunction>, *mut NSError)> {
        &self.0
    }
}
