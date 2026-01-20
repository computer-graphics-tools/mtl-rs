use objc2::{Encode, Encoding, RefEncode};

/// Controls the acceleration structure refit operation.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLAccelerationStructureRefitOptions(pub u64);

bitflags::bitflags! {
    impl MTLAccelerationStructureRefitOptions: u64 {
        /// Refitting shall result in updated vertex data from the provided geometry descriptor.
        /// If not set, vertex buffers shall be ignored on the geometry descriptor and vertex data previously
        /// encoded shall be copied.
        const VertexData = 1 << 0;

        /// Refitting shall result in updated per primitive data from the provided geometry descriptor.
        /// If not set, per primitive data buffers shall be ignored on the geometry descriptor and per primitive
        /// data previously encoded shall be copied.
        const PerPrimitiveData = 1 << 1;
    }
}

unsafe impl Encode for MTLAccelerationStructureRefitOptions {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for MTLAccelerationStructureRefitOptions {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
