use objc2::{extern_class, extern_conformance, extern_methods, runtime::NSObject};
use objc2_foundation::{CopyingHelper, NSCopying, NSObjectProtocol};

use super::{MTLLoadAction, MTLStoreAction, MTLStoreActionOptions};

extern_class!(
    /// Common attachment descriptor fields.
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLRenderPassAttachmentDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for MTLRenderPassAttachmentDescriptor {}
);

unsafe impl CopyingHelper for MTLRenderPassAttachmentDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLRenderPassAttachmentDescriptor {}
);

impl MTLRenderPassAttachmentDescriptor {
    extern_methods!(
        /// The mipmap level of the texture to be used for rendering. Default is zero.
        #[unsafe(method(level))]
        #[unsafe(method_family = none)]
        pub fn level(&self) -> usize;

        /// Setter for [`level`][Self::level].
        #[unsafe(method(setLevel:))]
        #[unsafe(method_family = none)]
        pub fn set_level(&self, level: usize);

        /// The slice of the texture to be used for rendering. Default is zero.
        #[unsafe(method(slice))]
        #[unsafe(method_family = none)]
        pub fn slice(&self) -> usize;

        /// Setter for [`slice`][Self::slice].
        #[unsafe(method(setSlice:))]
        #[unsafe(method_family = none)]
        pub fn set_slice(&self, slice: usize);

        /// The depth plane of the texture to be used for rendering. Default is zero.
        #[unsafe(method(depthPlane))]
        #[unsafe(method_family = none)]
        pub fn depth_plane(&self) -> usize;

        /// Setter for [`depth_plane`][Self::depth_plane].
        #[unsafe(method(setDepthPlane:))]
        #[unsafe(method_family = none)]
        pub fn set_depth_plane(&self, depth_plane: usize);

        /// The action to be performed at the beginning of a render pass.
        #[unsafe(method(loadAction))]
        #[unsafe(method_family = none)]
        pub fn load_action(&self) -> MTLLoadAction;

        /// Setter for [`load_action`][Self::load_action].
        #[unsafe(method(setLoadAction:))]
        #[unsafe(method_family = none)]
        pub fn set_load_action(&self, load_action: MTLLoadAction);

        /// The action to be performed at the end of a render pass.
        #[unsafe(method(storeAction))]
        #[unsafe(method_family = none)]
        pub fn store_action(&self) -> MTLStoreAction;

        /// Setter for [`store_action`][Self::store_action].
        #[unsafe(method(setStoreAction:))]
        #[unsafe(method_family = none)]
        pub fn set_store_action(&self, store_action: MTLStoreAction);

        /// Optional configuration for the store action performed at the end of a render pass.
        #[unsafe(method(storeActionOptions))]
        #[unsafe(method_family = none)]
        pub fn store_action_options(&self) -> MTLStoreActionOptions;

        /// Setter for [`store_action_options`][Self::store_action_options].
        #[unsafe(method(setStoreActionOptions:))]
        #[unsafe(method_family = none)]
        pub fn set_store_action_options(&self, options: MTLStoreActionOptions);
    );
}
