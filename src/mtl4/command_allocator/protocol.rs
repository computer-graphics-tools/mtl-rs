use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2::{Message, extern_protocol, msg_send};
use objc2_foundation::{NSObjectProtocol, NSString};

use crate::*;

extern_protocol!(
    /// Manages the memory backing the encoding of GPU commands into command buffers.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metal/mtl4commandallocator?language=objc)
    pub unsafe trait MTL4CommandAllocator: NSObjectProtocol {
        /// Returns the GPU device that this command allocator belongs to.
        #[unsafe(method(device))]
        #[unsafe(method_family = none)]
        fn device(&self) -> Retained<ProtocolObject<dyn MTLDevice>>;

        /// Queries the size of the internal memory heaps of this command allocator that support encoding
        /// commands into command buffers.
        ///
        /// - Returns: a size in bytes.
        #[unsafe(method(allocatedSize))]
        #[unsafe(method_family = none)]
        fn allocated_size(&self) -> u64;

        /// Marks the command allocator's heaps for reuse.
        #[unsafe(method(reset))]
        #[unsafe(method_family = none)]
        fn reset(&self);
    }
);

pub trait MTL4CommandAllocatorExt: MTL4CommandAllocator + Message {
    /// Provides the optional label you specify at creation time for debug purposes.
    fn label(&self) -> Option<String> {
        let s: Option<Retained<NSString>> = unsafe { msg_send![self, label] };
        s.map(|s| s.to_string())
    }
}

impl<T: MTL4CommandAllocator + Message> MTL4CommandAllocatorExt for T {}
