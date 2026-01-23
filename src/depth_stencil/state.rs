use objc2::{Message, extern_protocol, msg_send, rc::Retained, runtime::ProtocolObject};
use objc2_foundation::{NSObjectProtocol, NSString};

use crate::{MTLDevice, MTLResourceID};

extern_protocol!(
    /// Device-specific compiled depth/stencil state object.
    ///
    /// Availability: macOS 10.11+, iOS 8.0+
    pub unsafe trait MTLDepthStencilState: NSObjectProtocol + Send + Sync {
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

#[allow(unused)]
pub trait MTLDepthStencilStateExt: MTLDepthStencilState + Message {
    fn label(&self) -> Option<String>;
}

impl MTLDepthStencilStateExt for ProtocolObject<dyn MTLDepthStencilState> {
    fn label(&self) -> Option<String> {
        let label: Option<Retained<NSString>> = unsafe { msg_send![self, label] };
        label.map(|s| s.to_string())
    }
}
