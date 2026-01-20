use objc2::{extern_protocol, rc::Retained, runtime::ProtocolObject};
use objc2_foundation::NSObjectProtocol;

use super::MTLIOScratchBuffer;

extern_protocol!(
    /// Custom allocator for scratch buffers used by IO queues.
    ///
    /// Availability: macOS 13.0+, iOS 16.0+
    pub unsafe trait MTLIOScratchBufferAllocator: NSObjectProtocol {
        /// Called when additional scratch memory is required by a load command.
        #[unsafe(method(newScratchBufferWithMinimumSize:))]
        #[unsafe(method_family = new)]
        unsafe fn new_scratch_buffer_with_minimum_size(
            &self,
            minimum_size: usize,
        ) -> Option<Retained<ProtocolObject<dyn MTLIOScratchBuffer>>>;
    }
);
