use std::path::{Path, PathBuf};

use objc2::{
    extern_class, extern_conformance, extern_methods, msg_send,
    rc::{Allocated, Retained},
    runtime::{AnyObject, NSObject},
};
use objc2_foundation::{CopyingHelper, NSCopying, NSObjectProtocol, NSURL};

use crate::capture_manager::MTLCaptureDestination;

extern_class!(
    /// Parameters that describe how to start a capture.
    ///
    /// Availability: macOS 10.15+, iOS 13.0+
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLCaptureDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for MTLCaptureDescriptor {}
);

unsafe impl CopyingHelper for MTLCaptureDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLCaptureDescriptor {}
);

impl MTLCaptureDescriptor {
    extern_methods!(
        /// The object that is captured.
        ///
        /// Must be one of the following:
        /// - `MTLDevice` captures all command queues of the device.
        /// - `MTLCommandQueue` captures a single command queue.
        /// - `MTLCaptureScope` captures between the next begin and end of the scope.
        #[unsafe(method(captureObject))]
        #[unsafe(method_family = none)]
        pub fn capture_object(&self) -> Option<Retained<AnyObject>>;

        /// Safety: `capture_object` should be of the correct type.
        #[unsafe(method(setCaptureObject:))]
        #[unsafe(method_family = none)]
        pub fn set_capture_object(&self, capture_object: Option<&AnyObject>);

        /// The destination where you want the GPU trace to be captured to.
        #[unsafe(method(destination))]
        #[unsafe(method_family = none)]
        pub fn destination(&self) -> MTLCaptureDestination;

        #[unsafe(method(setDestination:))]
        #[unsafe(method_family = none)]
        pub fn set_destination(&self, destination: MTLCaptureDestination);
    );

    /// Filesystem path where the GPU trace document will be captured.
    ///
    /// Must be specified when destination is `MTLCaptureDestination::GPUTraceDocument`.
    pub fn output_path(&self) -> Option<PathBuf> {
        let output_url: Option<Retained<NSURL>> = unsafe { msg_send![self, outputURL] };
        output_url.and_then(|url| url.to_file_path())
    }

    pub fn set_output_path(&self, output_path: Option<&Path>) {
        let output_url = output_path.and_then(NSURL::from_file_path);
        unsafe {
            let _: () = msg_send![self, setOutputURL: output_url.as_deref()];
        }
    }
}

impl MTLCaptureDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub fn new() -> Retained<Self>;
    );
}
