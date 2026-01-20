use objc2::{Encode, Encoding, RefEncode};

/// Memory size requirements for an acceleration structure (from `MTLAccelerationStructureSizes`).
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct MTLAccelerationStructureSizes {
    /// The required size, in bytes, of the built acceleration structure
    pub acceleration_structure_size: usize,
    /// The required size, in bytes, of the scratch buffer used to build the acceleration structure
    pub build_scratch_buffer_size: usize,
    /// The required size, in bytes, of the scratch buffer used to refit the acceleration structure
    pub refit_scratch_buffer_size: usize,
}

unsafe impl Encode for MTLAccelerationStructureSizes {
    const ENCODING: Encoding = Encoding::Struct(
        "{MTLAccelerationStructureSizes=QQQ}",
        &[usize::ENCODING, usize::ENCODING, usize::ENCODING],
    );
}

unsafe impl RefEncode for MTLAccelerationStructureSizes {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
