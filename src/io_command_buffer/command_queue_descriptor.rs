use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
    runtime::{NSObject, ProtocolObject},
};
use objc2_foundation::{CopyingHelper, NSCopying, NSObjectProtocol};

use crate::{MTLIOCommandQueueType, MTLIOPriority, MTLIOScratchBufferAllocator};

extern_class!(
    /// Descriptor for creating an IO command queue.
    ///
    /// Availability: macOS 13.0+, iOS 16.0+
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLIOCommandQueueDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for MTLIOCommandQueueDescriptor {}
);

unsafe impl CopyingHelper for MTLIOCommandQueueDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLIOCommandQueueDescriptor {}
);

impl MTLIOCommandQueueDescriptor {
    extern_methods!(
        /// Maximum number of command buffers in flight.
        #[unsafe(method(maxCommandBufferCount))]
        #[unsafe(method_family = none)]
        pub fn max_command_buffer_count(&self) -> usize;

        /// Setter for [`max_command_buffer_count`][Self::max_command_buffer_count].
        #[unsafe(method(setMaxCommandBufferCount:))]
        #[unsafe(method_family = none)]
        pub fn set_max_command_buffer_count(&self, count: usize);

        /// Priority of this queue.
        #[unsafe(method(priority))]
        #[unsafe(method_family = none)]
        pub fn priority(&self) -> MTLIOPriority;

        /// Setter for [`priority`][Self::priority].
        #[unsafe(method(setPriority:))]
        #[unsafe(method_family = none)]
        pub fn set_priority(&self, priority: MTLIOPriority);

        /// Type (serial or concurrent) of queue.
        #[unsafe(method(r#type))]
        #[unsafe(method_family = none)]
        pub fn queue_type(&self) -> MTLIOCommandQueueType;

        /// Setter for [`queue_type`][Self::queue_type].
        #[unsafe(method(setType:))]
        #[unsafe(method_family = none)]
        pub fn set_queue_type(&self, t: MTLIOCommandQueueType);

        /// Maximum number of IO commands in flight.
        #[unsafe(method(maxCommandsInFlight))]
        #[unsafe(method_family = none)]
        pub fn max_commands_in_flight(&self) -> usize;

        /// Setter for [`max_commands_in_flight`][Self::max_commands_in_flight].
        #[unsafe(method(setMaxCommandsInFlight:))]
        #[unsafe(method_family = none)]
        pub fn set_max_commands_in_flight(&self, count: usize);

        /// Optional custom allocator for scratch buffers used by the queue.
        #[unsafe(method(scratchBufferAllocator))]
        #[unsafe(method_family = none)]
        pub fn scratch_buffer_allocator(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn MTLIOScratchBufferAllocator>>>;

        /// Setter for [`scratch_buffer_allocator`][Self::scratch_buffer_allocator].
        #[unsafe(method(setScratchBufferAllocator:))]
        #[unsafe(method_family = none)]
        pub fn set_scratch_buffer_allocator(
            &self,
            allocator: Option<&ProtocolObject<dyn MTLIOScratchBufferAllocator>>,
        );
    );
}

/// Methods declared on superclass `NSObject`.
impl MTLIOCommandQueueDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub fn new() -> Retained<Self>;
    );
}
