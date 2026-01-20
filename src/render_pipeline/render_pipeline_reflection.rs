use objc2::{extern_class, extern_conformance, runtime::NSObject};
use objc2_foundation::NSObjectProtocol;

extern_class!(
    /// Reflection info for a render pipeline.
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLRenderPipelineReflection;
);

unsafe impl Send for MTLRenderPipelineReflection {}
unsafe impl Sync for MTLRenderPipelineReflection {}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLRenderPipelineReflection {}
);
