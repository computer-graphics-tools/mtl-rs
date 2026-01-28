use objc2::extern_protocol;
use objc2_foundation::NSObjectProtocol;

extern_protocol!(
    /// Base class for Metal allocations.
    ///
    /// This protocol provides a common interface for adding Metal resources to residency sets.
    /// Call `MTLResidencySet::addAllocation` to add a Metal resource allocation to a residency set.
    ///
    /// # Thread Safety
    /// Metal resource objects (MTLBuffer, MTLTexture) are thread-safe and can be used from multiple threads.
    /// See Apple's Metal Programming Guide for synchronization requirements when modifying contents.
    pub unsafe trait MTLAllocation: NSObjectProtocol + Send + Sync {
        /// The size, in bytes, occupied by this allocation.
        #[unsafe(method(allocatedSize))]
        #[unsafe(method_family = none)]
        fn allocated_size(&self) -> usize;
    }
);
