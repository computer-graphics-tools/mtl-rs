use objc2::{Encode, Encoding, RefEncode};

/// Physical size of sparse resource page in KBs (from `MTLSparsePageSize`).
#[repr(i64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum MTLSparsePageSize {
    /// 16 KB sparse page size.
    KB16 = 101,
    /// 64 KB sparse page size.
    KB64 = 102,
    /// 256 KB sparse page size.
    KB256 = 103,
}

unsafe impl Encode for MTLSparsePageSize {
    const ENCODING: Encoding = i64::ENCODING;
}

unsafe impl RefEncode for MTLSparsePageSize {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
