use objc2::{Message, msg_send, runtime::ProtocolObject};

use super::types::*;
use crate::util::ref_slice_as_ptr;
use crate::*;

pub trait MTL4CommandQueueExt: MTL4CommandQueue + Message {
    /// Enqueues an array of command buffers for execution.
    /// The order of command buffers in the slice is meaningful, especially for
    /// suspending/resuming render passes.
    fn commit(&self, command_buffers: &[&ProtocolObject<dyn MTL4CommandBuffer>])
    where
        Self: Sized,
    {
        let ptr = ref_slice_as_ptr(command_buffers);
        unsafe { msg_send![self, commit: ptr, count: command_buffers.len()] }
    }

    /// Enqueues an array of command buffers for execution with a set of options.
    /// Provide an `MTL4CommitOptions` instance to configure the commit operation.
    fn commit_with_options(
        &self,
        command_buffers: &[&ProtocolObject<dyn MTL4CommandBuffer>],
        options: &MTL4CommitOptions,
    ) where
        Self: Sized,
    {
        let ptr = ref_slice_as_ptr(command_buffers);
        unsafe { msg_send![self, commit: ptr, count: command_buffers.len(), options: options] }
    }

    /// Marks an array of residency sets as part of this command queue.
    /// Ensures that Metal makes them resident during execution of all command buffers committed
    /// to this queue. Each command queue supports up to 32 unique residency set instances.
    fn add_residency_sets(&self, residency_sets: &[&ProtocolObject<dyn MTLResidencySet>])
    where
        Self: Sized,
    {
        let ptr = ref_slice_as_ptr(residency_sets);
        unsafe { msg_send![self, addResidencySets: ptr, count: residency_sets.len()] }
    }

    /// Removes multiple residency sets from the command queue. After calling this method only
    /// the remaining residency sets remain resident during execution of committed command buffers.
    fn remove_residency_sets(&self, residency_sets: &[&ProtocolObject<dyn MTLResidencySet>])
    where
        Self: Sized,
    {
        let ptr = ref_slice_as_ptr(residency_sets);
        unsafe { msg_send![self, removeResidencySets: ptr, count: residency_sets.len()] }
    }

    /// Updates multiple regions within a placement sparse texture to alias specific tiles
    /// of a Metal heap. Pass `None` for `heap` only when performing unmap operations.
    fn update_texture_mappings(
        &self,
        texture: &ProtocolObject<dyn MTLTexture>,
        heap: Option<&ProtocolObject<dyn MTLHeap>>,
        operations: &[MTL4UpdateSparseTextureMappingOperation],
    ) where
        Self: Sized,
    {
        unsafe {
            msg_send![
                self,
                updateTextureMappings: texture,
                heap: heap,
                operations: operations.as_ptr(),
                count: operations.len()
            ]
        }
    }

    /// Copies multiple regions within a source placement sparse texture to a destination
    /// placement sparse texture. Both textures must have the same `placementSparsePageSize`.
    fn copy_texture_mappings(
        &self,
        source: &ProtocolObject<dyn MTLTexture>,
        destination: &ProtocolObject<dyn MTLTexture>,
        operations: &[MTL4CopySparseTextureMappingOperation],
    ) where
        Self: Sized,
    {
        unsafe {
            msg_send![
                self,
                copyTextureMappingsFromTexture: source,
                toTexture: destination,
                operations: operations.as_ptr(),
                count: operations.len()
            ]
        }
    }

    /// Updates multiple regions within a placement sparse buffer to alias specific tiles
    /// from a Metal heap. Pass `None` for `heap` only when performing unmap operations.
    fn update_buffer_mappings(
        &self,
        buffer: &ProtocolObject<dyn MTLBuffer>,
        heap: Option<&ProtocolObject<dyn MTLHeap>>,
        operations: &[MTL4UpdateSparseBufferMappingOperation],
    ) where
        Self: Sized,
    {
        unsafe {
            msg_send![
                self,
                updateBufferMappings: buffer,
                heap: heap,
                operations: operations.as_ptr(),
                count: operations.len()
            ]
        }
    }

    /// Copies multiple offsets within a source placement sparse buffer to a destination
    /// placement sparse buffer. Both buffers must have the same `placementSparsePageSize`.
    fn copy_buffer_mappings(
        &self,
        source: &ProtocolObject<dyn MTLBuffer>,
        destination: &ProtocolObject<dyn MTLBuffer>,
        operations: &[MTL4CopySparseBufferMappingOperation],
    ) where
        Self: Sized,
    {
        unsafe {
            msg_send![
                self,
                copyBufferMappingsFromBuffer: source,
                toBuffer: destination,
                operations: operations.as_ptr(),
                count: operations.len()
            ]
        }
    }
}

impl<T: MTL4CommandQueue + Message> MTL4CommandQueueExt for T {}
