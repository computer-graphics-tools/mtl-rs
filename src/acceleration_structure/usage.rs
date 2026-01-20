use objc2::{Encode, Encoding, RefEncode};

/// Usage flags for an acceleration structure.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLAccelerationStructureUsage(pub u64);

bitflags::bitflags! {
    impl MTLAccelerationStructureUsage: u64 {
        /// Default usage.
        const None = 0;

        /// Enable refitting for this acceleration structure. Note that this may reduce
        /// acceleration structure quality.
        const Refit = 1<<0;

        /// Prefer building this acceleration structure quickly at the cost of reduced ray
        /// tracing performance.
        const PreferFastBuild = 1<<1;

        /// Enable extended limits for this acceleration structure, possibly at the cost of
        /// reduced ray tracing performance.
        const ExtendedLimits = 1<<2;

        /// Prioritize intersection performance over acceleration structure build time.
        const PreferFastIntersection = 1<<4;

        /// Minimize the size of the acceleration structure in memory, potentially at
        /// the cost of increased build time or reduced intersection performance.
        const MinimizeMemory = 1<<5;
    }
}

unsafe impl Encode for MTLAccelerationStructureUsage {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for MTLAccelerationStructureUsage {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
