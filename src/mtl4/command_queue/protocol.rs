use core::ptr::NonNull;

use objc2::{extern_protocol, rc::Retained, runtime::ProtocolObject};
use objc2_foundation::{NSObjectProtocol, NSString, NSUInteger};

use super::types::*;
use crate::*;

extern_protocol!(
    /// An abstraction representing a command queue that you use to commit and synchronize command
    /// buffers and to perform other GPU operations.
    pub unsafe trait MTL4CommandQueue: NSObjectProtocol + Send + Sync {
        /// Returns the GPU device that the command queue belongs to.
        #[unsafe(method(device))]
        #[unsafe(method_family = none)]
        fn device(&self) -> Retained<ProtocolObject<dyn MTLDevice>>;

        /// Obtains this queue's optional label for debugging purposes.
        #[unsafe(method(label))]
        #[unsafe(method_family = none)]
        fn label(&self) -> Option<Retained<NSString>>;

        /// Schedules an operation to signal a GPU event with a specific value after all GPU work
        /// prior to this point is complete.
        #[unsafe(method(signalEvent:value:))]
        #[unsafe(method_family = none)]
        fn signal_event_value(
            &self,
            event: &ProtocolObject<dyn MTLEvent>,
            value: u64,
        );

        /// Schedules an operation to wait for a GPU event of a specific value before continuing
        /// to execute any future GPU work.
        #[unsafe(method(waitForEvent:value:))]
        #[unsafe(method_family = none)]
        fn wait_for_event_value(
            &self,
            event: &ProtocolObject<dyn MTLEvent>,
            value: u64,
        );

        /// Schedules a signal operation on the command queue to indicate when rendering to a
        /// Metal drawable is complete. Signaling indicates it's safe to present.
        #[unsafe(method(signalDrawable:))]
        #[unsafe(method_family = none)]
        fn signal_drawable(
            &self,
            drawable: &ProtocolObject<dyn MTLDrawable>,
        );

        /// Schedules a wait operation on the command queue to ensure the display is no longer
        /// using a specific Metal drawable before executing subsequent commands.
        #[unsafe(method(waitForDrawable:))]
        #[unsafe(method_family = none)]
        fn wait_for_drawable(
            &self,
            drawable: &ProtocolObject<dyn MTLDrawable>,
        );

        /// Marks a residency set as part of this command queue. Ensures that Metal makes the
        /// residency set resident during execution of all command buffers committed to this queue.
        /// Each command queue supports up to 32 unique residency set instances.
        #[unsafe(method(addResidencySet:))]
        #[unsafe(method_family = none)]
        fn add_residency_set(
            &self,
            residency_set: &ProtocolObject<dyn MTLResidencySet>,
        );

        /// Removes a residency set from the command queue. After calling this method only the
        /// remaining residency sets remain resident during execution of committed command buffers.
        #[unsafe(method(removeResidencySet:))]
        #[unsafe(method_family = none)]
        fn remove_residency_set(
            &self,
            residency_set: &ProtocolObject<dyn MTLResidencySet>,
        );

        /// Updates multiple regions within a placement sparse texture to alias specific tiles
        /// of a Metal heap. Pass `None` for `heap` only when performing unmap operations.
        ///
        /// # Safety
        ///
        /// `operations` must be a valid pointer to an array of length `count`.
        #[unsafe(method(updateTextureMappings:heap:operations:count:))]
        #[unsafe(method_family = none)]
        fn update_texture_mappings_heap_operations_count(
            &self,
            texture: &ProtocolObject<dyn MTLTexture>,
            heap: Option<&ProtocolObject<dyn MTLHeap>>,
            operations: NonNull<MTL4UpdateSparseTextureMappingOperation>,
            count: NSUInteger,
        );

        /// Copies multiple regions within a source placement sparse texture to a destination
        /// placement sparse texture. Both textures must have the same `placementSparsePageSize`.
        ///
        /// # Safety
        ///
        /// `operations` must be a valid pointer to an array of length `count`.
        #[unsafe(method(copyTextureMappingsFromTexture:toTexture:operations:count:))]
        #[unsafe(method_family = none)]
        fn copy_texture_mappings_from_texture_to_texture_operations_count(
            &self,
            source_texture: &ProtocolObject<dyn MTLTexture>,
            destination_texture: &ProtocolObject<dyn MTLTexture>,
            operations: NonNull<MTL4CopySparseTextureMappingOperation>,
            count: NSUInteger,
        );

        /// Updates multiple regions within a placement sparse buffer to alias specific tiles
        /// from a Metal heap. Pass `None` for `heap` only when performing unmap operations.
        ///
        /// # Safety
        ///
        /// `operations` must be a valid pointer to an array of length `count`.
        #[unsafe(method(updateBufferMappings:heap:operations:count:))]
        #[unsafe(method_family = none)]
        fn update_buffer_mappings_heap_operations_count(
            &self,
            buffer: &ProtocolObject<dyn MTLBuffer>,
            heap: Option<&ProtocolObject<dyn MTLHeap>>,
            operations: NonNull<MTL4UpdateSparseBufferMappingOperation>,
            count: NSUInteger,
        );

        /// Copies multiple offsets within a source placement sparse buffer to a destination
        /// placement sparse buffer. Both buffers must have the same `placementSparsePageSize`.
        ///
        /// # Safety
        ///
        /// `operations` must be a valid pointer to an array of length `count`.
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
