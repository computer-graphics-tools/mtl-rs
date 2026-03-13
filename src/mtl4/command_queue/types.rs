use objc2::encode::{Encode, Encoding, RefEncode};
use objc2_foundation::{NSErrorDomain, NSInteger, NSRange, NSUInteger};

use crate::*;

/// Enumeration of kinds of errors that committing an array of command buffers instances can produce.
///
/// See also [Apple's documentation](https://developer.apple.com/documentation/metal/mtl4commandqueueerror?language=objc)
/// NS_ENUM
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTL4CommandQueueError(pub NSInteger);
impl MTL4CommandQueueError {
    /// Indicates the absence of any problems.
    #[doc(alias = "MTL4CommandQueueErrorNone")]
    pub const NONE: Self = Self(0);
    /// Indicates the workload takes longer to execute than the system allows.
    #[doc(alias = "MTL4CommandQueueErrorTimeout")]
    pub const TIMEOUT: Self = Self(1);
    /// Indicates a process doesn’t have access to a GPU device.
    #[doc(alias = "MTL4CommandQueueErrorNotPermitted")]
    pub const NOT_PERMITTED: Self = Self(2);
    /// Indicates the GPU doesn’t have sufficient memory to execute a command buffer.
    #[doc(alias = "MTL4CommandQueueErrorOutOfMemory")]
    pub const OUT_OF_MEMORY: Self = Self(3);
    /// Indicates the physical removal of the GPU before the command buffer completed.
    #[doc(alias = "MTL4CommandQueueErrorDeviceRemoved")]
    pub const DEVICE_REMOVED: Self = Self(4);
    /// Indicates that the system revokes GPU access because it’s responsible for too many timeouts or hangs.
    #[doc(alias = "MTL4CommandQueueErrorAccessRevoked")]
    pub const ACCESS_REVOKED: Self = Self(5);
    /// Indicates an internal problem in the Metal framework.
    #[doc(alias = "MTL4CommandQueueErrorInternal")]
    pub const INTERNAL: Self = Self(6);
}

unsafe impl Encode for MTL4CommandQueueError {
    const ENCODING: Encoding = NSInteger::ENCODING;
}

unsafe impl RefEncode for MTL4CommandQueueError {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

unsafe extern "C" {
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtl4commandqueueerrordomain?language=objc)
    pub static MTL4CommandQueueErrorDomain: &'static NSErrorDomain;
}

/// Groups together arguments for an operation to update a sparse texture mapping.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MTL4UpdateSparseTextureMappingOperation {
    pub mode: MTLSparseTextureMappingMode,
    pub texture_region: MTLRegion,
    pub texture_level: NSUInteger,
    pub texture_slice: NSUInteger,
    pub heap_offset: NSUInteger,
}

unsafe impl Encode for MTL4UpdateSparseTextureMappingOperation {
    const ENCODING: Encoding = Encoding::Struct(
        "?",
        &[
            <MTLSparseTextureMappingMode>::ENCODING,
            <MTLRegion>::ENCODING,
            <NSUInteger>::ENCODING,
            <NSUInteger>::ENCODING,
            <NSUInteger>::ENCODING,
        ],
    );
}

unsafe impl RefEncode for MTL4UpdateSparseTextureMappingOperation {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// Groups together arguments for an operation to copy a sparse texture mapping.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MTL4CopySparseTextureMappingOperation {
    pub source_region: MTLRegion,
    pub source_level: NSUInteger,
    pub source_slice: NSUInteger,
    pub destination_origin: MTLOrigin,
    pub destination_level: NSUInteger,
    pub destination_slice: NSUInteger,
}

unsafe impl Encode for MTL4CopySparseTextureMappingOperation {
    const ENCODING: Encoding = Encoding::Struct(
        "?",
        &[
            <MTLRegion>::ENCODING,
            <NSUInteger>::ENCODING,
            <NSUInteger>::ENCODING,
            <MTLOrigin>::ENCODING,
            <NSUInteger>::ENCODING,
            <NSUInteger>::ENCODING,
        ],
    );
}

unsafe impl RefEncode for MTL4CopySparseTextureMappingOperation {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// Groups together arguments for an operation to update a sparse buffer mapping.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MTL4UpdateSparseBufferMappingOperation {
    pub mode: MTLSparseTextureMappingMode,
    pub buffer_range: NSRange,
    pub heap_offset: NSUInteger,
}

unsafe impl Encode for MTL4UpdateSparseBufferMappingOperation {
    const ENCODING: Encoding = Encoding::Struct(
        "?",
        &[
            <MTLSparseTextureMappingMode>::ENCODING,
            <NSRange>::ENCODING,
            <NSUInteger>::ENCODING,
        ],
    );
}

unsafe impl RefEncode for MTL4UpdateSparseBufferMappingOperation {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// Groups together arguments for an operation to copy a sparse buffer mapping.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MTL4CopySparseBufferMappingOperation {
    pub source_range: NSRange,
    pub destination_offset: NSUInteger,
}

unsafe impl Encode for MTL4CopySparseBufferMappingOperation {
    const ENCODING: Encoding =
        Encoding::Struct("?", &[<NSRange>::ENCODING, <NSUInteger>::ENCODING]);
}

unsafe impl RefEncode for MTL4CopySparseBufferMappingOperation {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
