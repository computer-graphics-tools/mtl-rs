use std::ops::Deref;

use block2::{Block, RcBlock};
use objc2::{rc::Retained, runtime::ProtocolObject};
use objc2_foundation::NSError;

use super::MTLLibrary;

/// A completion handler invoked when an asynchronous library creation finishes.
///
/// Signature mirrors Metal's `void (^MTLNewLibraryCompletionHandler)(id<MTLLibrary> library, NSError *error)`.
pub struct NewLibraryCompletionHandler(RcBlock<dyn Fn(*mut ProtocolObject<dyn MTLLibrary>, *mut NSError)>);

impl NewLibraryCompletionHandler {
    pub fn new<F>(handler: F) -> Self
    where
        F: Fn(Option<Retained<ProtocolObject<dyn MTLLibrary>>>, *mut NSError) + 'static,
    {
        Self(RcBlock::new(move |library_ptr: *mut ProtocolObject<dyn MTLLibrary>, error: *mut NSError| {
            let library = unsafe { Retained::from_raw(library_ptr) };
            handler(library, error);
        }))
    }
}

impl Deref for NewLibraryCompletionHandler {
    type Target = Block<dyn Fn(*mut ProtocolObject<dyn MTLLibrary>, *mut NSError)>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
