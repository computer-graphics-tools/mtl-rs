use objc2::{extern_protocol, rc::Retained, runtime::ProtocolObject};

use crate::{
    MTLCommandEncoder,
    render_command_encoder::MTLRenderCommandEncoder,
    render_pass::{MTLStoreAction, MTLStoreActionOptions},
};

extern_protocol!(
    /// Parallel render command encoder interface.
    ///
    /// Availability: macOS 10.11+, iOS 8.0+
    pub unsafe trait MTLParallelRenderCommandEncoder: MTLCommandEncoder {
        /// Return a new autoreleased render command encoder to encode on a different thread.
        #[unsafe(method(renderCommandEncoder))]
        #[unsafe(method_family = none)]
        fn render_command_encoder(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn MTLRenderCommandEncoder>>>;

        /// Finalize color store action for a given color attachment.
        #[unsafe(method(setColorStoreAction:atIndex:))]
        #[unsafe(method_family = none)]
        unsafe fn set_color_store_action_at_index(
            &self,
            store_action: MTLStoreAction,
            color_attachment_index: usize,
        );

        /// Finalize depth store action.
        #[unsafe(method(setDepthStoreAction:))]
        #[unsafe(method_family = none)]
        unsafe fn set_depth_store_action(&self, store_action: MTLStoreAction);

        /// Finalize stencil store action.
        #[unsafe(method(setStencilStoreAction:))]
        #[unsafe(method_family = none)]
        unsafe fn set_stencil_store_action(&self, store_action: MTLStoreAction);

        /// Finalize color store action options for a given color attachment.
        #[unsafe(method(setColorStoreActionOptions:atIndex:))]
        #[unsafe(method_family = none)]
        unsafe fn set_color_store_action_options_at_index(
            &self,
            store_action_options: MTLStoreActionOptions,
            color_attachment_index: usize,
        );

        /// Finalize depth store action options.
        #[unsafe(method(setDepthStoreActionOptions:))]
        #[unsafe(method_family = none)]
        unsafe fn set_depth_store_action_options(
            &self,
            store_action_options: MTLStoreActionOptions,
        );

        /// Finalize stencil store action options.
        #[unsafe(method(setStencilStoreActionOptions:))]
        #[unsafe(method_family = none)]
        unsafe fn set_stencil_store_action_options(
            &self,
            store_action_options: MTLStoreActionOptions,
        );
    }
);
