use objc2::{
    Encode, Encoding, RefEncode, extern_class, extern_conformance, extern_methods,
    runtime::NSObject,
};
use objc2_foundation::{CopyingHelper, NSCopying, NSObjectProtocol};

use super::MTLRenderPassAttachmentDescriptor;

/// Controls the MSAA stencil resolve operation.
#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MTLMultisampleStencilResolveFilter {
    /// The stencil sample corresponding to sample 0. Default.
    Sample0 = 0,
    /// The stencil sample corresponding to the depth-resolved sample.
    DepthResolvedSample = 1,
}

unsafe impl Encode for MTLMultisampleStencilResolveFilter {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for MTLMultisampleStencilResolveFilter {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

extern_class!(
    /// Stencil attachment descriptor for a render pass.
    #[unsafe(super(MTLRenderPassAttachmentDescriptor, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLRenderPassStencilAttachmentDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for MTLRenderPassStencilAttachmentDescriptor {}
);

unsafe impl CopyingHelper for MTLRenderPassStencilAttachmentDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLRenderPassStencilAttachmentDescriptor {}
);

impl MTLRenderPassStencilAttachmentDescriptor {
    extern_methods!(
        /// The clear stencil value to be used if the load action is Clear.
        #[unsafe(method(clearStencil))]
        #[unsafe(method_family = none)]
        pub fn clear_stencil(&self) -> u32;

        /// Setter for [`clear_stencil`][Self::clear_stencil].
        #[unsafe(method(setClearStencil:))]
        #[unsafe(method_family = none)]
        pub fn set_clear_stencil(&self, clear_stencil: u32);

        /// The filter to be used for stencil multisample resolve. Defaults to Sample0.
        #[unsafe(method(stencilResolveFilter))]
        #[unsafe(method_family = none)]
        pub fn stencil_resolve_filter(&self) -> MTLMultisampleStencilResolveFilter;

        /// Setter for [`stencil_resolve_filter`][Self::stencil_resolve_filter].
        #[unsafe(method(setStencilResolveFilter:))]
        #[unsafe(method_family = none)]
        pub fn set_stencil_resolve_filter(&self, filter: MTLMultisampleStencilResolveFilter);
    );
}
