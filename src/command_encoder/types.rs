use objc2::{Encode, Encoding, RefEncode};

/// Describes how a resource will be used by a shader through an argument buffer (from `MTLResourceUsage`).
///
/// Availability: macOS 10.13+, iOS 11.0+
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLResourceUsage(pub u64);
bitflags::bitflags! {
    impl MTLResourceUsage: u64 {
        const Read = 1<<0;
        const Write = 1<<1;
        const Sample = 1<<2; // deprecated upstream but keep for compatibility
    }
}

unsafe impl Encode for MTLResourceUsage {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for MTLResourceUsage {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// Describes the types of resources that a barrier operates on (from `MTLBarrierScope`).
///
/// Availability: macOS 10.14+, iOS 12.0+
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLBarrierScope(pub u64);
bitflags::bitflags! {
    impl MTLBarrierScope: u64 {
        const Buffers = 1<<0;
        const Textures = 1<<1;
        const RenderTargets = 1<<2;
    }
}

unsafe impl Encode for MTLBarrierScope {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for MTLBarrierScope {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// Describes stages of GPU work (from `MTLStages`).
///
/// Availability: macOS 26.0+, iOS 26.0+
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLRenderStages(pub u64);
bitflags::bitflags! {
    impl MTLRenderStages: u64 {
        const Vertex = 1<<0;
        const Fragment = 1<<1;
        const Tile = 1<<2;
        const Object = 1<<3;
        const Mesh = 1<<4;
        const ResourceState = 1<<26;
        const Dispatch = 1<<27;
        const Blit = 1<<28;
        const AccelerationStructure = 1<<29;
        const MachineLearning = 1<<30;
        const All = u64::MAX; // matches NSIntegerMax bitmask intent
    }
}

unsafe impl Encode for MTLRenderStages {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for MTLRenderStages {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
