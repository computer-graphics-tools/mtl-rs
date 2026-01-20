use objc2::{Encode, Encoding, RefEncode};

/// Tessellation partition mode (from `MTLTessellationPartitionMode`).
#[repr(u64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum MTLTessellationPartitionMode {
    Pow2 = 0,
    Integer = 1,
    FractionalOdd = 2,
    FractionalEven = 3,
}

unsafe impl Encode for MTLTessellationPartitionMode {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for MTLTessellationPartitionMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
