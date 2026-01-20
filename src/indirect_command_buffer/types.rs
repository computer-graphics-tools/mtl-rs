use objc2::{Encode, Encoding, RefEncode};

/// Commands that may be performed indirectly (from `MTLIndirectCommandType`).
///
/// Availability: macOS 10.14+, iOS 12.0+
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLIndirectCommandType(pub usize);
bitflags::bitflags! {
    impl MTLIndirectCommandType: usize {
        /// Availability: macOS 10.14+, iOS 12.0+
        const Draw = 1<<0;
        /// Availability: macOS 10.14+, iOS 12.0+
        const DrawIndexed = 1<<1;
        /// Availability: tvOS 14.5+
        const DrawPatches = 1<<2;
        /// Availability: tvOS 14.5+
        const DrawIndexedPatches = 1<<3;
        /// Dispatch threadgroups with concurrent execution.
        /// Availability: macOS 11.0+, iOS 13.0+
        const ConcurrentDispatch = 1<<5;
        /// Dispatch threads with concurrent execution.
        /// Availability: macOS 11.0+, iOS 13.0+
        const ConcurrentDispatchThreads = 1<<6;
        /// Availability: macOS 14.0+, iOS 17.0+
        const DrawMeshThreadgroups = 1<<7;
        /// Availability: macOS 14.0+, iOS 17.0+
        const DrawMeshThreads = 1<<8;
    }
}

unsafe impl Encode for MTLIndirectCommandType {
    const ENCODING: Encoding = usize::ENCODING;
}

unsafe impl RefEncode for MTLIndirectCommandType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// Execution range for indirect command buffer (from `MTLIndirectCommandBufferExecutionRange`).
///
/// Availability: macOS 10.14+, Mac Catalyst 13.0+, iOS 13.0+
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MTLIndirectCommandBufferExecutionRange {
    pub location: u32,
    pub length: u32,
}

unsafe impl Encode for MTLIndirectCommandBufferExecutionRange {
    const ENCODING: Encoding = Encoding::Struct(
        "{MTLIndirectCommandBufferExecutionRange=II}",
        &[u32::ENCODING, u32::ENCODING],
    );
}

unsafe impl RefEncode for MTLIndirectCommandBufferExecutionRange {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
