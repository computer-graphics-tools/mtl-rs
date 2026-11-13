use objc2::{Encode, Encoding, RefEncode};

/// Describes how a resource will be used by a shader through an argument buffer (from `MTLResourceUsage`).
///
/// Availability: macOS 10.13+, iOS 11.0+
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLResourceUsage(pub usize);
bitflags::bitflags! {
    impl MTLResourceUsage: usize {
        const Read = 1<<0;
        const Write = 1<<1;
        #[deprecated(note = "use MTLResourceUsage::Read")]
        const Sample = 1<<2;
    }
}

unsafe impl Encode for MTLResourceUsage {
    const ENCODING: Encoding = usize::ENCODING;
}

unsafe impl RefEncode for MTLResourceUsage {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// Describes the types of resources that a barrier operates on (from `MTLBarrierScope`).
///
/// Availability: macOS 10.14+, iOS 12.0+
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLBarrierScope(pub usize);
bitflags::bitflags! {
    impl MTLBarrierScope: usize {
        const Buffers = 1<<0;
        const Textures = 1<<1;
        /// Available on macOS and Mac Catalyst; unavailable on iOS.
        const RenderTargets = 1<<2;
    }
}

unsafe impl Encode for MTLBarrierScope {
    const ENCODING: Encoding = usize::ENCODING;
}

unsafe impl RefEncode for MTLBarrierScope {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// Describes stages of GPU work (from `MTLStages`).
///
/// Availability: macOS 26.0+, iOS 26.0+
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLStages(pub usize);
bitflags::bitflags! {
    impl MTLStages: usize {
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
        const All = isize::MAX as usize;
    }
}

unsafe impl Encode for MTLStages {
    const ENCODING: Encoding = usize::ENCODING;
}

unsafe impl RefEncode for MTLStages {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
