use objc2::{Message, extern_protocol, msg_send, rc::Retained, runtime::ProtocolObject};
use objc2_foundation::{NSObjectProtocol, NSString};

use crate::{MTLDevice, MTLResourceID};

extern_protocol!(
    /// An immutable collection of sampler state compiled for a single device.
    ///
    /// # Thread safety
    ///
    /// [Apple's declaration](https://developer.apple.com/documentation/metal/mtlsamplerstate):
    ///
    /// > `protocol MTLSamplerState : NSObjectProtocol, Sendable`
    ///
    /// The `Send` and `Sync` bounds rely on this guarantee.
    pub unsafe trait MTLSamplerState: NSObjectProtocol + Send + Sync {
        /// The device this resource was created against.
        #[unsafe(method(device))]
        #[unsafe(method_family = none)]
        fn device(&self) -> Retained<ProtocolObject<dyn MTLDevice>>;

        /// Handle of the GPU resource suitable for storing in an Argument Buffer
        #[unsafe(method(gpuResourceID))]
        #[unsafe(method_family = none)]
        fn gpu_resource_id(&self) -> MTLResourceID;
    }
);

/// Rust-native accessors for sampler state.
pub trait MTLSamplerStateExt: MTLSamplerState + Message {
    /// A string to help identify this object.
    fn label(&self) -> Option<String>
    where
        Self: Sized,
    {
        let label: Option<Retained<NSString>> = unsafe { msg_send![self, label] };
        label.map(|label| label.to_string())
    }
}

impl<T: MTLSamplerState + Message> MTLSamplerStateExt for T {}
