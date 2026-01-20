use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
    runtime::{NSObject, ProtocolObject},
};
use objc2_foundation::{CopyingHelper, NSCopying, NSObjectProtocol, NSString};

use super::MTLCounterSet;
use crate::resource::MTLStorageMode;

extern_class!(
    /// Object to represent the counter state.
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLCounterSampleBufferDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for MTLCounterSampleBufferDescriptor {}
);

unsafe impl CopyingHelper for MTLCounterSampleBufferDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLCounterSampleBufferDescriptor {}
);

impl MTLCounterSampleBufferDescriptor {
    extern_methods!(
        #[unsafe(method(counterSet))]
        #[unsafe(method_family = none)]
        pub unsafe fn counter_set(&self) -> Option<Retained<ProtocolObject<dyn MTLCounterSet>>>;

        #[unsafe(method(setCounterSet:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_counter_set(
            &self,
            counter_set: Option<&ProtocolObject<dyn MTLCounterSet>>,
        );

        #[unsafe(method(label))]
        #[unsafe(method_family = none)]
        pub unsafe fn label(&self) -> Retained<NSString>;

        #[unsafe(method(setLabel:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_label(&self, label: &NSString);

        /// The storage mode for the sample buffer. Only `Shared` and `Private` may be used.
        #[unsafe(method(storageMode))]
        #[unsafe(method_family = none)]
        pub unsafe fn storage_mode(&self) -> MTLStorageMode;

        #[unsafe(method(setStorageMode:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_storage_mode(&self, mode: MTLStorageMode);

        #[unsafe(method(sampleCount))]
        #[unsafe(method_family = none)]
        pub unsafe fn sample_count(&self) -> usize;

        #[unsafe(method(setSampleCount:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_sample_count(&self, sample_count: usize);
    );
}

impl MTLCounterSampleBufferDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
