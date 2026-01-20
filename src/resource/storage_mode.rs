use objc2::{Encode, Encoding, RefEncode};

/// Describes location and CPU mapping of a resource (from `MTLStorageMode`).
#[repr(u64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum MTLStorageMode {
    /// In this mode, CPU and device will nominally both use the same underlying memory
    /// when accessing the contents of the resource. However, coherency is only guaranteed
    /// at command buffer boundaries to minimize the required flushing of CPU and GPU caches.
    /// This is the default storage mode for iOS textures.
    Shared = 0,

    /// This mode relaxes the coherency requirements and requires that the developer make explicit
    /// requests to maintain coherency between a CPU and GPU version of the resource.
    /// In order for CPU to access up-to-date GPU results, a blit synchronization must be completed
    /// (see synchronize methods of MTLBlitCommandEncoder). Blit overhead is only incurred if GPU
    /// has modified the resource. This is the default storage mode for macOS textures.
    Managed = 1,

    /// This mode allows the resource data to be kept entirely to GPU (or driver) private memory
    /// that will never be accessed by the CPU directly, so no coherency of any kind must be maintained.
    Private = 2,

    /// This mode allows creation of resources that do not have a GPU or CPU memory backing, but do have
    /// on-chip storage for TBDR devices. The contents of the on-chip storage is undefined and does not
    /// persist, but its configuration is controlled by the texture/buffer descriptor. Resources created
    /// with Memoryless storage don't have an IOAccelResource at any point in their lifetime. The only way
    /// to populate such a resource is to perform rendering operations on it. Blit operations are disallowed.
    Memoryless = 3,
}

unsafe impl Encode for MTLStorageMode {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for MTLStorageMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
