use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
    runtime::{NSObject, ProtocolObject},
};
use objc2_foundation::{CopyingHelper, NSCopying, NSObjectProtocol};

use super::{
    MTLRenderPassColorAttachmentDescriptorArray, MTLRenderPassDepthAttachmentDescriptor,
    MTLRenderPassSampleBufferAttachmentDescriptorArray, MTLRenderPassStencilAttachmentDescriptor,
    MTLVisibilityResultType,
};
use crate::{MTLBuffer, MTLSamplePosition, MTLRasterizationRateMap};

extern_class!(
    /// A collection of attachments used to create a render command encoder.
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLRenderPassDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for MTLRenderPassDescriptor {}
);

unsafe impl CopyingHelper for MTLRenderPassDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLRenderPassDescriptor {}
);

impl MTLRenderPassDescriptor {
    extern_methods!(
        /// Create a default frame buffer descriptor.
        #[unsafe(method(renderPassDescriptor))]
        #[unsafe(method_family = none)]
        pub fn render_pass_descriptor() -> Retained<MTLRenderPassDescriptor>;

        #[unsafe(method(colorAttachments))]
        #[unsafe(method_family = none)]
        pub fn color_attachments(&self) -> Retained<MTLRenderPassColorAttachmentDescriptorArray>;

        #[unsafe(method(depthAttachment))]
        #[unsafe(method_family = none)]
        pub fn depth_attachment(&self) -> Retained<MTLRenderPassDepthAttachmentDescriptor>;

        #[unsafe(method(setDepthAttachment:))]
        #[unsafe(method_family = none)]
        pub fn set_depth_attachment(&self, depth: Option<&MTLRenderPassDepthAttachmentDescriptor>);

        #[unsafe(method(stencilAttachment))]
        #[unsafe(method_family = none)]
        pub fn stencil_attachment(&self) -> Retained<MTLRenderPassStencilAttachmentDescriptor>;

        #[unsafe(method(setStencilAttachment:))]
        #[unsafe(method_family = none)]
        pub fn set_stencil_attachment(
            &self,
            stencil: Option<&MTLRenderPassStencilAttachmentDescriptor>,
        );

        /// Buffer into which samples passing the depth and stencil tests are counted.
        #[unsafe(method(visibilityResultBuffer))]
        #[unsafe(method_family = none)]
        pub fn visibility_result_buffer(&self) -> Option<Retained<ProtocolObject<dyn MTLBuffer>>>;

        /// Setter for [`visibility_result_buffer`][Self::visibility_result_buffer].
        #[unsafe(method(setVisibilityResultBuffer:))]
        #[unsafe(method_family = none)]
        pub fn set_visibility_result_buffer(&self, buffer: Option<&ProtocolObject<dyn MTLBuffer>>);

        /// The number of active layers.
        #[unsafe(method(renderTargetArrayLength))]
        #[unsafe(method_family = none)]
        pub fn render_target_array_length(&self) -> usize;

        #[unsafe(method(setRenderTargetArrayLength:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_render_target_array_length(&self, len: usize);

        /// The per sample size in bytes of the largest explicit imageblock layout in the render pass.
        #[unsafe(method(imageblockSampleLength))]
        #[unsafe(method_family = none)]
        pub fn imageblock_sample_length(&self) -> usize;

        #[unsafe(method(setImageblockSampleLength:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_imageblock_sample_length(&self, len: usize);

        /// The per tile size in bytes of the persistent threadgroup memory allocation.
        #[unsafe(method(threadgroupMemoryLength))]
        #[unsafe(method_family = none)]
        pub fn threadgroup_memory_length(&self) -> usize;

        #[unsafe(method(setThreadgroupMemoryLength:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_threadgroup_memory_length(&self, len: usize);

        /// The width in pixels of the tile.
        #[unsafe(method(tileWidth))]
        #[unsafe(method_family = none)]
        pub fn tile_width(&self) -> usize;

        #[unsafe(method(setTileWidth:))]
        #[unsafe(method_family = none)]
        pub fn set_tile_width(&self, width: usize);

        /// The height in pixels of the tile.
        #[unsafe(method(tileHeight))]
        #[unsafe(method_family = none)]
        pub fn tile_height(&self) -> usize;

        #[unsafe(method(setTileHeight:))]
        #[unsafe(method_family = none)]
        pub fn set_tile_height(&self, height: usize);

        /// The raster sample count for the render pass when no attachments are given.
        #[unsafe(method(defaultRasterSampleCount))]
        #[unsafe(method_family = none)]
        pub fn default_raster_sample_count(&self) -> usize;

        #[unsafe(method(setDefaultRasterSampleCount:))]
        #[unsafe(method_family = none)]
        pub fn set_default_raster_sample_count(&self, count: usize);

        /// Constrain the render target width.
        #[unsafe(method(renderTargetWidth))]
        #[unsafe(method_family = none)]
        pub fn render_target_width(&self) -> usize;

        #[unsafe(method(setRenderTargetWidth:))]
        #[unsafe(method_family = none)]
        pub fn set_render_target_width(&self, width: usize);

        /// Constrain the render target height.
        #[unsafe(method(renderTargetHeight))]
        #[unsafe(method_family = none)]
        pub fn render_target_height(&self) -> usize;

        #[unsafe(method(setRenderTargetHeight:))]
        #[unsafe(method_family = none)]
        pub fn set_render_target_height(&self, height: usize);

        /// Configure custom sample positions for MSAA.
        /// Safety: `positions` must be a valid pointer or null.
        #[unsafe(method(setSamplePositions:count:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_sample_positions(
            &self,
            positions: *const MTLSamplePosition,
            count: usize,
        );

        /// Retrieve previously configured custom sample positions.
        /// Safety: `positions` must be a valid pointer or null.
        #[unsafe(method(getSamplePositions:count:))]
        #[unsafe(method_family = none)]
        pub unsafe fn get_sample_positions(
            &self,
            positions: *mut MTLSamplePosition,
            count: usize,
        ) -> usize;

        /// The variable rasterization rate map for this pass.
        #[unsafe(method(rasterizationRateMap))]
        #[unsafe(method_family = none)]
        pub fn rasterization_rate_map(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn MTLRasterizationRateMap>>>;

        /// Setter for [`rasterization_rate_map`][Self::rasterization_rate_map].
        #[unsafe(method(setRasterizationRateMap:))]
        #[unsafe(method_family = none)]
        pub fn set_rasterization_rate_map(
            &self,
            map: Option<&ProtocolObject<dyn MTLRasterizationRateMap>>,
        );

        /// An array of sample buffers and associated sample indices.
        #[unsafe(method(sampleBufferAttachments))]
        #[unsafe(method_family = none)]
        pub fn sample_buffer_attachments(
            &self,
        ) -> Retained<MTLRenderPassSampleBufferAttachmentDescriptorArray>;

        /// Specifies if Metal accumulates visibility results between render encoders or resets them.
        #[unsafe(method(visibilityResultType))]
        #[unsafe(method_family = none)]
        pub unsafe fn visibility_result_type(&self) -> MTLVisibilityResultType;

        /// Setter for [`visibility_result_type`][Self::visibility_result_type].
        #[unsafe(method(setVisibilityResultType:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_visibility_result_type(&self, v: MTLVisibilityResultType);
    );
}

impl MTLRenderPassDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
