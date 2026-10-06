use core::ops::Range;

use objc2::{Message, extern_protocol, msg_send, rc::Retained, runtime::ProtocolObject};
use objc2_foundation::{NSData, NSObjectProtocol, NSRange, NSString};

use crate::MTLDevice;

extern_protocol!(
    /// The Counter Sample Buffer contains opaque counter samples as well
    /// as state needed to request a sample from the API.
    pub unsafe trait MTLCounterSampleBuffer: NSObjectProtocol + Send + Sync {
        /// The device this sample buffer was created against.
        #[unsafe(method(device))]
        #[unsafe(method_family = none)]
        fn device(&self) -> Retained<ProtocolObject<dyn MTLDevice>>;

        /// Number of samples in this buffer.
        #[unsafe(method(sampleCount))]
        #[unsafe(method_family = none)]
        fn sample_count(&self) -> usize;
    }
);

pub trait MTLCounterSampleBufferExt: MTLCounterSampleBuffer + Message {
    fn label(&self) -> String
    where
        Self: Sized,
    {
        let label: Retained<NSString> = unsafe { msg_send![self, label] };
        label.to_string()
    }

    fn resolve_counter_range(
        &self,
        range: Range<usize>,
    ) -> Option<Box<[u8]>>
    where
        Self: Sized,
    {
        let data: Option<Retained<NSData>> = unsafe { msg_send![self, resolveCounterRange: NSRange::from(range)] };
        data.map(|data| data.to_vec().into_boxed_slice())
    }
}

impl<T: MTLCounterSampleBuffer + Message> MTLCounterSampleBufferExt for T {}
