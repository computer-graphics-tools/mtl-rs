use objc2::{extern_class, extern_conformance, runtime::NSObject};
use objc2_foundation::NSObjectProtocol;

extern_class!(
    /// Reflection info for a function in a Metal library.
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLFunctionReflection;
);

unsafe impl Send for MTLFunctionReflection {}
unsafe impl Sync for MTLFunctionReflection {}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLFunctionReflection {}
);
