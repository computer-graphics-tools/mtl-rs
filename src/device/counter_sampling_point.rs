use objc2::{Encode, Encoding, RefEncode};

/// Counter sampling points supported by a device (from `MTLCounterSamplingPoint`).
#[repr(u64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum MTLCounterSamplingPoint {
    AtStageBoundary = 0,
    AtDrawBoundary = 1,
    AtDispatchBoundary = 2,
    AtTileDispatchBoundary = 3,
    AtBlitBoundary = 4,
}

unsafe impl Encode for MTLCounterSamplingPoint {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for MTLCounterSamplingPoint {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
