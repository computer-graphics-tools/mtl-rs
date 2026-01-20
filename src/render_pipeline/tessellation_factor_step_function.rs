use objc2::{Encode, Encoding, RefEncode};

/// Tessellation factor step function (from `MTLTessellationFactorStepFunction`).
#[repr(u64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum MTLTessellationFactorStepFunction {
    Constant = 0,
    PerPatch = 1,
    PerInstance = 2,
    PerPatchAndPerInstance = 3,
}

unsafe impl Encode for MTLTessellationFactorStepFunction {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for MTLTessellationFactorStepFunction {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
