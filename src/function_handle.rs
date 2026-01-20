use objc2::{extern_protocol, rc::Retained, runtime::ProtocolObject};
use objc2_foundation::{NSObjectProtocol, NSString};

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
        fn function_type(&self) -> crate::MTLFunctionType;

        /// The name of the function.
        ///
        /// Availability: macOS 11.0+, iOS 14.0+, tvOS 16.0+
        #[unsafe(method(name))]
        #[unsafe(method_family = none)]
        fn name(&self) -> Retained<NSString>;

        /// The device that created this handle.
        ///
        /// Availability: macOS 11.0+, iOS 14.0+, tvOS 16.0+
        #[unsafe(method(device))]
        #[unsafe(method_family = none)]
        fn device(&self) -> Retained<ProtocolObject<dyn crate::MTLDevice>>;

        /// Handle of the GPU resource suitable for storing in an Intersection Function Buffer.
        /// The handle must have been created from an intersection function annotated with the
        /// `intersection_function_buffer` tag.
        ///
        /// Availability: macOS 26.0+, iOS 26.0+
        #[unsafe(method(gpuResourceID))]
        #[unsafe(method_family = none)]
        unsafe fn gpu_resource_id(&self) -> crate::types::MTLResourceID;
    }
);
