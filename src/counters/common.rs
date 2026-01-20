use objc2_foundation::NSString;

/// Common counters that, when present, have similar meanings across implementations.
pub type MTLCommonCounter = NSString;

#[allow(unused)]
unsafe extern "C" {
    pub static MTLCommonCounterTimestamp: &'static MTLCommonCounter;
    pub static MTLCommonCounterTessellationInputPatches: &'static MTLCommonCounter;
    pub static MTLCommonCounterVertexInvocations: &'static MTLCommonCounter;
    pub static MTLCommonCounterPostTessellationVertexInvocations: &'static MTLCommonCounter;
    pub static MTLCommonCounterClipperInvocations: &'static MTLCommonCounter;
    pub static MTLCommonCounterClipperPrimitivesOut: &'static MTLCommonCounter;
    pub static MTLCommonCounterFragmentInvocations: &'static MTLCommonCounter;
    pub static MTLCommonCounterFragmentsPassed: &'static MTLCommonCounter;
    pub static MTLCommonCounterComputeKernelInvocations: &'static MTLCommonCounter;
    pub static MTLCommonCounterTotalCycles: &'static MTLCommonCounter;
    pub static MTLCommonCounterVertexCycles: &'static MTLCommonCounter;
    pub static MTLCommonCounterTessellationCycles: &'static MTLCommonCounter;
    pub static MTLCommonCounterPostTessellationVertexCycles: &'static MTLCommonCounter;
    pub static MTLCommonCounterFragmentCycles: &'static MTLCommonCounter;
    pub static MTLCommonCounterRenderTargetWriteCycles: &'static MTLCommonCounter;
}

/// Common counter set names.
pub type MTLCommonCounterSet = NSString;

#[allow(unused)]
unsafe extern "C" {
    pub static MTLCommonCounterSetTimestamp: &'static MTLCommonCounterSet;
    pub static MTLCommonCounterSetStageUtilization: &'static MTLCommonCounterSet;
    pub static MTLCommonCounterSetStatistic: &'static MTLCommonCounterSet;
}
