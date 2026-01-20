use objc2::{Encode, Encoding, RefEncode};

/// Metal feature sets (ported from `MTLFeatureSet`).
#[repr(u64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum MTLFeatureSet {
    IosGpuFamily1V1 = 0,
    IosGpuFamily2V1 = 1,

    IosGpuFamily1V2 = 2,
    IosGpuFamily2V2 = 3,
    IosGpuFamily3V1 = 4,

    IosGpuFamily1V3 = 5,
    IosGpuFamily2V3 = 6,
    IosGpuFamily3V2 = 7,

    IosGpuFamily1V4 = 8,
    IosGpuFamily2V4 = 9,
    IosGpuFamily3V3 = 10,
    IosGpuFamily4V1 = 11,

    IosGpuFamily1V5 = 12,
    IosGpuFamily2V5 = 13,
    IosGpuFamily3V4 = 14,
    IosGpuFamily4V2 = 15,
    IosGpuFamily5V1 = 16,

    MacosGpuFamily1V1 = 10000,
    MacosGpuFamily1V2 = 10001,
    MacosReadWriteTextureTier2 = 10002,

    MacosGpuFamily1V3 = 10003,

    MacosGpuFamily1V4 = 10004,
    MacosGpuFamily2V1 = 10005,

    TvosGpuFamily1V1 = 30000,
    TvosGpuFamily1V2 = 30001,
    TvosGpuFamily1V3 = 30002,
    TvosGpuFamily2V1 = 30003,

    TvosGpuFamily1V4 = 30004,
    TvosGpuFamily2V2 = 30005,
}

unsafe impl Encode for MTLFeatureSet {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for MTLFeatureSet {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
