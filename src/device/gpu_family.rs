use objc2::{Encode, Encoding, RefEncode};

/// Metal GPU family (ported from `MTLGPUFamily`).
#[repr(i64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum MTLGPUFamily {
    Apple1 = 1001,
    Apple2 = 1002,
    Apple3 = 1003,
    Apple4 = 1004,
    Apple5 = 1005,
    Apple6 = 1006,
    Apple7 = 1007,
    Apple8 = 1008,
    Apple9 = 1009,

    Mac1 = 2001, // deprecated in headers, kept for completeness
    Mac2 = 2002,

    Common1 = 3001,
    Common2 = 3002,
    Common3 = 3003,

    MacCatalyst1 = 4001, // deprecated in headers, kept for completeness
    MacCatalyst2 = 4002, // deprecated in headers, kept for completeness

    Metal3 = 5001,
    Metal4 = 5002,
}

unsafe impl Encode for MTLGPUFamily {
    const ENCODING: Encoding = i64::ENCODING;
}

unsafe impl RefEncode for MTLGPUFamily {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
