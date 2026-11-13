use objc2::{
    extern_class, extern_conformance, msg_send,
    rc::Retained,
    runtime::{NSObject, ProtocolObject},
};
use objc2_foundation::{NSArray, NSObjectProtocol};

use crate::{MTLArgument, MTLBinding};

extern_class!(
    /// Reflection info for a render pipeline.
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLRenderPipelineReflection;
);

// SAFETY (Send + Sync): [Apple lists this class's conformances](https://developer.apple.com/documentation/metal/mtlrenderpipelinereflection):
//
// > `Sendable`
//
// This permits transferring and concurrently sharing instances of this reference type.
unsafe impl Send for MTLRenderPipelineReflection {}
unsafe impl Sync for MTLRenderPipelineReflection {}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLRenderPipelineReflection {}
);

impl MTLRenderPipelineReflection {
    /// The vertex-stage resource bindings.
    pub fn vertex_bindings(&self) -> Box<[Retained<ProtocolObject<dyn MTLBinding>>]> {
        let bindings: Retained<NSArray<ProtocolObject<dyn MTLBinding>>> = unsafe { msg_send![self, vertexBindings] };
        bindings.to_vec().into_boxed_slice()
    }

    /// The fragment-stage resource bindings.
    pub fn fragment_bindings(&self) -> Box<[Retained<ProtocolObject<dyn MTLBinding>>]> {
        let bindings: Retained<NSArray<ProtocolObject<dyn MTLBinding>>> = unsafe { msg_send![self, fragmentBindings] };
        bindings.to_vec().into_boxed_slice()
    }

    /// The tile-stage resource bindings.
    pub fn tile_bindings(&self) -> Box<[Retained<ProtocolObject<dyn MTLBinding>>]> {
        let bindings: Retained<NSArray<ProtocolObject<dyn MTLBinding>>> = unsafe { msg_send![self, tileBindings] };
        bindings.to_vec().into_boxed_slice()
    }

    /// The object-stage resource bindings.
    pub fn object_bindings(&self) -> Box<[Retained<ProtocolObject<dyn MTLBinding>>]> {
        let bindings: Retained<NSArray<ProtocolObject<dyn MTLBinding>>> = unsafe { msg_send![self, objectBindings] };
        bindings.to_vec().into_boxed_slice()
    }

    /// The mesh-stage resource bindings.
    pub fn mesh_bindings(&self) -> Box<[Retained<ProtocolObject<dyn MTLBinding>>]> {
        let bindings: Retained<NSArray<ProtocolObject<dyn MTLBinding>>> = unsafe { msg_send![self, meshBindings] };
        bindings.to_vec().into_boxed_slice()
    }

    /// The deprecated vertex-stage argument reflection.
    #[deprecated(note = "use vertex_bindings")]
    pub fn vertex_arguments(&self) -> Option<Box<[Retained<MTLArgument>]>> {
        let arguments: Option<Retained<NSArray<MTLArgument>>> = unsafe { msg_send![self, vertexArguments] };
        arguments.map(|arguments| arguments.to_vec().into_boxed_slice())
    }

    /// The deprecated fragment-stage argument reflection.
    #[deprecated(note = "use fragment_bindings")]
    pub fn fragment_arguments(&self) -> Option<Box<[Retained<MTLArgument>]>> {
        let arguments: Option<Retained<NSArray<MTLArgument>>> = unsafe { msg_send![self, fragmentArguments] };
        arguments.map(|arguments| arguments.to_vec().into_boxed_slice())
    }

    /// The deprecated tile-stage argument reflection.
    #[deprecated(note = "use tile_bindings")]
    pub fn tile_arguments(&self) -> Option<Box<[Retained<MTLArgument>]>> {
        let arguments: Option<Retained<NSArray<MTLArgument>>> = unsafe { msg_send![self, tileArguments] };
        arguments.map(|arguments| arguments.to_vec().into_boxed_slice())
    }
}
