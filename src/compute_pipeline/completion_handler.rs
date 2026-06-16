use std::ops::Deref;

use block2::{Block, RcBlock};
use objc2::{rc::Retained, runtime::ProtocolObject};
use objc2_foundation::NSError;

use super::{MTLComputePipelineReflection, MTLComputePipelineState};

/// A completion handler invoked when an asynchronous compute pipeline creation finishes.
///
/// Signature mirrors Metal's
/// `void (^MTLNewComputePipelineStateCompletionHandler)(id<MTLComputePipelineState> state, NSError *error)`.
pub struct NewComputePipelineStateCompletionHandler(
    RcBlock<dyn Fn(*mut ProtocolObject<dyn MTLComputePipelineState>, *mut NSError)>,
);

impl NewComputePipelineStateCompletionHandler {
    pub fn new<F>(handler: F) -> Self
    where
        F: Fn(Option<Retained<ProtocolObject<dyn MTLComputePipelineState>>>, *mut NSError) + 'static,
    {
        Self(RcBlock::new(move |state_ptr: *mut ProtocolObject<dyn MTLComputePipelineState>, error: *mut NSError| {
            let state = unsafe { Retained::from_raw(state_ptr) };
            handler(state, error);
        }))
    }
}

impl Deref for NewComputePipelineStateCompletionHandler {
    type Target = Block<dyn Fn(*mut ProtocolObject<dyn MTLComputePipelineState>, *mut NSError)>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// A completion handler invoked when an asynchronous compute pipeline creation finishes,
/// also delivering reflection info.
///
/// Signature mirrors Metal's `void (^MTLNewComputePipelineStateWithReflectionCompletionHandler)(
/// id<MTLComputePipelineState> state, MTLComputePipelineReflection *reflection, NSError *error)`.
pub struct NewComputePipelineStateWithReflectionCompletionHandler(
    RcBlock<dyn Fn(*mut ProtocolObject<dyn MTLComputePipelineState>, *mut MTLComputePipelineReflection, *mut NSError)>,
);

impl NewComputePipelineStateWithReflectionCompletionHandler {
    pub fn new<F>(handler: F) -> Self
    where
        F: Fn(
                Option<Retained<ProtocolObject<dyn MTLComputePipelineState>>>,
                Option<Retained<MTLComputePipelineReflection>>,
                *mut NSError,
            ) + 'static,
    {
        Self(RcBlock::new(
            move |state_ptr: *mut ProtocolObject<dyn MTLComputePipelineState>,
                  reflection_ptr: *mut MTLComputePipelineReflection,
                  error: *mut NSError| {
                let state = unsafe { Retained::from_raw(state_ptr) };
                let reflection = unsafe { Retained::from_raw(reflection_ptr) };
                handler(state, reflection, error);
            },
        ))
    }
}

impl Deref for NewComputePipelineStateWithReflectionCompletionHandler {
    type Target = Block<
        dyn Fn(*mut ProtocolObject<dyn MTLComputePipelineState>, *mut MTLComputePipelineReflection, *mut NSError),
    >;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
