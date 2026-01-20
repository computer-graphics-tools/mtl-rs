use bitflags::bitflags;
use objc2::{Encode, Encoding, RefEncode};

bitflags! {
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
    /// The type that represents the different contexts for a tensor.
    pub struct MTLTensorUsage: usize {
        /// A tensor context that applies to compute encoders.
        ///
        /// You can use tensors with this context in ``MTL4ComputeCommandEncoder`` or ``MTLComputeCommandEncoder`` instances.
        const COMPUTE = 1 << 0;
        /// A tensor context that applies to render encoders.
        ///
        /// You can use tensors with this context in ``MTL4RenderCommandEncoder`` or ``MTLRenderCommandEncoder`` instances.
        const RENDER  = 1 << 1;
        /// A tensor context that applies to machine learning encoders.
        ///
        /// You can use tensors with this context in ``MTL4MachineLearningCommandEncoder`` instances.
        const MACHINE_LEARNING = 1 << 2;
    }
}

unsafe impl Encode for MTLTensorUsage {
    const ENCODING: Encoding = usize::ENCODING;
}

unsafe impl RefEncode for MTLTensorUsage {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
