use core::ptr::NonNull;

use objc2::{extern_protocol, runtime::ProtocolObject};

use crate::{
    MTLAccelerationStructure, MTLBuffer, MTLCommandEncoder, MTLCounterSampleBuffer, MTLDataType,
    MTLFence, MTLHeap, MTLResource, MTLResourceUsage,
};

extern_protocol!(
    /// Command encoder for building and managing acceleration structures.
    ///
    /// Availability: macOS 11.0+, iOS 14.0+, tvOS 16.0+
    ///
    /// The encoder records operations related to acceleration structures, such as
    /// building, refitting, copying, compacting, and resource/fence usage. Work
    /// encoded here executes when the enclosing `MTLCommandBuffer` is committed
    /// and finished by the GPU.
    pub unsafe trait MTLAccelerationStructureCommandEncoder: MTLCommandEncoder {
        /// Encode a build of an acceleration structure into the command buffer.
        ///
        /// All bottom-level builds must complete before a top-level build may
        /// begin. The resulting acceleration structure will not retain any
        /// references to input buffers (vertex, instance, etc.).
        ///
        /// The build is not complete until the command buffer has been
        /// committed and finished executing. It is safe to encode ray tracing
        /// work against the acceleration structure as long as command buffers
        /// are scheduled and synchronized such that this build has completed by
        /// the time ray tracing starts.
        ///
        /// The `acceleration_structure` and `scratch_buffer` must be at least
        /// the sizes returned by `MTLDevice::accelerationStructureSizesWithDescriptor`.
        ///
        /// - acceleration_structure: Storage object to build into.
        /// - descriptor: Describes the acceleration structure to build.
        /// - scratch_buffer: Scratch buffer used during the build. Contents
        ///   may be overwritten and are undefined after the build starts/completes.
        /// - scratch_buffer_offset: Byte offset into the scratch buffer.
        #[unsafe(method(buildAccelerationStructure:descriptor:scratchBuffer:scratchBufferOffset:))]
        #[unsafe(method_family = none)]
        fn build_acceleration_structure(
            &self,
            acceleration_structure: &ProtocolObject<dyn MTLAccelerationStructure>,
            descriptor: &crate::acceleration_structure::MTLAccelerationStructureDescriptor,
            scratch_buffer: &ProtocolObject<dyn MTLBuffer>,
            scratch_buffer_offset: usize,
        );

        /// Encode a refit of an acceleration structure (basic form).
        ///
        /// Refitting updates the acceleration structure when geometry changes
        /// and is typically much faster than rebuilding. Performance and quality
        /// may degrade depending on how much geometry changes. Refitting cannot
        /// be used for certain changes (e.g. adding/removing geometry).
        ///
        /// Refitting can occur in-place by specifying the same source and
        /// destination or by passing `None` for the destination. If source and
        /// destination are different, they must not overlap in memory.
        ///
        /// The destination must be at least as large as the source, unless the
        /// source has been compacted, in which case the destination must be at
        /// least the compacted size.
        ///
        /// The scratch buffer must be at least the size returned by
        /// `MTLDevice::accelerationStructureSizesWithDescriptor`.
        ///
        /// - source_acceleration_structure: Acceleration structure to refit from.
        /// - descriptor: Describes the acceleration structure to build/refit.
        /// - destination_acceleration_structure: Optional destination structure
        ///   to write into; `None` or same as source refits in-place.
        /// - scratch_buffer: Optional scratch buffer used during refit. Contents
        ///   may be overwritten and are undefined after refit starts/completes.
        /// - scratch_buffer_offset: Byte offset into the scratch buffer.
        #[unsafe(method(refitAccelerationStructure:descriptor:destination:scratchBuffer:scratchBufferOffset:))]
        #[unsafe(method_family = none)]
        unsafe fn refit_acceleration_structure(
            &self,
            source_acceleration_structure: &ProtocolObject<dyn MTLAccelerationStructure>,
            descriptor: &crate::acceleration_structure::MTLAccelerationStructureDescriptor,
            destination_acceleration_structure: Option<
                &ProtocolObject<dyn MTLAccelerationStructure>,
            >,
            scratch_buffer: Option<&ProtocolObject<dyn MTLBuffer>>,
            scratch_buffer_offset: usize,
        );

        /// Encode a refit of an acceleration structure with options.
        ///
        /// Same behavior and constraints as the basic refit method, but allows
        /// specifying `options` that control which elements of the acceleration
        /// structure are refit.
        ///
        /// Availability: macOS 13.0+, iOS 16.0+
        ///
        /// - source_acceleration_structure: Acceleration structure to refit from.
        /// - descriptor: Describes the acceleration structure to build/refit.
        /// - destination_acceleration_structure: Optional destination structure
        ///   to write into; `None` or same as source refits in-place.
        /// - scratch_buffer: Optional scratch buffer used during refit.
        /// - scratch_buffer_offset: Byte offset into the scratch buffer.
        /// - options: Refit options to apply.
        #[unsafe(method(refitAccelerationStructure:descriptor:destination:scratchBuffer:scratchBufferOffset:options:))]
        #[unsafe(method_family = none)]
        unsafe fn refit_acceleration_structure_with_options(
            &self,
            source_acceleration_structure: &ProtocolObject<dyn MTLAccelerationStructure>,
            descriptor: &crate::acceleration_structure::MTLAccelerationStructureDescriptor,
            destination_acceleration_structure: Option<
                &ProtocolObject<dyn MTLAccelerationStructure>,
            >,
            scratch_buffer: Option<&ProtocolObject<dyn MTLBuffer>>,
            scratch_buffer_offset: usize,
            options: crate::acceleration_structure::MTLAccelerationStructureRefitOptions,
        ); // Availability: API_AVAILABLE(macos(13.0), ios(16.0))

        /// Copy an acceleration structure from source to destination.
        ///
        /// Source and destination must not overlap in memory. For top-level
        /// acceleration structures, references to bottom-level structures are
        /// preserved.
        ///
        /// The destination must be at least as large as the source, unless the
        /// source has been compacted, in which case the destination must be at
        /// least the compacted size.
        #[unsafe(method(copyAccelerationStructure:toAccelerationStructure:))]
        #[unsafe(method_family = none)]
        unsafe fn copy_acceleration_structure(
            &self,
            source_acceleration_structure: &ProtocolObject<dyn MTLAccelerationStructure>,
            destination_acceleration_structure: &ProtocolObject<dyn MTLAccelerationStructure>,
        );

        /// Compute the compacted size for an acceleration structure and write
        /// it into a buffer as a 32-bit unsigned integer.
        ///
        /// Read this size after the command buffer completes, allocate a smaller
        /// acceleration structure accordingly, and then call
        /// [`copy_and_compact_acceleration_structure`].
        ///
        /// - acceleration_structure: Source acceleration structure.
        /// - buffer: Destination buffer to receive the 32-bit size in bytes.
        /// - offset: Byte offset into the destination buffer.
        #[unsafe(method(writeCompactedAccelerationStructureSize:toBuffer:offset:))]
        #[unsafe(method_family = none)]
        fn write_compacted_acceleration_structure_size(
            &self,
            acceleration_structure: &ProtocolObject<dyn MTLAccelerationStructure>,
            buffer: &ProtocolObject<dyn MTLBuffer>,
            offset: usize,
        );

        /// Compute the compacted size for an acceleration structure and write
        /// it into a buffer as either a 32-bit or 64-bit value depending on
        /// `size_data_type`.
        ///
        /// Availability: macOS 12.0+, iOS 15.0+, tvOS 16.0+
        ///
        /// - acceleration_structure: Source acceleration structure.
        /// - buffer: Destination buffer to receive the size in bytes.
        /// - offset: Byte offset into the destination buffer.
        /// - size_data_type: `MTLDataTypeUInt` (32-bit) or `MTLDataTypeULong` (64-bit).
        #[unsafe(method(writeCompactedAccelerationStructureSize:toBuffer:offset:sizeDataType:))]
        #[unsafe(method_family = none)]
        unsafe fn write_compacted_acceleration_structure_size_with_type(
            &self,
            acceleration_structure: &ProtocolObject<dyn MTLAccelerationStructure>,
            buffer: &ProtocolObject<dyn MTLBuffer>,
            offset: usize,
            size_data_type: MTLDataType,
        ); // Availability: API_AVAILABLE(macos(12.0), ios(15.0), tvos(16.0))

        /// Copy and compact an acceleration structure.
        ///
        /// Source and destination must not overlap in memory. For top-level
        /// acceleration structures, references to bottom-level structures are
        /// preserved.
        ///
        /// The destination must be at least as large as the compacted size of
        /// the source, as computed by
        /// [`write_compacted_acceleration_structure_size`] (or the typed
        /// variant).
        #[unsafe(method(copyAndCompactAccelerationStructure:toAccelerationStructure:))]
        #[unsafe(method_family = none)]
        fn copy_and_compact_acceleration_structure(
            &self,
            source_acceleration_structure: &ProtocolObject<dyn MTLAccelerationStructure>,
            destination_acceleration_structure: &ProtocolObject<dyn MTLAccelerationStructure>,
        );

        /// Update the fence to capture all GPU work so far enqueued by this encoder.
        ///
        /// The fence is updated at build submission to maintain global order and
        /// prevent deadlock. Drivers may delay fence updates until the end of
        /// the encoder and may also wait on fences at the beginning of an
        /// encoder. It is illegal to wait on a fence after it has been updated
        /// in the same encoder.
        #[unsafe(method(updateFence:))]
        #[unsafe(method_family = none)]
        fn update_fence(&self, fence: &ProtocolObject<dyn MTLFence>);

        /// Prevent further GPU work until the fence is reached.
        ///
        /// The fence is evaluated at build submission to maintain global order
        /// and prevent deadlock. Drivers may delay fence updates until the end
        /// of the encoder and may also wait on fences at the beginning of an
        /// encoder. It is illegal to wait on a fence after it has been updated
        /// in the same encoder.
        #[unsafe(method(waitForFence:))]
        #[unsafe(method_family = none)]
        fn wait_for_fence(&self, fence: &ProtocolObject<dyn MTLFence>);

        /// Declare that a resource may be accessed by the encoder through an
        /// argument buffer.
        ///
        /// For tracked `MTLResource`s, this protects against data hazards. Call
        /// before encoding any acceleration structure commands that may access
        /// the resource through an argument buffer.
        ///
        /// Warning: Prior to iOS 13 and macOS 10.15, this does not protect
        /// against data hazards. Use fences to ensure hazards are resolved on
        /// older OS versions.
        #[unsafe(method(useResource:usage:))]
        #[unsafe(method_family = none)]
        fn use_resource(&self, resource: &ProtocolObject<dyn MTLResource>, usage: MTLResourceUsage);

        /// Declare that an array of resources may be accessed by the encoder
        /// through an argument buffer.
        ///
        /// For tracked resources, this protects against data hazards. Call
        /// before encoding any acceleration structure commands that may access
        /// the resources through an argument buffer.
        ///
        /// Warning: Prior to iOS 13 and macOS 10.15, this does not protect
        /// against data hazards. Use fences to ensure hazards are resolved on
        /// older OS versions.
        ///
        /// Safety: `resources` must be a valid, non-null pointer to an array of
        /// non-null `ProtocolObject<dyn MTLResource>` pointers of length `count`.
        #[unsafe(method(useResources:count:usage:))]
        #[unsafe(method_family = none)]
        unsafe fn use_resources(
            &self,
            resources: NonNull<NonNull<ProtocolObject<dyn MTLResource>>>,
            count: usize,
            usage: MTLResourceUsage,
        );

        /// Declare that the resources allocated from a heap may be accessed as
        /// read-only by the encoder through an argument buffer.
        ///
        /// For tracked `MTLHeap`s, this protects against data hazards. Call
        /// before encoding any acceleration structure commands that may access
        /// resources allocated from the heap through an argument buffer. This
        /// may cause all color attachments allocated from the heap to become
        /// decompressed; prefer `use_resource`/`use_resources` for color
        /// attachments with minimal (read-only) usage.
        ///
        /// Warning: Prior to iOS 13 and macOS 10.15, this does not protect
        /// against data hazards. Use fences to ensure hazards are resolved on
        /// older OS versions.
        #[unsafe(method(useHeap:))]
        #[unsafe(method_family = none)]
        fn use_heap(&self, heap: &ProtocolObject<dyn MTLHeap>);

        /// Declare that the resources allocated from an array of heaps may be
        /// accessed as read-only by the encoder through an argument buffer.
        ///
        /// For tracked heaps, this protects against data hazards. Call before
        /// encoding commands that may access resources allocated from the heaps
        /// through an argument buffer. This may cause all color attachments
        /// allocated from the heaps to become decompressed; prefer
        /// `use_resource`/`use_resources` for color attachments with minimal
        /// (read-only) usage.
        ///
        /// Warning: Prior to iOS 13 and macOS 10.15, this does not protect
        /// against data hazards. Use fences to ensure hazards are resolved on
        /// older OS versions.
        ///
        /// Safety: `heaps` must be a valid, non-null pointer to an array of
        /// non-null `ProtocolObject<dyn MTLHeap>` pointers of length `count`.
        #[unsafe(method(useHeaps:count:))]
        #[unsafe(method_family = none)]
        unsafe fn use_heaps(
            &self,
            heaps: NonNull<NonNull<ProtocolObject<dyn MTLHeap>>>,
            count: usize,
        );

        /// Sample hardware counters at this point in the acceleration structure
        /// encoder and store the counter sample into the sample buffer at the
        /// specified index.
        ///
        /// Passing `barrier = true` inserts a barrier before taking the sample
        /// ensuring all work encoded before this operation in the encoder is
        /// complete, but it does not isolate the work with respect to other
        /// encoders. Passing `barrier = false` allows the sample to be taken
        /// concurrently with other operations in this encoder.
        ///
        /// In general, `true` yields more repeatable results but may reduce
        /// performance; `false` is higher performance but results may be less
        /// repeatable.
        ///
        /// Availability: macOS 11.0+, iOS 14.0+, tvOS 16.0+
        ///
        /// - sample_buffer: The counter sample buffer to sample into.
        /// - sample_index: Index into the counter buffer to write the sample.
        /// - barrier: Whether to insert a barrier before sampling.
        #[unsafe(method(sampleCountersInBuffer:atSampleIndex:withBarrier:))]
        #[unsafe(method_family = none)]
        unsafe fn sample_counters_in_buffer(
            &self,
            sample_buffer: &ProtocolObject<dyn MTLCounterSampleBuffer>,
            sample_index: usize,
            barrier: bool,
        ); // Availability: API_AVAILABLE(macos(11.0), ios(14.0), tvos(16.0))
    }
);
