use objc2::{Encode, Encoding, RefEncode};

/// Indirect arguments for `dispatchThreadgroups` (from `MTLDispatchThreadgroupsIndirectArguments`).
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MTLDispatchThreadgroupsIndirectArguments {
    pub threadgroups_per_grid: [u32; 3],
}

unsafe impl Encode for MTLDispatchThreadgroupsIndirectArguments {
    const ENCODING: Encoding = Encoding::Struct(
        "{MTLDispatchThreadgroupsIndirectArguments=[3I]}",
        &[<[u32; 3]>::ENCODING],
    );
}

unsafe impl RefEncode for MTLDispatchThreadgroupsIndirectArguments {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// Indirect arguments for `dispatchThreads` (from `MTLDispatchThreadsIndirectArguments`).
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MTLDispatchThreadsIndirectArguments {
    pub threads_per_grid: [u32; 3],
    pub threads_per_threadgroup: [u32; 3],
}

unsafe impl Encode for MTLDispatchThreadsIndirectArguments {
    const ENCODING: Encoding = Encoding::Struct(
        "{MTLDispatchThreadsIndirectArguments=[3I][3I]}",
        &[<[u32; 3]>::ENCODING, <[u32; 3]>::ENCODING],
    );
}

unsafe impl RefEncode for MTLDispatchThreadsIndirectArguments {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// Indirect arguments for stage-in region (from `MTLStageInRegionIndirectArguments`).
///
/// Availability: macOS 10.14+, iOS 12.0+
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MTLStageInRegionIndirectArguments {
    pub stage_in_origin: [u32; 3],
    pub stage_in_size: [u32; 3],
}

unsafe impl Encode for MTLStageInRegionIndirectArguments {
    const ENCODING: Encoding = Encoding::Struct(
        "{MTLStageInRegionIndirectArguments=[3I][3I]}",
        &[<[u32; 3]>::ENCODING, <[u32; 3]>::ENCODING],
    );
}

unsafe impl RefEncode for MTLStageInRegionIndirectArguments {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
