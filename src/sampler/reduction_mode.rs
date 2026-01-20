use objc2::{Encode, Encoding, RefEncode};

/// Reduction mode for sampler filtering (from `MTLSamplerReductionMode`).
#[repr(u64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum MTLSamplerReductionMode {
    WeightedAverage = 0,
    Minimum = 1,
    Maximum = 2,
}

unsafe impl Encode for MTLSamplerReductionMode {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for MTLSamplerReductionMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
