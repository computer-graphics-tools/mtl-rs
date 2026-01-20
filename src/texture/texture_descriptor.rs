use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
    runtime::NSObject,
};
use objc2_foundation::{CopyingHelper, NSCopying, NSObjectProtocol};

use crate::{
    MTLCPUCacheMode, MTLHazardTrackingMode, MTLPixelFormat, MTLResourceOptions, MTLSparsePageSize,
    MTLStorageMode, MTLTextureCompressionType, MTLTextureSwizzleChannels, MTLTextureType,
    MTLTextureUsage,
};

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtltexturedescriptor?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLTextureDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for MTLTextureDescriptor {}
);

unsafe impl CopyingHelper for MTLTextureDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLTextureDescriptor {}
);

impl MTLTextureDescriptor {
    extern_methods!(
        /// Create a TextureDescriptor for a common 2D texture.
        #[unsafe(method(texture2DDescriptorWithPixelFormat:width:height:mipmapped:))]
        #[unsafe(method_family = none)]
        pub unsafe fn texture_2d_descriptor_with_pixel_format_width_height_mipmapped(
            pixel_format: MTLPixelFormat,
            width: usize,
            height: usize,
            mipmapped: bool,
        ) -> Retained<MTLTextureDescriptor>;

        /// Create a TextureDescriptor for a common Cube texture.
        #[unsafe(method(textureCubeDescriptorWithPixelFormat:size:mipmapped:))]
        #[unsafe(method_family = none)]
        pub unsafe fn texture_cube_descriptor_with_pixel_format_size_mipmapped(
            pixel_format: MTLPixelFormat,
            size: usize,
            mipmapped: bool,
        ) -> Retained<MTLTextureDescriptor>;

        /// Create a TextureDescriptor for a common texture buffer.
        #[unsafe(method(textureBufferDescriptorWithPixelFormat:width:resourceOptions:usage:))]
        #[unsafe(method_family = none)]
        pub unsafe fn texture_buffer_descriptor_with_pixel_format_width_resource_options_usage(
            pixel_format: MTLPixelFormat,
            width: usize,
            resource_options: MTLResourceOptions,
            usage: MTLTextureUsage,
        ) -> Retained<MTLTextureDescriptor>;

        /// The overall type of the texture to be created. The default value is MTLTextureType2D.
        #[unsafe(method(textureType))]
        #[unsafe(method_family = none)]
        pub fn texture_type(&self) -> MTLTextureType;

        /// Setter for [`textureType`][Self::textureType].
        #[unsafe(method(setTextureType:))]
        #[unsafe(method_family = none)]
        pub fn set_texture_type(&self, texture_type: MTLTextureType);

        /// The pixel format to use when allocating this texture. This is also the pixel format that will be used to when the caller writes or reads pixels from this texture. The default value is MTLPixelFormatRGBA8Unorm.
        #[unsafe(method(pixelFormat))]
        #[unsafe(method_family = none)]
        pub fn pixel_format(&self) -> MTLPixelFormat;

        /// Setter for [`pixelFormat`][Self::pixelFormat].
        #[unsafe(method(setPixelFormat:))]
        #[unsafe(method_family = none)]
        pub fn set_pixel_format(&self, pixel_format: MTLPixelFormat);

        /// The width of the texture to create. The default value is 1.
        #[unsafe(method(width))]
        #[unsafe(method_family = none)]
        pub fn width(&self) -> usize;

        /// Setter for [`width`][Self::width].
        #[unsafe(method(setWidth:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_width(&self, width: usize);

        /// The height of the texture to create. The default value is 1.
        ///
        /// height If allocating a 1D texture, height must be 1.
        #[unsafe(method(height))]
        #[unsafe(method_family = none)]
        pub fn height(&self) -> usize;

        /// Setter for [`height`][Self::height].
        #[unsafe(method(setHeight:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_height(&self, height: usize);

        /// The depth of the texture to create. The default value is 1.
        ///
        /// depth When allocating any texture types other than 3D, depth must be 1.
        #[unsafe(method(depth))]
        #[unsafe(method_family = none)]
        pub fn depth(&self) -> usize;

        /// Setter for [`depth`][Self::depth].
        #[unsafe(method(setDepth:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_depth(&self, depth: usize);

        /// The number of mipmap levels to allocate. The default value is 1.
        ///
        /// When creating Buffer and Multisample textures, mipmapLevelCount must be 1.
        #[unsafe(method(mipmapLevelCount))]
        #[unsafe(method_family = none)]
        pub fn mipmap_level_count(&self) -> usize;

        /// Setter for [`mipmapLevelCount`][Self::mipmapLevelCount].
        #[unsafe(method(setMipmapLevelCount:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_mipmap_level_count(&self, mipmap_level_count: usize);

        /// The number of samples in the texture to create. The default value is 1.
        ///
        /// When creating Buffer textures sampleCount must be 1. Implementations may round sample counts up to the next supported value.
        #[unsafe(method(sampleCount))]
        #[unsafe(method_family = none)]
        pub fn sample_count(&self) -> usize;

        /// Setter for [`sampleCount`][Self::sampleCount].
        #[unsafe(method(setSampleCount:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_sample_count(&self, sample_count: usize);

        /// The number of array elements to allocate. The default value is 1.
        ///
        /// When allocating any non-Array texture type, arrayLength has to be 1. Otherwise it must be set to something greater than 1 and less than 2048.
        #[unsafe(method(arrayLength))]
        #[unsafe(method_family = none)]
        pub fn array_length(&self) -> usize;

        /// Setter for [`arrayLength`][Self::arrayLength].
        #[unsafe(method(setArrayLength:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_array_length(&self, array_length: usize);

        /// Options to control memory allocation parameters, etc.
        ///
        /// Contains a packed set of the storageMode, cpuCacheMode and hazardTrackingMode properties.
        #[unsafe(method(resourceOptions))]
        #[unsafe(method_family = none)]
        pub fn resource_options(&self) -> MTLResourceOptions;

        /// Setter for [`resourceOptions`][Self::resourceOptions].
        #[unsafe(method(setResourceOptions:))]
        #[unsafe(method_family = none)]
        pub fn set_resource_options(&self, resource_options: MTLResourceOptions);

        /// Options to specify CPU cache mode of texture resource.
        #[unsafe(method(cpuCacheMode))]
        #[unsafe(method_family = none)]
        pub fn cpu_cache_mode(&self) -> MTLCPUCacheMode;

        /// Setter for [`cpuCacheMode`][Self::cpuCacheMode].
        #[unsafe(method(setCpuCacheMode:))]
        #[unsafe(method_family = none)]
        pub fn set_cpu_cache_mode(&self, cpu_cache_mode: MTLCPUCacheMode);

        /// To specify storage mode of texture resource.
        #[unsafe(method(storageMode))]
        #[unsafe(method_family = none)]
        pub fn storage_mode(&self) -> MTLStorageMode;

        /// Setter for [`storageMode`][Self::storageMode].
        #[unsafe(method(setStorageMode:))]
        #[unsafe(method_family = none)]
        pub fn set_storage_mode(&self, storage_mode: MTLStorageMode);

        /// Set hazard tracking mode for the texture. The default value is `HazardTrackingMode::Default`.
        ///
        /// For resources created from the device, `HazardTrackingMode::Default` is treated as `HazardTrackingMode::Tracked`.
        /// For resources created on a heap, `HazardTrackingMode::Default` is treated as the hazardTrackingMode of the heap itself.
        /// In either case, it is possible to opt-out of hazard tracking by setting `HazardTrackingMode::Untracked`.
        /// It is not possible to opt-in to hazard tracking on a heap that itself is not hazard tracked.
        /// For optimal performance, perform hazard tracking manually through `Fence` or `Event` instead.
        #[unsafe(method(hazardTrackingMode))]
        #[unsafe(method_family = none)]
        pub fn hazard_tracking_mode(&self) -> MTLHazardTrackingMode;

        /// Setter for [`hazardTrackingMode`][Self::hazardTrackingMode].
        #[unsafe(method(setHazardTrackingMode:))]
        #[unsafe(method_family = none)]
        pub fn set_hazard_tracking_mode(&self, hazard_tracking_mode: MTLHazardTrackingMode);

        /// Description of texture usage
        #[unsafe(method(usage))]
        #[unsafe(method_family = none)]
        pub fn usage(&self) -> MTLTextureUsage;

        /// Setter for [`usage`][Self::usage].
        #[unsafe(method(setUsage:))]
        #[unsafe(method_family = none)]
        pub fn set_usage(&self, usage: MTLTextureUsage);

        /// Allow GPU-optimization for the contents of this texture. The default value is true.
        ///
        /// Useful for opting-out of GPU-optimization when implicit optimization (e.g. RT writes) is regressing CPU-read-back performance. See the documentation for optimizeContentsForGPUAccess: and optimizeContentsForCPUAccess: APIs.
        #[unsafe(method(allowGPUOptimizedContents))]
        #[unsafe(method_family = none)]
        pub fn allow_gpu_optimized_contents(&self) -> bool;

        /// Setter for [`allowGPUOptimizedContents`][Self::allowGPUOptimizedContents].
        #[unsafe(method(setAllowGPUOptimizedContents:))]
        #[unsafe(method_family = none)]
        pub fn set_allow_gpu_optimized_contents(&self, allow_gpu_optimized_contents: bool);

        /// Controls how the texture contents will be compressed when written to by the GPU. Compression can be used to reduce the bandwidth usage and storage requirements of a texture.
        ///
        /// The default compression type is lossless, meaning that no loss of precision will occur when the texture content is modified.
        /// Losslessly compressed textures may benefit from reduced bandwidth usage when regions of correlated color values are written, but do not benefit from reduced storage requirements.
        /// Enabling lossy compression for textures that can tolerate some precision loss will guarantee both reduced bandwidth usage and reduced storage requirements.
        /// The amount of precision loss depends on the color values stored; regions with correlated color values can be represented with limited to no precision loss, whereas regions with unrelated color values suffer more precision loss.
        /// Enabling lossy compression requires both storageMode == `StorageMode::Private`, allowGPUOptimizedContents == `YES`, and cannot be combined with either `TextureUsage::PixelFormatView`, `TextureUsage::ShaderWrite`, `TextureUsage::ShaderAtomic`, `TextureType::Type1D(Array)` or `TextureType::TextureBuffer`.
        /// Moreover, not all `PixelFormat` are supported with lossy compression, verify that the `Device`'s GPU family supports the lossy compression feature for the pixelFormat requested.
        /// Set allowGPUOptimizedContents to `NO` to opt out of both lossless and lossy compression; such textures do not benefit from either reduced bandwidth usage or reduced storage requirements, but have predictable CPU readback performance.
        #[unsafe(method(compressionType))]
        #[unsafe(method_family = none)]
        pub unsafe fn compression_type(&self) -> MTLTextureCompressionType;

        /// Setter for [`compressionType`][Self::compressionType].
        #[unsafe(method(setCompressionType:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_compression_type(&self, compression_type: MTLTextureCompressionType);

        /// Channel swizzle to use when reading or sampling from the texture, the default value is MTLTextureSwizzleChannelsDefault.
        #[unsafe(method(swizzle))]
        #[unsafe(method_family = none)]
        pub fn swizzle(&self) -> MTLTextureSwizzleChannels;

        /// Setter for [`swizzle`][Self::swizzle].
        #[unsafe(method(setSwizzle:))]
        #[unsafe(method_family = none)]
        pub fn set_swizzle(&self, swizzle: MTLTextureSwizzleChannels);

        /// Determines the page size for a placement sparse texture.
        ///
        /// Set this property to a non-zero value to create a placement sparse texture.
        /// Placement sparse textures are instances of `Texture` that you assign memory to using a `Heap`
        /// of type `HeapType::Placement` and a `HeapDescriptor::maxCompatiblePlacementSparsePageSize`
        /// at least as large as the `SparsePageSize` value you assign to this property.
        /// This value is 0 by default.
        #[unsafe(method(placementSparsePageSize))]
        #[unsafe(method_family = none)]
        pub unsafe fn placement_sparse_page_size(&self) -> MTLSparsePageSize;

        /// Setter for [`placementSparsePageSize`][Self::placementSparsePageSize].
        #[unsafe(method(setPlacementSparsePageSize:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_placement_sparse_page_size(
            &self,
            placement_sparse_page_size: MTLSparsePageSize,
        );
    );
}

/// Methods declared on superclass `NSObject`.
impl MTLTextureDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
