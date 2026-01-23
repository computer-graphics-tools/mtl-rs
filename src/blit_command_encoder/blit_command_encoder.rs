use objc2::{extern_protocol, runtime::ProtocolObject};
use objc2_foundation::NSRange;

use super::MTLBlitOption;
use crate::{
    MTLBuffer, MTLCommandEncoder, MTLCounterSampleBuffer, MTLFence, MTLIndirectCommandBuffer,
    MTLResource, MTLTensor, MTLTexture,
    types::{MTLOrigin, MTLRegion, MTLSize},
};

extern_protocol!(
    /// A command encoder that performs basic copies and blits between buffers and textures.
    pub unsafe trait MTLBlitCommandEncoder: MTLCommandEncoder {
        /// Flush any copy of this resource from the device's caches, and invalidate any CPU caches if needed.
        ///
        /// When the device writes to a resource with a storage mode of MTLResourceStorageModeManaged, those writes may be cached (for example, in VRAM or on chip renderer cache),
        /// making any CPU access (either MTLBuffer.contents or -[MTLTexture getBytes:...] and -[MTLTexture replaceRegion:]) produce undefined results.  To allow the CPU to see what the device
        /// has written, a CommandBuffer containing this synchronization must be executed.  After completion of the CommandBuffer, the CPU can access the contents of the resource safely.
        #[unsafe(method(synchronizeResource:))]
        #[unsafe(method_family = none)]
        fn synchronize_resource(&self, resource: &ProtocolObject<dyn MTLResource>);

        /// Flush any copy of this image from the device's caches, and invalidate CPU caches if needed.
        ///
        /// See the discussion of -synchronizeResource.   -synchronizeTexture:slice:mipmapLevel performs the same role, except it may flush only a subset of the texture storage, rather than the entire texture.
        #[unsafe(method(synchronizeTexture:slice:level:))]
        #[unsafe(method_family = none)]
        fn synchronize_texture_slice_level(
            &self,
            texture: &ProtocolObject<dyn MTLTexture>,
            slice: usize,
            level: usize,
        );

        /// Copy a rectangle of pixels between textures.
        #[unsafe(method(copyFromTexture:sourceSlice:sourceLevel:sourceOrigin:sourceSize:toTexture:destinationSlice:destinationLevel:destinationOrigin:))]
        #[unsafe(method_family = none)]
        fn copy_from_texture_to_texture(
            &self,
            source_texture: &ProtocolObject<dyn MTLTexture>,
            source_slice: usize,
            source_level: usize,
            source_origin: MTLOrigin,
            source_size: MTLSize,
            destination_texture: &ProtocolObject<dyn MTLTexture>,
            destination_slice: usize,
            destination_level: usize,
            destination_origin: MTLOrigin,
        );

        /// Copy an image from a buffer into a texture.
        #[unsafe(method(copyFromBuffer:sourceOffset:sourceBytesPerRow:sourceBytesPerImage:sourceSize:toTexture:destinationSlice:destinationLevel:destinationOrigin:))]
        #[unsafe(method_family = none)]
        fn copy_from_buffer_to_texture(
            &self,
            source_buffer: &ProtocolObject<dyn MTLBuffer>,
            source_offset: usize,
            source_bytes_per_row: usize,
            source_bytes_per_image: usize,
            source_size: MTLSize,
            destination_texture: &ProtocolObject<dyn MTLTexture>,
            destination_slice: usize,
            destination_level: usize,
            destination_origin: MTLOrigin,
        );

        /// Copy an image from a buffer into a texture.
        #[unsafe(method(copyFromBuffer:sourceOffset:sourceBytesPerRow:sourceBytesPerImage:sourceSize:toTexture:destinationSlice:destinationLevel:destinationOrigin:options:))]
        #[unsafe(method_family = none)]
        fn copy_from_buffer_to_texture_with_options(
            &self,
            source_buffer: &ProtocolObject<dyn MTLBuffer>,
            source_offset: usize,
            source_bytes_per_row: usize,
            source_bytes_per_image: usize,
            source_size: MTLSize,
            destination_texture: &ProtocolObject<dyn MTLTexture>,
            destination_slice: usize,
            destination_level: usize,
            destination_origin: MTLOrigin,
            options: MTLBlitOption,
        );

        /// Copy an image from a texture into a buffer.
        #[unsafe(method(copyFromTexture:sourceSlice:sourceLevel:sourceOrigin:sourceSize:toBuffer:destinationOffset:destinationBytesPerRow:destinationBytesPerImage:))]
        #[unsafe(method_family = none)]
        fn copy_from_texture_to_buffer(
            &self,
            source_texture: &ProtocolObject<dyn MTLTexture>,
            source_slice: usize,
            source_level: usize,
            source_origin: MTLOrigin,
            source_size: MTLSize,
            destination_buffer: &ProtocolObject<dyn MTLBuffer>,
            destination_offset: usize,
            destination_bytes_per_row: usize,
            destination_bytes_per_image: usize,
        );

        /// Copy an image from a texture into a buffer.
        #[unsafe(method(copyFromTexture:sourceSlice:sourceLevel:sourceOrigin:sourceSize:toBuffer:destinationOffset:destinationBytesPerRow:destinationBytesPerImage:options:))]
        #[unsafe(method_family = none)]
        fn copy_from_texture_to_buffer_with_options(
            &self,
            source_texture: &ProtocolObject<dyn MTLTexture>,
            source_slice: usize,
            source_level: usize,
            source_origin: MTLOrigin,
            source_size: MTLSize,
            destination_buffer: &ProtocolObject<dyn MTLBuffer>,
            destination_offset: usize,
            destination_bytes_per_row: usize,
            destination_bytes_per_image: usize,
            options: MTLBlitOption,
        );

        /// Generate mipmaps for a texture from the base level up to the max level.
        #[unsafe(method(generateMipmapsForTexture:))]
        #[unsafe(method_family = none)]
        fn generate_mipmaps_for_texture(&self, texture: &ProtocolObject<dyn MTLTexture>);

        /// Fill a buffer with a fixed value in each byte.
        #[unsafe(method(fillBuffer:range:value:))]
        #[unsafe(method_family = none)]
        fn fill_buffer_range_value(
            &self,
            buffer: &ProtocolObject<dyn MTLBuffer>,
            range: NSRange,
            value: u8,
        );

        /// Copy whole surfaces between textures.
        /// Convenience function to copy sliceCount * levelCount whole surfaces between textures
        /// The source and destination pixel format must be identical.
        /// The source and destination sample count must be identical.
        /// The sourceLevel mip in sourceTexture must have the same dimension as the destinationLevel mip in destinationTexture.
        /// The sourceTexture must have at least sourceLevel + levelCount mips
        /// The destinationTexture must have at least destinationLevel + levelCount mips
        /// The sourceTexture must have at least sourceSlice + sliceCount array slices
        /// The destinationTexture must have at least destinationSlice + sliceCount array slices
        #[unsafe(method(copyFromTexture:sourceSlice:sourceLevel:toTexture:destinationSlice:destinationLevel:sliceCount:levelCount:))]
        #[unsafe(method_family = none)]
        fn copy_surfaces_between_textures(
            &self,
            source_texture: &ProtocolObject<dyn MTLTexture>,
            source_slice: usize,
            source_level: usize,
            destination_texture: &ProtocolObject<dyn MTLTexture>,
            destination_slice: usize,
            destination_level: usize,
            slice_count: usize,
            level_count: usize,
        );

        /// Copy as many whole surfaces as possible between textures.
        /// Convenience function that calls copyFromTexture:sourceSlice:sourceLevel:toTexture:destinationSlice:destinationLevel:sliceCount:levelCount:
        /// The source and destination pixel format must be identical.
        /// The source and destination sample count must be identical.
        /// Either:
        /// - sourceTexture must have a mip M with identical dimensions as the first mip of destinationTexture: sourceLevel = M, destinationLevel = 0
        /// - destinationTexture must have a mip M with identical dimensions as the first mip of sourceTexture: sourceLevel = 0, destinationLevel = M
        /// Computes: levelCount = min(sourceTexture.mipmapLevelCount - sourceLevel, destinationTexture.mipmapLevelCount - destinationLevel)
        ///           sliceCount = min(sourceTexture.arrayLength, destinationTexture.arrayLength)
        /// Then invokes the method above using the computed parameters.
        #[unsafe(method(copyFromTexture:toTexture:))]
        #[unsafe(method_family = none)]
        fn copy_from_texture_to_texture_simple(
            &self,
            source_texture: &ProtocolObject<dyn MTLTexture>,
            destination_texture: &ProtocolObject<dyn MTLTexture>,
        );

        /// Basic memory copy between buffers.
        #[unsafe(method(copyFromBuffer:sourceOffset:toBuffer:destinationOffset:size:))]
        #[unsafe(method_family = none)]
        fn copy_buffer_to_buffer(
            &self,
            source_buffer: &ProtocolObject<dyn MTLBuffer>,
            source_offset: usize,
            destination_buffer: &ProtocolObject<dyn MTLBuffer>,
            destination_offset: usize,
            size: usize,
        );

        /// Update the fence to capture all GPU work so far enqueued by this encoder.
        /// The fence is updated at kernel submission to maintain global order and prevent deadlock.
        /// Drivers may delay fence updates until the end of the encoder. Drivers may also wait on fences at the beginning of an encoder. It is therefore illegal to wait on a fence after it has been updated in the same encoder.
        #[unsafe(method(updateFence:))]
        #[unsafe(method_family = none)]
        fn update_fence(&self, fence: &ProtocolObject<dyn MTLFence>);

        /// Prevent further GPU work until the fence is reached.
        /// The fence is evaluated at kernel submission to maintain global order and prevent deadlock.
        /// Drivers may delay fence updates until the end of the encoder. Drivers may also wait on fences at the beginning of an encoder. It is therefore illegal to wait on a fence after it has been updated in the same encoder.
        #[unsafe(method(waitForFence:))]
        #[unsafe(method_family = none)]
        fn wait_for_fence(&self, fence: &ProtocolObject<dyn MTLFence>);

        /// Copies tile access counters within specified region into provided buffer
        #[optional]
        #[unsafe(method(getTextureAccessCounters:region:mipLevel:slice:resetCounters:countersBuffer:countersBufferOffset:))]
        #[unsafe(method_family = none)]
        fn get_texture_access_counters(
            &self,
            texture: &ProtocolObject<dyn MTLTexture>,
            region: MTLRegion,
            mip_level: usize,
            slice: usize,
            reset_counters: bool,
            counters_buffer: &ProtocolObject<dyn MTLBuffer>,
            counters_buffer_offset: usize,
        );

        /// Resets tile access counters within specified region
        #[optional]
        #[unsafe(method(resetTextureAccessCounters:region:mipLevel:slice:))]
        #[unsafe(method_family = none)]
        fn reset_texture_access_counters(
            &self,
            texture: &ProtocolObject<dyn MTLTexture>,
            region: MTLRegion,
            mip_level: usize,
            slice: usize,
        );

        /// Optimizes the texture data to ensure the best possible performance when accessing content on the GPU at the expense of CPU-access performance.
        #[unsafe(method(optimizeContentsForGPUAccess:))]
        #[unsafe(method_family = none)]
        fn optimize_contents_for_gpu_access(&self, texture: &ProtocolObject<dyn MTLTexture>);

        /// Optimizes a subset of the texture data to ensure the best possible performance when accessing content on the GPU at the expense of CPU-access performance.
        #[unsafe(method(optimizeContentsForGPUAccess:slice:level:))]
        #[unsafe(method_family = none)]
        fn optimize_contents_for_gpu_access_slice_level(
            &self,
            texture: &ProtocolObject<dyn MTLTexture>,
            slice: usize,
            level: usize,
        );

        /// Optimizes the texture data to ensure the best possible performance when accessing content on the CPU at the expense of GPU-access performance.
        #[unsafe(method(optimizeContentsForCPUAccess:))]
        #[unsafe(method_family = none)]
        fn optimize_contents_for_cpu_access(&self, texture: &ProtocolObject<dyn MTLTexture>);

        /// Optimizes a subset of the texture data to ensure the best possible performance when accessing content on the CPU at the expense of GPU-access performance.
        #[unsafe(method(optimizeContentsForCPUAccess:slice:level:))]
        #[unsafe(method_family = none)]
        fn optimize_contents_for_cpu_access_slice_level(
            &self,
            texture: &ProtocolObject<dyn MTLTexture>,
            slice: usize,
            level: usize,
        );

        /// reset commands in a indirect command buffer using the GPU
        #[unsafe(method(resetCommandsInBuffer:withRange:))]
        #[unsafe(method_family = none)]
        fn reset_commands_in_buffer(
            &self,
            buffer: &ProtocolObject<dyn MTLIndirectCommandBuffer>,
            range: NSRange,
        );

        /// copy a region of a buffer into a destination buffer starting at destinationIndex using the GPU
        #[unsafe(method(copyIndirectCommandBuffer:sourceRange:destination:destinationIndex:))]
        #[unsafe(method_family = none)]
        fn copy_indirect_command_buffer(
            &self,
            source: &ProtocolObject<dyn MTLIndirectCommandBuffer>,
            source_range: NSRange,
            destination: &ProtocolObject<dyn MTLIndirectCommandBuffer>,
            destination_index: usize,
        );

        /// Encodes a command that can improve the performance of a range of commands within an indirect command buffer.
        #[unsafe(method(optimizeIndirectCommandBuffer:withRange:))]
        #[unsafe(method_family = none)]
        fn optimize_indirect_command_buffer(
            &self,
            indirect_command_buffer: &ProtocolObject<dyn MTLIndirectCommandBuffer>,
            range: NSRange,
        );

        /// Sample hardware counters at this point in the blit encoder and
        /// store the counter sample into the sample buffer at the specified index.
        /// @param sampleBuffer The sample buffer to sample into
        /// @param sampleIndex The index into the counter buffer to write the sample.
        /// @param barrier Insert a barrier before taking the sample.  Passing
        /// YES will ensure that all work encoded before this operation in the encoder is
        /// complete but does not isolate the work with respect to other encoders.  Passing
        /// NO will allow the sample to be taken concurrently with other operations in this
        /// encoder.
        /// In general, passing YES will lead to more repeatable counter results but
        /// may negatively impact performance.  Passing NO will generally be higher performance
        /// but counter results may not be repeatable.
        /// On devices where MTLCounterSamplingPointAtBlitBoundary is unsupported,
        /// this method is not available and will generate an error if called.
        #[unsafe(method(sampleCountersInBuffer:atSampleIndex:withBarrier:))]
        #[unsafe(method_family = none)]
        fn sample_counters_in_buffer(
            &self,
            sample_buffer: &ProtocolObject<dyn MTLCounterSampleBuffer>,
            sample_index: usize,
            barrier: bool,
        );

        /// @param sampleBuffer The sample buffer to resolve.
        /// @param range The range of indices to resolve.
        /// @param destinationBuffer The buffer to resolve values into.
        /// @param destinationOffset The offset to begin writing values out to.  This must be a multiple of
        /// the minimum constant buffer alignment.
        /// @abstract Resolve the counters from the raw buffer to a processed buffer.
        /// @discussion Samples that encountered an error during resolve will be set to
        /// MTLCounterErrorValue.
        #[unsafe(method(resolveCounters:inRange:destinationBuffer:destinationOffset:))]
        #[unsafe(method_family = none)]
        fn resolve_counters(
            &self,
            sample_buffer: &ProtocolObject<dyn MTLCounterSampleBuffer>,
            range: NSRange,
            destination_buffer: &ProtocolObject<dyn MTLBuffer>,
            destination_offset: usize,
        );

        /// Encodes a command to copy data from a slice of one tensor into a slice of another tensor.
        ///
        /// This command applies reshapes if `sourceTensor` and `destinationTensor` are not aliasable.
        /// - Parameters:
        ///    - sourceTensor: A tensor instance that this command copies data from.
        ///    - sourceOrigin: An array of offsets, in elements, to the first element of the slice of `sourceTensor` that this command copies data from.
        ///    - sourceDimensions: An array of sizes, in elements, of the slice `sourceTensor` that this command copies data from.
        ///    - destinationTensor: A tensor instance that this command copies data to.
        ///    - destinationOrigin: An array of offsets, in elements, to the first element of the slice of `destinationTensor` that this command copies data to.
        ///    - destinationDimensions: An array of sizes, in elements, of the slice of `destinationTensor` that this command copies data to.
        #[unsafe(method(copyFromTensor:sourceOrigin:sourceDimensions:toTensor:destinationOrigin:destinationDimensions:))]
        #[unsafe(method_family = none)]
        fn copy_between_tensors(
            &self,
            source_tensor: &ProtocolObject<dyn MTLTensor>,
            source_origin: &crate::tensor::MTLTensorExtents,
            source_dimensions: &crate::tensor::MTLTensorExtents,
            destination_tensor: &ProtocolObject<dyn MTLTensor>,
            destination_origin: &crate::tensor::MTLTensorExtents,
            destination_dimensions: &crate::tensor::MTLTensorExtents,
        );
    }
);
