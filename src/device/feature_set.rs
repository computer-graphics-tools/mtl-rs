use objc2::{Encode, Encoding, RefEncode};

/// Metal feature sets
#[allow(non_camel_case_types)]
#[repr(u64)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum MTLFeatureSet {
    iOS_GPUFamily1_v1 = 0,
    iOS_GPUFamily2_v1 = 1,

    iOS_GPUFamily1_v2 = 2,
    iOS_GPUFamily2_v2 = 3,
    iOS_GPUFamily3_v1 = 4,

    iOS_GPUFamily1_v3 = 5,
    iOS_GPUFamily2_v3 = 6,
    iOS_GPUFamily3_v2 = 7,

    iOS_GPUFamily1_v4 = 8,
    iOS_GPUFamily2_v4 = 9,
    iOS_GPUFamily3_v3 = 10,
    iOS_GPUFamily4_v1 = 11,

    iOS_GPUFamily1_v5 = 12,
    iOS_GPUFamily2_v5 = 13,
    iOS_GPUFamily3_v4 = 14,
    iOS_GPUFamily4_v2 = 15,
    iOS_GPUFamily5_v1 = 16,

    macOS_GPUFamily1_v1 = 10000,
    macOS_GPUFamily1_v2 = 10001,
    macOS_ReadWriteTextureTier2 = 10002,

    macOS_GPUFamily1_v3 = 10003,

    macOS_GPUFamily1_v4 = 10004,
    macOS_GPUFamily2_v1 = 10005,

    tvOS_GPUFamily1_v1 = 30000,
    tvOS_GPUFamily1_v2 = 30001,
    tvOS_GPUFamily1_v3 = 30002,
    tvOS_GPUFamily2_v1 = 30003,

    tvOS_GPUFamily1_v4 = 30004,
    tvOS_GPUFamily2_v2 = 30005,
}

unsafe impl Encode for MTLFeatureSet {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for MTLFeatureSet {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
