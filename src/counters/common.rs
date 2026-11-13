use objc2_foundation::NSString;

/// A common counter name that has similar meaning across Metal implementations.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MTLCommonCounter {
    Timestamp,
    TessellationInputPatches,
    VertexInvocations,
    PostTessellationVertexInvocations,
    ClipperInvocations,
    ClipperPrimitivesOut,
    FragmentInvocations,
    FragmentsPassed,
    ComputeKernelInvocations,
    TotalCycles,
    VertexCycles,
    TessellationCycles,
    PostTessellationVertexCycles,
    FragmentCycles,
    RenderTargetWriteCycles,
}

impl MTLCommonCounter {
    /// Returns Metal's platform-defined string for this counter.
    pub fn name(self) -> String {
        let name = unsafe {
            match self {
                Self::Timestamp => MTL_COMMON_COUNTER_TIMESTAMP,
                Self::TessellationInputPatches => MTL_COMMON_COUNTER_TESSELLATION_INPUT_PATCHES,
                Self::VertexInvocations => MTL_COMMON_COUNTER_VERTEX_INVOCATIONS,
                Self::PostTessellationVertexInvocations => MTL_COMMON_COUNTER_POST_TESSELLATION_VERTEX_INVOCATIONS,
                Self::ClipperInvocations => MTL_COMMON_COUNTER_CLIPPER_INVOCATIONS,
                Self::ClipperPrimitivesOut => MTL_COMMON_COUNTER_CLIPPER_PRIMITIVES_OUT,
                Self::FragmentInvocations => MTL_COMMON_COUNTER_FRAGMENT_INVOCATIONS,
                Self::FragmentsPassed => MTL_COMMON_COUNTER_FRAGMENTS_PASSED,
                Self::ComputeKernelInvocations => MTL_COMMON_COUNTER_COMPUTE_KERNEL_INVOCATIONS,
                Self::TotalCycles => MTL_COMMON_COUNTER_TOTAL_CYCLES,
                Self::VertexCycles => MTL_COMMON_COUNTER_VERTEX_CYCLES,
                Self::TessellationCycles => MTL_COMMON_COUNTER_TESSELLATION_CYCLES,
                Self::PostTessellationVertexCycles => MTL_COMMON_COUNTER_POST_TESSELLATION_VERTEX_CYCLES,
                Self::FragmentCycles => MTL_COMMON_COUNTER_FRAGMENT_CYCLES,
                Self::RenderTargetWriteCycles => MTL_COMMON_COUNTER_RENDER_TARGET_WRITE_CYCLES,
            }
        };
        name.to_string()
    }
}

/// A common counter-set name.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MTLCommonCounterSet {
    Timestamp,
    StageUtilization,
    Statistic,
}

impl MTLCommonCounterSet {
    /// Returns Metal's platform-defined string for this counter set.
    pub fn name(self) -> String {
        let name = unsafe {
            match self {
                Self::Timestamp => MTL_COMMON_COUNTER_SET_TIMESTAMP,
                Self::StageUtilization => MTL_COMMON_COUNTER_SET_STAGE_UTILIZATION,
                Self::Statistic => MTL_COMMON_COUNTER_SET_STATISTIC,
            }
        };
        name.to_string()
    }
}

unsafe extern "C" {
    #[link_name = "MTLCommonCounterTimestamp"]
    static MTL_COMMON_COUNTER_TIMESTAMP: &'static NSString;
    #[link_name = "MTLCommonCounterTessellationInputPatches"]
    static MTL_COMMON_COUNTER_TESSELLATION_INPUT_PATCHES: &'static NSString;
    #[link_name = "MTLCommonCounterVertexInvocations"]
    static MTL_COMMON_COUNTER_VERTEX_INVOCATIONS: &'static NSString;
    #[link_name = "MTLCommonCounterPostTessellationVertexInvocations"]
    static MTL_COMMON_COUNTER_POST_TESSELLATION_VERTEX_INVOCATIONS: &'static NSString;
    #[link_name = "MTLCommonCounterClipperInvocations"]
    static MTL_COMMON_COUNTER_CLIPPER_INVOCATIONS: &'static NSString;
    #[link_name = "MTLCommonCounterClipperPrimitivesOut"]
    static MTL_COMMON_COUNTER_CLIPPER_PRIMITIVES_OUT: &'static NSString;
    #[link_name = "MTLCommonCounterFragmentInvocations"]
    static MTL_COMMON_COUNTER_FRAGMENT_INVOCATIONS: &'static NSString;
    #[link_name = "MTLCommonCounterFragmentsPassed"]
    static MTL_COMMON_COUNTER_FRAGMENTS_PASSED: &'static NSString;
    #[link_name = "MTLCommonCounterComputeKernelInvocations"]
    static MTL_COMMON_COUNTER_COMPUTE_KERNEL_INVOCATIONS: &'static NSString;
    #[link_name = "MTLCommonCounterTotalCycles"]
    static MTL_COMMON_COUNTER_TOTAL_CYCLES: &'static NSString;
    #[link_name = "MTLCommonCounterVertexCycles"]
    static MTL_COMMON_COUNTER_VERTEX_CYCLES: &'static NSString;
    #[link_name = "MTLCommonCounterTessellationCycles"]
    static MTL_COMMON_COUNTER_TESSELLATION_CYCLES: &'static NSString;
    #[link_name = "MTLCommonCounterPostTessellationVertexCycles"]
    static MTL_COMMON_COUNTER_POST_TESSELLATION_VERTEX_CYCLES: &'static NSString;
    #[link_name = "MTLCommonCounterFragmentCycles"]
    static MTL_COMMON_COUNTER_FRAGMENT_CYCLES: &'static NSString;
    #[link_name = "MTLCommonCounterRenderTargetWriteCycles"]
    static MTL_COMMON_COUNTER_RENDER_TARGET_WRITE_CYCLES: &'static NSString;

    #[link_name = "MTLCommonCounterSetTimestamp"]
    static MTL_COMMON_COUNTER_SET_TIMESTAMP: &'static NSString;
    #[link_name = "MTLCommonCounterSetStageUtilization"]
    static MTL_COMMON_COUNTER_SET_STAGE_UTILIZATION: &'static NSString;
    #[link_name = "MTLCommonCounterSetStatistic"]
    static MTL_COMMON_COUNTER_SET_STATISTIC: &'static NSString;
}
