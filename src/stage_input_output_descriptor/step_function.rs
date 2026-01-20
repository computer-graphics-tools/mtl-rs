use objc2::{Encode, Encoding, RefEncode};

/// Step function for buffer layouts (ported from `MTLStepFunction`).
#[repr(u64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum MTLStepFunction {
    Constant = 0,
    PerVertex = 1,
    PerInstance = 2,
    PerPatch = 3,
    PerPatchControlPoint = 4,
    ThreadPositionInGridX = 5,
    ThreadPositionInGridY = 6,
    ThreadPositionInGridXIndexed = 7,
    ThreadPositionInGridYIndexed = 8,
}

unsafe impl Encode for MTLStepFunction {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for MTLStepFunction {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
