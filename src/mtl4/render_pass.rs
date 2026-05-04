use objc2::{
    extern_class, extern_conformance, extern_methods, msg_send,
    rc::{Allocated, Retained},
    runtime::ProtocolObject,
};
use objc2_foundation::{CopyingHelper, NSCopying, NSObject, NSObjectProtocol};

use crate::*;

extern_class!(
    /// Describes a render pass.
    ///
    /// You use render pass descriptors to create instances of ``MTL4RenderCommandEncoder`` and encode draw
    /// commands into instances of ``MTL4CommandBuffer``.
    ///
    /// To create render command encoders, you typically call ``MTL4CommandBuffer/renderCommandEncoderWithDescriptor:``.
    /// The ``MTL4CommandBuffer/renderCommandEncoderWithDescriptor:options:`` variant of this method allows you to specify
    /// additional options to encode a render pass in parallel from multiple CPU cores by creating *suspending* and *resuming*
    /// render passes.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metal/mtl4renderpassdescriptor?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTL4RenderPassDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for MTL4RenderPassDescriptor {}
);

unsafe impl CopyingHelper for MTL4RenderPassDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTL4RenderPassDescriptor {}
);

impl MTL4RenderPassDescriptor {
    extern_methods!(
        /// Accesses the array of state information for render attachments that store color data.
        #[unsafe(method(colorAttachments))]
        #[unsafe(method_family = none)]
        pub fn color_attachments(&self) -> Retained<MTLRenderPassColorAttachmentDescriptorArray>;

        /// Accesses state information for a render attachment that stores depth data.
        #[unsafe(method(depthAttachment))]
        #[unsafe(method_family = none)]
        pub fn depth_attachment(&self) -> Retained<MTLRenderPassDepthAttachmentDescriptor>;

        /// Setter for [`depthAttachment`][Self::depthAttachment].
        ///
        /// This is [copied][objc2_foundation::NSCopying::copy] when set.
        #[unsafe(method(setDepthAttachment:))]
        #[unsafe(method_family = none)]
        pub fn set_depth_attachment(
            &self,
            depth_attachment: Option<&MTLRenderPassDepthAttachmentDescriptor>,
        );

        /// Accesses state information for a render attachment that stores stencil data.
        #[unsafe(method(stencilAttachment))]
        #[unsafe(method_family = none)]
        pub fn stencil_attachment(&self) -> Retained<MTLRenderPassStencilAttachmentDescriptor>;

        /// Setter for [`stencilAttachment`][Self::stencilAttachment].
        ///
        /// This is [copied][objc2_foundation::NSCopying::copy] when set.
        #[unsafe(method(setStencilAttachment:))]
        #[unsafe(method_family = none)]
        pub fn set_stencil_attachment(
            &self,
            stencil_attachment: Option<&MTLRenderPassStencilAttachmentDescriptor>,
        );

        /// Assigns the number of layers that all attachments this descriptor references have.
        #[unsafe(method(renderTargetArrayLength))]
        #[unsafe(method_family = none)]
        pub fn render_target_array_length(&self) -> usize;

        /// Setter for [`renderTargetArrayLength`][Self::renderTargetArrayLength].
        #[unsafe(method(setRenderTargetArrayLength:))]
        #[unsafe(method_family = none)]
        pub fn set_render_target_array_length(
            &self,
            render_target_array_length: usize,
        );

        /// Assigns the per-sample size, in bytes, of the largest explicit imageblock layout in the render pass.
        #[unsafe(method(imageblockSampleLength))]
        #[unsafe(method_family = none)]
        pub fn imageblock_sample_length(&self) -> usize;

        /// Setter for [`imageblockSampleLength`][Self::imageblockSampleLength].
        #[unsafe(method(setImageblockSampleLength:))]
        #[unsafe(method_family = none)]
        pub fn set_imageblock_sample_length(
            &self,
            imageblock_sample_length: usize,
        );

        /// Assigns the per-tile size, in bytes, of the persistent threadgroup memory allocation of this render pass.
        #[unsafe(method(threadgroupMemoryLength))]
        #[unsafe(method_family = none)]
        pub fn threadgroup_memory_length(&self) -> usize;

        /// Setter for [`threadgroupMemoryLength`][Self::threadgroupMemoryLength].
        #[unsafe(method(setThreadgroupMemoryLength:))]
        #[unsafe(method_family = none)]
        pub fn set_threadgroup_memory_length(
            &self,
            threadgroup_memory_length: usize,
        );

        /// The width of the tiles, in pixels, a render pass you create with this descriptor applies to its attachments.
        ///
        /// For tile-based rendering, Metal divides each render attachment into smaller regions, or _tiles_.
        /// The property's default is `0`, which tells Metal to select a size that fits in tile memory.
        ///
        /// See
        /// <doc
        /// :tailor-your-apps-for-apple-gpus-and-tile-based-deferred-rendering>
        /// for more information about tiles, tile memory, and deferred rendering.
        #[unsafe(method(tileWidth))]
        #[unsafe(method_family = none)]
        pub fn tile_width(&self) -> usize;

        /// Setter for [`tileWidth`][Self::tileWidth].
        #[unsafe(method(setTileWidth:))]
        #[unsafe(method_family = none)]
        pub fn set_tile_width(
            &self,
            tile_width: usize,
        );

        /// The height of the tiles, in pixels, a render pass you create with this descriptor applies to its attachments.
        ///
        /// For tile-based rendering, Metal divides each render attachment into smaller regions, or _tiles_.
        /// The property's default is `0`, which tells Metal to select a size that fits in tile memory.
        ///
        /// See
        /// <doc
        /// :tailor-your-apps-for-apple-gpus-and-tile-based-deferred-rendering>
        /// for more information about tiles, tile memory, and deferred rendering.
        #[unsafe(method(tileHeight))]
        #[unsafe(method_family = none)]
        pub fn tile_height(&self) -> usize;

        /// Setter for [`tileHeight`][Self::tileHeight].
        #[unsafe(method(setTileHeight:))]
        #[unsafe(method_family = none)]
        pub fn set_tile_height(
            &self,
            tile_height: usize,
        );

        /// Sets the default raster sample count for the render pass when it references no attachments.
        #[unsafe(method(defaultRasterSampleCount))]
        #[unsafe(method_family = none)]
        pub fn default_raster_sample_count(&self) -> usize;

        /// Setter for [`defaultRasterSampleCount`][Self::defaultRasterSampleCount].
        #[unsafe(method(setDefaultRasterSampleCount:))]
        #[unsafe(method_family = none)]
        pub fn set_default_raster_sample_count(
            &self,
            default_raster_sample_count: usize,
        );

        /// Sets the width, in pixels, to which Metal constrains the render target.
        ///
        /// When this value is non-zero, you need to assign it to be smaller than or equal to the minimum width of all attachments.
        ///
        /// The default value of this property is `0`.
        #[unsafe(method(renderTargetWidth))]
        #[unsafe(method_family = none)]
        pub fn render_target_width(&self) -> usize;

        /// Setter for [`renderTargetWidth`][Self::renderTargetWidth].
        #[unsafe(method(setRenderTargetWidth:))]
        #[unsafe(method_family = none)]
        pub fn set_render_target_width(
            &self,
            render_target_width: usize,
        );

        /// Sets the height, in pixels, to which Metal constrains the render target.
        ///
        /// When this value is non-zero, you need to assign it to be smaller than or equal to the minimum height of all attachments.
        ///
        /// The default value of this property is `0`.
        #[unsafe(method(renderTargetHeight))]
        #[unsafe(method_family = none)]
        pub fn render_target_height(&self) -> usize;

        /// Setter for [`renderTargetHeight`][Self::renderTargetHeight].
        #[unsafe(method(setRenderTargetHeight:))]
        #[unsafe(method_family = none)]
        pub fn set_render_target_height(
            &self,
            render_target_height: usize,
        );

        /// Assigns an optional variable rasterization rate map that Metal uses in the render pass.
        ///
        /// Enabling variable rasterization rate allows Metal to decrease the rasterization rate, typically in unimportant
        /// regions of color attachments, to accelerate processing.
        ///
        /// When set to `nil`, the default, Metal doesn't use variable rasterization rate.
        #[unsafe(method(rasterizationRateMap))]
        #[unsafe(method_family = none)]
        pub fn rasterization_rate_map(&self) -> Option<Retained<ProtocolObject<dyn MTLRasterizationRateMap>>>;

        /// Setter for [`rasterizationRateMap`][Self::rasterizationRateMap].
        #[unsafe(method(setRasterizationRateMap:))]
        #[unsafe(method_family = none)]
        pub fn set_rasterization_rate_map(
            &self,
            rasterization_rate_map: Option<&ProtocolObject<dyn MTLRasterizationRateMap>>,
        );

        /// Configures a buffer into which Metal writes counts of fragments (pixels) passing the depth and stencil tests.
        #[unsafe(method(visibilityResultBuffer))]
        #[unsafe(method_family = none)]
        pub fn visibility_result_buffer(&self) -> Option<Retained<ProtocolObject<dyn MTLBuffer>>>;

        /// Setter for [`visibilityResultBuffer`][Self::visibilityResultBuffer].
        #[unsafe(method(setVisibilityResultBuffer:))]
        #[unsafe(method_family = none)]
        pub fn set_visibility_result_buffer(
            &self,
            visibility_result_buffer: Option<&ProtocolObject<dyn MTLBuffer>>,
        );

        /// Determines if Metal accumulates visibility results between render encoders or resets them.
        #[unsafe(method(visibilityResultType))]
        #[unsafe(method_family = none)]
        pub fn visibility_result_type(&self) -> MTLVisibilityResultType;

        /// Setter for [`visibilityResultType`][Self::visibilityResultType].
        #[unsafe(method(setVisibilityResultType:))]
        #[unsafe(method_family = none)]
        pub fn set_visibility_result_type(
            &self,
            visibility_result_type: MTLVisibilityResultType,
        );

        /// Controls if the render pass supports color attachment mapping.
        #[unsafe(method(supportColorAttachmentMapping))]
        #[unsafe(method_family = none)]
        pub fn support_color_attachment_mapping(&self) -> bool;

        /// Setter for [`supportColorAttachmentMapping`][Self::supportColorAttachmentMapping].
        #[unsafe(method(setSupportColorAttachmentMapping:))]
        #[unsafe(method_family = none)]
        pub fn set_support_color_attachment_mapping(
            &self,
            support_color_attachment_mapping: bool,
        );
    );
}

/// Methods declared on superclass `NSObject`.
impl MTL4RenderPassDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub fn new() -> Retained<Self>;
    );
}

impl MTL4RenderPassDescriptor {
    /// Configures the custom sample positions for MSAA rendering. Pass an empty slice to
    /// disable custom sample positions. The slice length must be a valid sample count.
    pub fn set_sample_positions(&self, positions: &[MTLSamplePosition]) {
        unsafe {
            let _: () =
                msg_send![self, setSamplePositions: positions.as_ptr(), count: positions.len()];
        }
    }

    /// Reads the previously-configured custom sample positions into `positions`. Metal only
    /// fills the slice if it is large enough to hold every position. Returns the total number
    /// of positions Metal currently has configured (which may exceed `positions.len()`).
    pub fn get_sample_positions(&self, positions: &mut [MTLSamplePosition]) -> usize {
        unsafe { msg_send![self, getSamplePositions: positions.as_mut_ptr(), count: positions.len()] }
    }
}
