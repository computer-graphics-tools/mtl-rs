use objc2::{Message, msg_send, runtime::ProtocolObject};

use crate::util::ref_ptr_cast_const;
use crate::*;

pub trait MTL4CommandQueueExt: MTL4CommandQueue + Message {
    /// Enqueues an array of command buffers for execution.
    /// The order of command buffers in the slice is meaningful, especially for
    /// suspending/resuming render passes.
    fn commit(&self, command_buffers: &[&ProtocolObject<dyn MTL4CommandBuffer>])
    where
        Self: Sized,
    {
        let ptr = ref_ptr_cast_const(command_buffers.as_ptr());
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
        let ptr = ref_ptr_cast_const(command_buffers.as_ptr());
        unsafe { msg_send![self, commit: ptr, count: command_buffers.len(), options: options] }
    }

    /// Marks an array of residency sets as part of this command queue.
    /// Ensures that Metal makes them resident during execution of all command buffers committed
    /// to this queue. Each command queue supports up to 32 unique residency set instances.
    fn add_residency_sets(&self, residency_sets: &[&ProtocolObject<dyn MTLResidencySet>])
    where
        Self: Sized,
    {
        let ptr = ref_ptr_cast_const(residency_sets.as_ptr());
        unsafe { msg_send![self, addResidencySets: ptr, count: residency_sets.len()] }
    }

    /// Removes multiple residency sets from the command queue. After calling this method only
    /// the remaining residency sets remain resident during execution of committed command buffers.
    fn remove_residency_sets(&self, residency_sets: &[&ProtocolObject<dyn MTLResidencySet>])
    where
        Self: Sized,
    {
        let ptr = ref_ptr_cast_const(residency_sets.as_ptr());
        unsafe { msg_send![self, removeResidencySets: ptr, count: residency_sets.len()] }
    }
}

impl<T: MTL4CommandQueue + Message> MTL4CommandQueueExt for T {}
