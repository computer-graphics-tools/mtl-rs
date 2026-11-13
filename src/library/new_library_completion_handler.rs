use block2::{Block, RcBlock};
use objc2::{rc::Retained, runtime::ProtocolObject};
use objc2_foundation::NSError;

use super::MTLLibrary;
use crate::{CallbackBlock, MetalError};

/// A completion handler invoked when an asynchronous library creation finishes.
///
/// Signature mirrors Metal's `void (^MTLNewLibraryCompletionHandler)(id<MTLLibrary> library, NSError *error)`.
pub struct NewLibraryCompletionHandler(RcBlock<dyn Fn(*mut ProtocolObject<dyn MTLLibrary>, *mut NSError)>);

impl NewLibraryCompletionHandler {
    /// Creates a callback with captures that can be transferred and shared with Metal's worker threads.
    ///
    /// [Apple's callback declaration](https://developer.apple.com/documentation/metal/mtldevice/makelibrary(source:options:completionhandler:)) includes:
    ///
    /// > `completionHandler: @escaping @Sendable`
    pub fn new<F>(handler: F) -> Self
    where
        F: Fn(Option<Retained<ProtocolObject<dyn MTLLibrary>>>, Option<MetalError>) + Send + Sync + 'static,
    {
        Self(RcBlock::new(move |library_ptr: *mut ProtocolObject<dyn MTLLibrary>, error: *mut NSError| {
            let library = unsafe { Retained::retain(library_ptr) };
            let error = unsafe { MetalError::from_unretained(error) };
            handler(library, error);
        }))
    }
}

impl CallbackBlock for NewLibraryCompletionHandler {
    type Signature = dyn Fn(*mut ProtocolObject<dyn MTLLibrary>, *mut NSError);

    fn as_block(&self) -> &Block<Self::Signature> {
        &self.0
    }
}
