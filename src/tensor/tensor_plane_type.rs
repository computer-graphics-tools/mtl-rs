use objc2::{Encode, Encoding, RefEncode};

/// The possible tensor plane types (from `MTLTensorPlaneType`).
#[repr(isize)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum MTLTensorPlaneType {
    /// The main data plane, which every tensor has.
    Data = 0,
    /// The auxiliary plane that stores scale factors for elements in the data plane.
    Scales = 1,
}

unsafe impl Encode for MTLTensorPlaneType {
    const ENCODING: Encoding = isize::ENCODING;
}

unsafe impl RefEncode for MTLTensorPlaneType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
