use block2::{Block, RcBlock};
use objc2::{rc::Retained, runtime::ProtocolObject};
use objc2_foundation::NSError;

use super::{MTLComputePipelineReflection, MTLComputePipelineState};
use crate::{CallbackBlock, MetalError};

/// A completion handler invoked when an asynchronous compute pipeline creation finishes.
///
/// Signature mirrors Metal's
/// `void (^MTLNewComputePipelineStateCompletionHandler)(id<MTLComputePipelineState> state, NSError *error)`.
pub struct NewComputePipelineStateCompletionHandler(
    RcBlock<dyn Fn(*mut ProtocolObject<dyn MTLComputePipelineState>, *mut NSError)>,
);

impl NewComputePipelineStateCompletionHandler {
    /// Creates a callback with captures that can be transferred and shared with Metal's worker threads.
    ///
    /// [Apple's callback declaration](https://developer.apple.com/documentation/metal/mtldevice/makecomputepipelinestate(function:completionhandler:)) includes:
    ///
    /// > `completionHandler: @escaping @Sendable`
    pub fn new<F>(handler: F) -> Self
    where
        F: Fn(Option<Retained<ProtocolObject<dyn MTLComputePipelineState>>>, Option<MetalError>)
            + Send
            + Sync
            + 'static,
    {
        Self(RcBlock::new(move |state_ptr: *mut ProtocolObject<dyn MTLComputePipelineState>, error: *mut NSError| {
            let state = unsafe { Retained::retain(state_ptr) };
            let error = unsafe { MetalError::from_unretained(error) };
            handler(state, error);
        }))
    }
}

impl CallbackBlock for NewComputePipelineStateCompletionHandler {
    type Signature = dyn Fn(*mut ProtocolObject<dyn MTLComputePipelineState>, *mut NSError);

    fn as_block(&self) -> &Block<Self::Signature> {
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
    /// Creates a callback with captures that can be transferred and shared with Metal's worker threads.
    ///
    /// [Apple's callback declaration](https://developer.apple.com/documentation/metal/mtldevice/makecomputepipelinestate(function:options:completionhandler:)) includes:
    ///
    /// > `completionHandler: @escaping @Sendable`
    pub fn new<F>(handler: F) -> Self
    where
        F: Fn(
                Option<Retained<ProtocolObject<dyn MTLComputePipelineState>>>,
                Option<Retained<MTLComputePipelineReflection>>,
                Option<MetalError>,
            ) + Send
            + Sync
            + 'static,
    {
        Self(RcBlock::new(
            move |state_ptr: *mut ProtocolObject<dyn MTLComputePipelineState>,
                  reflection_ptr: *mut MTLComputePipelineReflection,
                  error: *mut NSError| {
                let state = unsafe { Retained::retain(state_ptr) };
                let reflection = unsafe { Retained::retain(reflection_ptr) };
                let error = unsafe { MetalError::from_unretained(error) };
                handler(state, reflection, error);
            },
        ))
    }
}

impl CallbackBlock for NewComputePipelineStateWithReflectionCompletionHandler {
    type Signature =
        dyn Fn(*mut ProtocolObject<dyn MTLComputePipelineState>, *mut MTLComputePipelineReflection, *mut NSError);

    fn as_block(&self) -> &Block<Self::Signature> {
        &self.0
    }
}
