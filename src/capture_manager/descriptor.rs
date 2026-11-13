use std::path::{Path, PathBuf};

use objc2::{
    ProtocolType, extern_class, extern_conformance, extern_methods, msg_send,
    rc::{Allocated, Retained},
    runtime::{AnyObject, NSObject, ProtocolObject},
};
use objc2_foundation::{CopyingHelper, NSCopying, NSObjectProtocol, NSURL};

use crate::{MTLCaptureScope, MTLCommandQueue, MTLDevice, capture_manager::MTLCaptureDestination};

/// A Metal object that can be captured by [`MTLCaptureDescriptor`].
#[derive(Clone, Debug)]
pub enum MTLCaptureTarget {
    /// Captures all command queues belonging to the device.
    Device(Retained<ProtocolObject<dyn MTLDevice>>),
    /// Captures a single command queue.
    CommandQueue(Retained<ProtocolObject<dyn MTLCommandQueue>>),
    /// Captures work bracketed by the capture scope.
    Scope(Retained<ProtocolObject<dyn MTLCaptureScope>>),
}

impl MTLCaptureTarget {
    fn as_any_object(&self) -> &AnyObject {
        match self {
            Self::Device(target) => protocol_object_as_any(target),
            Self::CommandQueue(target) => protocol_object_as_any(target),
            Self::Scope(target) => protocol_object_as_any(target),
        }
    }
}

fn protocol_object_as_any<P: ?Sized + 'static>(object: &ProtocolObject<P>) -> &AnyObject {
    <ProtocolObject<P> as AsRef<AnyObject>>::as_ref(object)
}

/// The descriptor contains a capture object outside Metal's supported target kinds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UnknownMTLCaptureTarget;

impl core::fmt::Display for UnknownMTLCaptureTarget {
    fn fmt(
        &self,
        formatter: &mut core::fmt::Formatter<'_>,
    ) -> core::fmt::Result {
        formatter.write_str("capture object is not a Metal device, command queue, or capture scope")
    }
}

impl std::error::Error for UnknownMTLCaptureTarget {}

fn conforms_to<P: ?Sized + ProtocolType>(object: &AnyObject) -> bool {
    let Some(protocol) = P::protocol() else {
        return false;
    };
    unsafe { msg_send![object, conformsToProtocol: protocol] }
}

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
        /// The destination where you want the GPU trace to be captured to.
        #[unsafe(method(destination))]
        #[unsafe(method_family = none)]
        pub fn destination(&self) -> MTLCaptureDestination;

        #[unsafe(method(setDestination:))]
        #[unsafe(method_family = none)]
        pub fn set_destination(
            &self,
            destination: MTLCaptureDestination,
        );
    );

    /// Returns the Metal object that is captured.
    ///
    /// An error means the property was populated outside this API with an
    /// object that does not conform to one of Metal's three supported target protocols.
    pub fn capture_object(&self) -> Result<Option<MTLCaptureTarget>, UnknownMTLCaptureTarget> {
        let object: Option<Retained<AnyObject>> = unsafe { msg_send![self, captureObject] };
        let Some(object) = object else {
            return Ok(None);
        };

        if conforms_to::<dyn MTLDevice>(&object) {
            // SAFETY: The runtime conformance check above proves that this
            // object implements `MTLDevice` before changing its Rust view.
            let target = unsafe { Retained::cast_unchecked(object) };
            return Ok(Some(MTLCaptureTarget::Device(target)));
        }
        if conforms_to::<dyn MTLCommandQueue>(&object) {
            // SAFETY: The runtime conformance check above proves that this
            // object implements `MTLCommandQueue` before changing its Rust view.
            let target = unsafe { Retained::cast_unchecked(object) };
            return Ok(Some(MTLCaptureTarget::CommandQueue(target)));
        }
        if conforms_to::<dyn MTLCaptureScope>(&object) {
            // SAFETY: The runtime conformance check above proves that this
            // object implements `MTLCaptureScope` before changing its Rust view.
            let target = unsafe { Retained::cast_unchecked(object) };
            return Ok(Some(MTLCaptureTarget::Scope(target)));
        }

        Err(UnknownMTLCaptureTarget)
    }

    /// Sets the Metal object that is captured.
    pub fn set_capture_object(
        &self,
        capture_object: Option<&MTLCaptureTarget>,
    ) {
        let capture_object = capture_object.map(MTLCaptureTarget::as_any_object);
        unsafe {
            let _: () = msg_send![self, setCaptureObject: capture_object];
        }
    }

    /// Filesystem path where the GPU trace document will be captured.
    ///
    /// Must be specified when destination is `MTLCaptureDestination::GPUTraceDocument`.
    pub fn output_path(&self) -> Option<PathBuf> {
        let output_url: Option<Retained<NSURL>> = unsafe { msg_send![self, outputURL] };
        output_url.and_then(|url| url.to_file_path())
    }

    pub fn set_output_path(
        &self,
        output_path: Option<&Path>,
    ) {
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
