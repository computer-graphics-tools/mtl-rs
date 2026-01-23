use core::ptr::NonNull;

use objc2::{extern_protocol, rc::Retained, runtime::ProtocolObject};
use objc2_foundation::{NSObjectProtocol, NSString, NSUInteger};

use super::types::*;
use crate::*;

extern_protocol!(
    /// An abstraction representing a command queue that you use commit and synchronize command buffers and to
    /// perform other GPU operations.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metal/mtl4commandqueue?language=objc)
    pub unsafe trait MTL4CommandQueue: NSObjectProtocol + Send + Sync {
        /// Returns the GPU device that the command queue belongs to.
        #[unsafe(method(device))]
        #[unsafe(method_family = none)]
        fn device(&self) -> Retained<ProtocolObject<dyn MTLDevice>>;

        /// Obtains this queue's optional label for debugging purposes.
        #[unsafe(method(label))]
        #[unsafe(method_family = none)]
        fn label(&self) -> Option<Retained<NSString>>;

        /// Enqueues an array of command buffers for execution.
        ///
        /// # Safety
        ///
        /// `command_buffers` must be a valid pointer.
        #[unsafe(method(commit:count:))]
        #[unsafe(method_family = none)]
        fn commit_count(
            &self,
            command_buffers: NonNull<NonNull<ProtocolObject<dyn MTL4CommandBuffer>>>,
            count: NSUInteger,
        );

        /// Enqueues an array of command buffer instances for execution with a set of options.
        ///
        /// # Safety
        ///
        /// `command_buffers` must be a valid pointer.
        #[unsafe(method(commit:count:options:))]
        #[unsafe(method_family = none)]
        fn commit_count_options(
            &self,
            command_buffers: NonNull<NonNull<ProtocolObject<dyn MTL4CommandBuffer>>>,
            count: NSUInteger,
            options: &MTL4CommitOptions,
        );

        /// Schedules an operation to signal a GPU event with a specific value.
        #[unsafe(method(signalEvent:value:))]
        #[unsafe(method_family = none)]
        fn signal_event_value(&self, event: &ProtocolObject<dyn MTLEvent>, value: u64);

        /// Schedules an operation to wait for a GPU event of a specific value.
        #[unsafe(method(waitForEvent:value:))]
        #[unsafe(method_family = none)]
        fn wait_for_event_value(&self, event: &ProtocolObject<dyn MTLEvent>, value: u64);

        /// Schedules a signal operation for a drawable.
        #[unsafe(method(signalDrawable:))]
        #[unsafe(method_family = none)]
        fn signal_drawable(&self, drawable: &ProtocolObject<dyn MTLDrawable>);

        /// Schedules a wait operation for a drawable.
        #[unsafe(method(waitForDrawable:))]
        #[unsafe(method_family = none)]
        fn wait_for_drawable(&self, drawable: &ProtocolObject<dyn MTLDrawable>);

        /// Marks a residency set as part of this command queue.
        #[unsafe(method(addResidencySet:))]
        #[unsafe(method_family = none)]
        fn add_residency_set(&self, residency_set: &ProtocolObject<dyn MTLResidencySet>);

        /// Marks an array of residency sets as part of this command queue.
        ///
        /// # Safety
        #[unsafe(method(addResidencySets:count:))]
        #[unsafe(method_family = none)]
        fn add_residency_sets_count(
            &self,
            residency_sets: NonNull<NonNull<ProtocolObject<dyn MTLResidencySet>>>,
            count: NSUInteger,
        );

        /// Removes a residency set from the command queue.
        #[unsafe(method(removeResidencySet:))]
        #[unsafe(method_family = none)]
        fn remove_residency_set(&self, residency_set: &ProtocolObject<dyn MTLResidencySet>);

        /// Removes multiple residency sets from the command queue.
        ///
        /// # Safety
        #[unsafe(method(removeResidencySets:count:))]
        #[unsafe(method_family = none)]
        fn remove_residency_sets_count(
            &self,
            residency_sets: NonNull<NonNull<ProtocolObject<dyn MTLResidencySet>>>,
            count: NSUInteger,
        );

        /// Updates multiple regions within a placement sparse texture to alias specific tiles of a Metal heap.
        ///
        /// # Safety
        #[unsafe(method(updateTextureMappings:heap:operations:count:))]
        #[unsafe(method_family = none)]
        fn update_texture_mappings_heap_operations_count(
            &self,
            texture: &ProtocolObject<dyn MTLTexture>,
            heap: Option<&ProtocolObject<dyn MTLHeap>>,
            operations: NonNull<MTL4UpdateSparseTextureMappingOperation>,
            count: NSUInteger,
        );

        /// Copies multiple regions within a source placement sparse texture to a destination placement sparse texture.
        ///
        /// # Safety
        #[unsafe(method(copyTextureMappingsFromTexture:toTexture:operations:count:))]
        #[unsafe(method_family = none)]
        fn copy_texture_mappings_from_texture_to_texture_operations_count(
            &self,
            source_texture: &ProtocolObject<dyn MTLTexture>,
            destination_texture: &ProtocolObject<dyn MTLTexture>,
            operations: NonNull<MTL4CopySparseTextureMappingOperation>,
            count: NSUInteger,
        );

        /// Updates multiple regions within a placement sparse buffer to alias specific tiles from a Metal heap.
        ///
        /// # Safety
        #[unsafe(method(updateBufferMappings:heap:operations:count:))]
        #[unsafe(method_family = none)]
        fn update_buffer_mappings_heap_operations_count(
            &self,
            buffer: &ProtocolObject<dyn MTLBuffer>,
            heap: Option<&ProtocolObject<dyn MTLHeap>>,
            operations: NonNull<MTL4UpdateSparseBufferMappingOperation>,
            count: NSUInteger,
        );

        /// Copies multiple offsets within a source placement sparse buffer to a destination placement sparse buffer.
        ///
        /// # Safety
        #[unsafe(method(copyBufferMappingsFromBuffer:toBuffer:operations:count:))]
        #[unsafe(method_family = none)]
        fn copy_buffer_mappings_from_buffer_to_buffer_operations_count(
            &self,
            source_buffer: &ProtocolObject<dyn MTLBuffer>,
            destination_buffer: &ProtocolObject<dyn MTLBuffer>,
            operations: NonNull<MTL4CopySparseBufferMappingOperation>,
            count: NSUInteger,
        );
    }
);
