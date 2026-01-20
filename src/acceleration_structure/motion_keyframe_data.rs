use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
    runtime::{NSObject, ProtocolObject},
};
use objc2_foundation::NSObjectProtocol;

use crate::MTLBuffer;

extern_class!(
    /// MTLbuffer and description how the data is stored in it.
    ///
    /// Availability: API_AVAILABLE(macos(12.0), ios(15.0), tvos(16.0))
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metal/mtlmotionkeyframedata?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLMotionKeyframeData;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLMotionKeyframeData {}
);

impl MTLMotionKeyframeData {
    extern_methods!(
        /// Buffer containing the data of a single keyframe. Multiple keyframes can be interleaved in one MTLBuffer.
        #[unsafe(method(buffer))]
        #[unsafe(method_family = none)]
        pub unsafe fn buffer(&self) -> Option<Retained<ProtocolObject<dyn MTLBuffer>>>;

        /// Setter for [`buffer`][Self::buffer].
        #[unsafe(method(setBuffer:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_buffer(&self, buffer: Option<&ProtocolObject<dyn MTLBuffer>>);

        /// Buffer offset. Must be a multiple of 4 bytes.
        #[unsafe(method(offset))]
        #[unsafe(method_family = none)]
        pub unsafe fn offset(&self) -> usize;

        /// Setter for [`offset`][Self::offset].
        #[unsafe(method(setOffset:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_offset(&self, offset: usize);

        #[unsafe(method(data))]
        #[unsafe(method_family = none)]
        pub unsafe fn data() -> Retained<Self>;
    );
}

/// Methods declared on superclass `NSObject`.
impl MTLMotionKeyframeData {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
