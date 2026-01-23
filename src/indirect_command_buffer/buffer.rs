use objc2::{extern_protocol, rc::Retained, runtime::ProtocolObject};
use objc2_foundation::{NSObjectProtocol, NSRange};

use crate::{MTLIndirectComputeCommand, MTLIndirectRenderCommand, types::MTLResourceID};

extern_protocol!(
    /// Bridged protocol for `MTLIndirectCommandBuffer`.
    ///
    /// Availability: macOS 10.14+, iOS 12.0+
    pub unsafe trait MTLIndirectCommandBuffer: NSObjectProtocol {
        #[unsafe(method(size))]
        #[unsafe(method_family = none)]
        fn size(&self) -> usize;

        /// Handle of the GPU resource suitable for storing in an Argument Buffer.
        ///
        /// Availability: macOS 13.0+, iOS 16.0+
        #[unsafe(method(gpuResourceID))]
        #[unsafe(method_family = none)]
        fn gpu_resource_id(&self) -> MTLResourceID;

        #[unsafe(method(resetWithRange:))]
        #[unsafe(method_family = none)]
        fn reset_with_range(&self, range: NSRange);

        #[unsafe(method(indirectRenderCommandAtIndex:))]
        #[unsafe(method_family = none)]
        fn indirect_render_command_at_index(
            &self,
            command_index: usize,
        ) -> Retained<ProtocolObject<dyn MTLIndirectRenderCommand>>;

        #[unsafe(method(indirectComputeCommandAtIndex:))]
        #[unsafe(method_family = none)]
        fn indirect_compute_command_at_index(
            &self,
            command_index: usize,
        ) -> Retained<ProtocolObject<dyn MTLIndirectComputeCommand>>;
    }
);
