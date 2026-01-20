use objc2::{Encode, Encoding, RefEncode};

/// Compression methods for Metal I/O handles (ported from `MTLIOCompressionMethod`).
#[repr(i64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum MTLIOCompressionMethod {
    Zlib = 0,
    Lzfse = 1,
    Lz4 = 2,
    Lzma = 3,
    LzBitmap = 4,
}

unsafe impl Encode for MTLIOCompressionMethod {
    const ENCODING: Encoding = i64::ENCODING;
}

unsafe impl RefEncode for MTLIOCompressionMethod {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
