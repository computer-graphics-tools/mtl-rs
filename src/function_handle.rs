use objc2::{Message, extern_protocol, msg_send, rc::Retained, runtime::ProtocolObject};
use objc2_foundation::{NSObjectProtocol, NSString};

use crate::{MTLDevice, MTLFunctionType};

extern_protocol!(
    /// Apple's documentation: `https://developer.apple.com/documentation/metal/mtlfunctionhandle?language=objc`
    ///
    /// Availability: macOS 11.0+, iOS 14.0+, tvOS 16.0+
    pub unsafe trait MTLFunctionHandle: NSObjectProtocol + Send + Sync {
        /// The type of the function represented by this handle.
        ///
        /// Availability: macOS 11.0+, iOS 14.0+, tvOS 16.0+
        #[unsafe(method(functionType))]
        #[unsafe(method_family = none)]
        fn function_type(&self) -> MTLFunctionType;

        /// The device that created this handle.
        ///
        /// Availability: macOS 11.0+, iOS 14.0+, tvOS 16.0+
        #[unsafe(method(device))]
        #[unsafe(method_family = none)]
        fn device(&self) -> Retained<ProtocolObject<dyn MTLDevice>>;

        /// Handle of the GPU resource suitable for storing in an Intersection Function Buffer.
        /// The handle must have been created from an intersection function annotated with the
        /// `intersection_function_buffer` tag.
        ///
        /// Availability: macOS 26.0+, iOS 26.0+
        #[unsafe(method(gpuResourceID))]
        #[unsafe(method_family = none)]
        fn gpu_resource_id(&self) -> crate::types::MTLResourceID;
    }
);

pub trait MTLFunctionHandleExt: MTLFunctionHandle + Message {
    fn name(&self) -> String
    where
        Self: Sized,
    {
        let name: Retained<NSString> = unsafe { msg_send![self, name] };
        name.to_string()
    }
}

impl<T: MTLFunctionHandle + Message> MTLFunctionHandleExt for T {}
