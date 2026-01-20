use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
};
use objc2_foundation::{CopyingHelper, NSCopying, NSObject, NSObjectProtocol};

use crate::*;

extern_class!(
    /// Groups together properties you use to create a tile render pipeline state object.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metal/mtl4tilerenderpipelinedescriptor?language=objc)
    #[unsafe(super(MTL4PipelineDescriptor, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTL4TileRenderPipelineDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for MTL4TileRenderPipelineDescriptor {}
);

unsafe impl CopyingHelper for MTL4TileRenderPipelineDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTL4TileRenderPipelineDescriptor {}
);

impl MTL4TileRenderPipelineDescriptor {
    extern_methods!(
        /// Configures the tile function that the render pipeline executes for each tile in the tile shader stage.
        #[unsafe(method(tileFunctionDescriptor))]
        #[unsafe(method_family = none)]
        pub unsafe fn tile_function_descriptor(&self) -> Option<Retained<MTL4FunctionDescriptor>>;

        /// Setter for [`tileFunctionDescriptor`][Self::tileFunctionDescriptor].
        ///
        /// This is [copied][objc2_foundation::NSCopying::copy] when set.
        #[unsafe(method(setTileFunctionDescriptor:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_tile_function_descriptor(
            &self,
            tile_function_descriptor: Option<&MTL4FunctionDescriptor>,
        );

        /// Configures the number of samples per pixel used for multisampling.
        #[unsafe(method(rasterSampleCount))]
        #[unsafe(method_family = none)]
        pub unsafe fn raster_sample_count(&self) -> usize;

        /// Setter for [`rasterSampleCount`][Self::rasterSampleCount].
        #[unsafe(method(setRasterSampleCount:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_raster_sample_count(&self, raster_sample_count: usize);

        /// Access an array of descriptors that configure the properties of each color attachment in the tile render
        /// pipeline.
        #[unsafe(method(colorAttachments))]
        #[unsafe(method_family = none)]
        pub unsafe fn color_attachments(
            &self,
        ) -> Retained<MTLTileRenderPipelineColorAttachmentDescriptorArray>;

        /// Indicating whether the size of the threadgroup matches the size of a tile in the render pipeline.
        #[unsafe(method(threadgroupSizeMatchesTileSize))]
        #[unsafe(method_family = none)]
        pub unsafe fn threadgroup_size_matches_tile_size(&self) -> bool;

        /// Setter for [`threadgroupSizeMatchesTileSize`][Self::threadgroupSizeMatchesTileSize].
        #[unsafe(method(setThreadgroupSizeMatchesTileSize:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_threadgroup_size_matches_tile_size(
            &self,
            threadgroup_size_matches_tile_size: bool,
        );

        /// Sets the maximum number of threads that the GPU can execute simultaneously within a single threadgroup in
        /// the tile render pipeline.
        #[unsafe(method(maxTotalThreadsPerThreadgroup))]
        #[unsafe(method_family = none)]
        pub unsafe fn max_total_threads_per_threadgroup(&self) -> usize;

        /// Setter for [`maxTotalThreadsPerThreadgroup`][Self::maxTotalThreadsPerThreadgroup].
        #[unsafe(method(setMaxTotalThreadsPerThreadgroup:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_max_total_threads_per_threadgroup(
            &self,
            max_total_threads_per_threadgroup: usize,
        );

        /// Sets the required number of threads per threadgroup for tile dispatches.
        ///
        /// This value is typically optional, except in the cases where the tile function that ``tileFunctionDescriptor``
        /// references uses `CooperativeTensors`. In this case, you need to provide a non-zero value to this property.
        ///
        /// Additionally, when you set this value, the `threadsPerTile` argument of any tile dispatch needs to match it.
        ///
        /// Setting this value to a size of 0 in every dimension disables this property.
        #[unsafe(method(requiredThreadsPerThreadgroup))]
        #[unsafe(method_family = none)]
        pub unsafe fn required_threads_per_threadgroup(&self) -> MTLSize;

        /// Setter for [`requiredThreadsPerThreadgroup`][Self::requiredThreadsPerThreadgroup].
        #[unsafe(method(setRequiredThreadsPerThreadgroup:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_required_threads_per_threadgroup(
            &self,
            required_threads_per_threadgroup: MTLSize,
        );

        /// Configures an object that contains information about functions to link to the tile render pipeline
        /// when Metal builds it.
        #[unsafe(method(staticLinkingDescriptor))]
        #[unsafe(method_family = none)]
        pub unsafe fn static_linking_descriptor(&self) -> Retained<MTL4StaticLinkingDescriptor>;

        /// Setter for [`staticLinkingDescriptor`][Self::staticLinkingDescriptor].
        ///
        /// This is [copied][objc2_foundation::NSCopying::copy] when set.
        #[unsafe(method(setStaticLinkingDescriptor:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_static_linking_descriptor(
            &self,
            static_linking_descriptor: Option<&MTL4StaticLinkingDescriptor>,
        );

        /// Indicates whether the pipeline supports linking binary functions.
        #[unsafe(method(supportBinaryLinking))]
        #[unsafe(method_family = none)]
        pub unsafe fn support_binary_linking(&self) -> bool;

        /// Setter for [`supportBinaryLinking`][Self::supportBinaryLinking].
        #[unsafe(method(setSupportBinaryLinking:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_support_binary_linking(&self, support_binary_linking: bool);

        /// Resets the descriptor to the default state.
        #[unsafe(method(reset))]
        #[unsafe(method_family = none)]
        pub unsafe fn reset(&self);
    );
}

/// Methods declared on superclass `NSObject`.
impl MTL4TileRenderPipelineDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
