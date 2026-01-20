use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::Retained,
    runtime::{NSObject, ProtocolObject},
};
use objc2_foundation::{NSError, NSObjectProtocol};

use crate::{
    MTLCaptureScope,
    capture_manager::{MTLCaptureDescriptor, MTLCaptureDestination},
    command_queue::MTLCommandQueue,
    device::MTLDevice,
};

extern_class!(
    /// Retrieves the shared capture manager and provides APIs to create capture scopes and trigger captures from code.
    ///
    /// Availability: macOS 10.13+, iOS 11.0+
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLCaptureManager;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLCaptureManager {}
);

impl MTLCaptureManager {
    extern_methods!(
        /// Retrieves the shared capture manager for this process. There is only one capture manager per process.
        /// The capture manager allows the user to create capture scopes and trigger captures from code.
        /// When a capture has been completed, it will be displayed in Xcode and the application will be paused.
        /// Only `MTLCommandBuffer`s created after starting a capture and committed before stopping it are captured.
        #[unsafe(method(sharedCaptureManager))]
        #[unsafe(method_family = none)]
        pub fn shared_capture_manager() -> Retained<MTLCaptureManager>;

        /// Creates a new capture scope for the given capture device.
        #[unsafe(method(newCaptureScopeWithDevice:))]
        #[unsafe(method_family = new)]
        pub fn new_capture_scope_with_device(
            &self,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Retained<ProtocolObject<dyn MTLCaptureScope>>;

        /// Creates a new capture scope for the given command queue.
        #[unsafe(method(newCaptureScopeWithCommandQueue:))]
        #[unsafe(method_family = new)]
        pub fn new_capture_scope_with_command_queue(
            &self,
            command_queue: &ProtocolObject<dyn MTLCommandQueue>,
        ) -> Retained<ProtocolObject<dyn MTLCaptureScope>>;

        /// Creates a new capture scope for the given Metal 4 command queue.
        #[unsafe(method(newCaptureScopeWithMTL4CommandQueue:))]
        #[unsafe(method_family = new)]
        pub fn new_capture_scope_with_mtl4_command_queue(
            &self,
            command_queue: &ProtocolObject<dyn crate::MTL4CommandQueue>,
        ) -> Retained<ProtocolObject<dyn MTLCaptureScope>>;

        /// Checks if a given capture destination is supported.
        ///
        /// Availability: macOS 10.15+, iOS 13.0+
        #[unsafe(method(supportsDestination:))]
        #[unsafe(method_family = none)]
        pub fn supports_destination(&self, destination: MTLCaptureDestination) -> bool;

        /// Start capturing until stopCapture is called.
        ///
        /// - Parameter descriptor: `MTLCaptureDescriptor` specifies the parameters.
        /// - Parameter error: Optional error output to check why a capture could not be started.
        /// - Returns: `true` if the capture was successfully started, otherwise `false`.
        ///
        /// Remarks: Only `MTLCommandBuffer`s created after starting and committed before stopping are captured.
        ///
        /// Availability: macOS 10.15+, iOS 13.0+
        #[unsafe(method(startCaptureWithDescriptor:error:_))]
        #[unsafe(method_family = none)]
        pub fn start_capture_with_descriptor_error(
            &self,
            descriptor: &MTLCaptureDescriptor,
        ) -> Result<(), Retained<NSError>>;

        /// Starts capturing for all queues of the given device.
        ///
        /// Deprecated: Use `startCaptureWithDescriptor:error:` instead.
        #[unsafe(method(startCaptureWithDevice:))]
        #[unsafe(method_family = none)]
        pub fn start_capture_with_device(&self, device: &ProtocolObject<dyn MTLDevice>);

        /// Starts capturing for the given command queue.
        ///
        /// Deprecated: Use `startCaptureWithDescriptor:error:` instead.
        #[unsafe(method(startCaptureWithCommandQueue:))]
        #[unsafe(method_family = none)]
        pub fn start_capture_with_command_queue(
            &self,
            command_queue: &ProtocolObject<dyn MTLCommandQueue>,
        );

        /// Start a capture with the given scope: from the scope's begin until its end, restricting the capture to the scope's device(s) and, if selected, the scope's command queue.
        ///
        /// Deprecated: Use `startCaptureWithDescriptor:error:` instead.
        #[unsafe(method(startCaptureWithScope:))]
        #[unsafe(method_family = none)]
        pub fn start_capture_with_scope(&self, capture_scope: &ProtocolObject<dyn MTLCaptureScope>);

        /// Stop any ongoing capture.
        #[unsafe(method(stopCapture))]
        #[unsafe(method_family = none)]
        pub fn stop_capture(&self);

        /// Default scope to be captured when a capture is initiated from Xcode’s capture button. When `nil`, it’ll fall back to `presentDrawable:` methods.
        #[unsafe(method(defaultCaptureScope))]
        #[unsafe(method_family = none)]
        pub fn default_capture_scope(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn MTLCaptureScope>>>;

        /// Set default capture scope.
        #[unsafe(method(setDefaultCaptureScope:))]
        #[unsafe(method_family = none)]
        pub fn set_default_capture_scope(
            &self,
            scope: Option<&ProtocolObject<dyn MTLCaptureScope>>,
        );

        /// Query if a capture is currently in progress.
        #[unsafe(method(isCapturing))]
        #[unsafe(method_family = none)]
        pub fn is_capturing(&self) -> bool;
    );
}
