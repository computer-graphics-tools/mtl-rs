use objc2::{extern_class, extern_conformance, msg_send, rc::Retained, runtime::ProtocolObject};
use objc2_foundation::{NSArray, NSObject, NSObjectProtocol};

use crate::argument::{MTLArgument, MTLBinding};

extern_class!(
    /// Reflection info for a compute pipeline.
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLComputePipelineReflection;
);

// SAFETY (Send + Sync): [Apple lists this class's conformances](https://developer.apple.com/documentation/metal/mtlcomputepipelinereflection):
//
// > `Sendable`
//
// This permits transferring and concurrently sharing instances of this reference type.
unsafe impl Send for MTLComputePipelineReflection {}
unsafe impl Sync for MTLComputePipelineReflection {}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLComputePipelineReflection {}
);

impl MTLComputePipelineReflection {
    /// Resource bindings reflected from this compute pipeline.
    pub fn bindings(&self) -> Box<[Retained<ProtocolObject<dyn MTLBinding>>]> {
        let bindings: Retained<NSArray<ProtocolObject<dyn MTLBinding>>> = unsafe { msg_send![self, bindings] };
        bindings.to_vec().into_boxed_slice()
    }

    /// Deprecated argument reflection; use [`bindings`][Self::bindings].
    #[deprecated(note = "use bindings")]
    pub fn arguments(&self) -> Box<[Retained<MTLArgument>]> {
        let arguments: Retained<NSArray<MTLArgument>> = unsafe { msg_send![self, arguments] };
        arguments.to_vec().into_boxed_slice()
    }
}
