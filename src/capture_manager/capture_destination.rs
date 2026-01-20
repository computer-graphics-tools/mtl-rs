use objc2::{Encode, Encoding, RefEncode};

/// The destination where you want the GPU trace to be captured to.
///
/// Availability: macOS 10.15+, iOS 13.0+
#[repr(i64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MTLCaptureDestination {
    /// Capture to Developer Tools (Xcode) and stop the execution after capturing.
    DeveloperTools = 1,
    /// Capture to a GPU Trace document and continue execution after capturing.
    GPUTraceDocument = 2,
}

unsafe impl Encode for MTLCaptureDestination {
    const ENCODING: Encoding = i64::ENCODING;
}
unsafe impl RefEncode for MTLCaptureDestination {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
