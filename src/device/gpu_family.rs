use objc2::{Encode, Encoding, RefEncode};

/// Metal GPU family (ported from `MTLGPUFamily`).
#[repr(isize)]
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
    Apple10 = 1010,

    #[deprecated(note = "use Apple7")]
    Mac1 = 2001,
    #[deprecated(note = "use Apple7")]
    Mac2 = 2002,

    #[deprecated(note = "use Apple1")]
    Common1 = 3001,
    #[deprecated(note = "use Apple3")]
    Common2 = 3002,
    #[deprecated(note = "use Apple5")]
    Common3 = 3003,

    #[deprecated(note = "use Apple7")]
    MacCatalyst1 = 4001,
    #[deprecated(note = "use Apple7")]
    MacCatalyst2 = 4002,

    Metal3 = 5001,
    Metal4 = 5002,
}

unsafe impl Encode for MTLGPUFamily {
    const ENCODING: Encoding = isize::ENCODING;
}

unsafe impl RefEncode for MTLGPUFamily {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
