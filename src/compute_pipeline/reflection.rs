use objc2::{extern_class, extern_conformance, msg_send, rc::Retained, runtime::ProtocolObject};
use objc2_foundation::{NSArray, NSObject, NSObjectProtocol};

use crate::argument::{MTLArgument, MTLBinding};

extern_class!(
    /// Reflection info for a compute pipeline.
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLComputePipelineReflection;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLComputePipelineReflection {}
);

#[allow(unused)]
impl MTLComputePipelineReflection {
    /// Bindings for this pipeline.
    fn bindings(&self) -> Option<Box<[Retained<ProtocolObject<dyn MTLBinding>>]>> {
        let array: Option<Retained<NSArray<ProtocolObject<dyn MTLBinding>>>> =
            unsafe { msg_send![self, bindings] };
        array.map(|a| a.to_vec().into_boxed_slice())
    }

    /// Deprecated: use `bindings` instead.
    fn arguments(&self) -> Option<Box<[Retained<MTLArgument>]>> {
        let array: Option<Retained<NSArray<MTLArgument>>> = unsafe { msg_send![self, arguments] };
        array.map(|a| a.to_vec().into_boxed_slice())
    }
}
