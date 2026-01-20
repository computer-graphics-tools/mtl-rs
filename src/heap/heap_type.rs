use objc2::{Encode, Encoding, RefEncode};

/// Describes the mode of operation for a Metal heap (from `MTLHeapType`).
///
/// Availability: macOS 10.15+, iOS 13.0+
#[repr(i64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum MTLHeapType {
    /// In this mode, resources are placed in the heap automatically.
    /// Automatically placed resources have optimal GPU-specific layout, and may perform better than MTLHeapTypePlacement.
    /// This heap type is recommended when the heap primarily contains temporary write-often resources.
    Automatic = 0,
    /// In this mode, the app places resources in the heap.
    /// Manually placed resources allow the app to control memory usage and heap fragmentation directly.
    /// This heap type is recommended when the heap primarily contains persistent write-rarely resources.
    Placement = 1,
    /// Sparse heaps for placement sparse resources.
    Sparse = 2,
}

unsafe impl Encode for MTLHeapType {
    const ENCODING: Encoding = i64::ENCODING;
}

unsafe impl RefEncode for MTLHeapType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
