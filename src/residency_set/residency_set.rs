use objc2::{Message, extern_protocol, msg_send, rc::Retained, runtime::ProtocolObject};
use objc2_foundation::{NSArray, NSObjectProtocol, NSString};

use crate::util::ref_ptr_cast_const;
use crate::{MTLAllocation, MTLDevice};

extern_protocol!(
    /// A residency set is responsible for managing resource and heap residency and is referenced
    /// by a command buffer or command queue in order to ensure that resources and heaps are resident.
    /// Resources and heaps are added and removed uncommitted and a subsequent commit call applies all
    /// of the changes in bulk.
    pub unsafe trait MTLResidencySet: NSObjectProtocol + Send + Sync {
        /// The device that created the residency set.
        #[unsafe(method(device))]
        #[unsafe(method_family = none)]
        fn device(&self) -> Retained<ProtocolObject<dyn MTLDevice>>;

        /// The memory footprint of the set in bytes at the last commit operation.
        /// This may include internal allocations as well.
        #[unsafe(method(allocatedSize))]
        #[unsafe(method_family = none)]
        fn allocated_size(&self) -> u64;

        /// Requests that the set and all the committed resources and heaps are made resident.
        #[unsafe(method(requestResidency))]
        #[unsafe(method_family = none)]
        fn request_residency(&self);

        /// Requests that the set and all the committed resources and heaps are made non-resident.
        #[unsafe(method(endResidency))]
        #[unsafe(method_family = none)]
        fn end_residency(&self);

        /// Adds one allocation to the set, leaving it uncommitted until commit is called.
        #[unsafe(method(addAllocation:))]
        #[unsafe(method_family = none)]
        fn add_allocation(
            &self,
            allocation: &ProtocolObject<dyn MTLAllocation>,
        );

        /// Marks an allocation to be removed from the set on the next commit call.
        #[unsafe(method(removeAllocation:))]
        #[unsafe(method_family = none)]
        fn remove_allocation(
            &self,
            allocation: &ProtocolObject<dyn MTLAllocation>,
        );

        /// Marks all allocations to be removed from the set on the next commit call.
        #[unsafe(method(removeAllAllocations))]
        #[unsafe(method_family = none)]
        fn remove_all_allocations(&self);

        /// Returns whether the allocation is present in the set or not.
        /// This check includes non-committed allocations in the set.
        #[unsafe(method(containsAllocation:))]
        #[unsafe(method_family = none)]
        fn contains_allocation(
            &self,
            allocation: &ProtocolObject<dyn MTLAllocation>,
        ) -> bool;

        /// The current number of unique allocations present in the set.
        /// This property includes non-committed allocations in the set.
        #[unsafe(method(allocationCount))]
        #[unsafe(method_family = none)]
        fn allocation_count(&self) -> usize;

        /// Commits any pending adds/removes.
        /// If the residency set is resident, this will try to make added resources and heaps
        /// resident instantly, and make removed resources and heaps non-resident.
        #[unsafe(method(commit))]
        #[unsafe(method_family = none)]
        fn commit(&self);
    }
);

#[allow(unused)]
pub trait MTLResidencySetExt: MTLResidencySet + Message {
    /// The label specified at creation.
    fn label(&self) -> Option<String>;

    /// Array of all allocations associated with the set, including non-committed ones.
    fn all_allocations(&self) -> Box<[Retained<ProtocolObject<dyn MTLAllocation>>]>;

    /// Adds allocations to the set, leaving them uncommitted until commit is called.
    fn add_allocations(&self, allocations: &[&ProtocolObject<dyn MTLAllocation>])
    where
        Self: Sized,
    {
        let ptr = ref_ptr_cast_const(allocations.as_ptr());
        unsafe { msg_send![self, addAllocations: ptr, count: allocations.len()] }
    }

    /// Marks allocations to be removed from the set on the next commit call.
    fn remove_allocations(&self, allocations: &[&ProtocolObject<dyn MTLAllocation>])
    where
        Self: Sized,
    {
        let ptr = ref_ptr_cast_const(allocations.as_ptr());
        unsafe { msg_send![self, removeAllocations: ptr, count: allocations.len()] }
    }
}

impl MTLResidencySetExt for ProtocolObject<dyn MTLResidencySet> {
    fn label(&self) -> Option<String> {
        let s: Option<Retained<NSString>> = unsafe { msg_send![self, label] };
        s.map(|s| s.to_string())
    }

    fn all_allocations(&self) -> Box<[Retained<ProtocolObject<dyn MTLAllocation>>]> {
        let arr: Retained<NSArray<ProtocolObject<dyn MTLAllocation>>> = unsafe { msg_send![self, allAllocations] };
        arr.to_vec().into_boxed_slice()
    }
}
