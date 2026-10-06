use objc2::{extern_class, extern_conformance, msg_send, rc::Retained, runtime::NSObject};
use objc2_foundation::NSObjectProtocol;

use super::MTLPipelineBufferDescriptor;

extern_class!(
    /// Apple's docs: `https://developer.apple.com/documentation/metal/mtlpipelinebufferdescriptorarray?language=objc`
    ///
    /// Availability: macOS 10.13+, iOS 11.0+
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLPipelineBufferDescriptorArray;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLPipelineBufferDescriptorArray {}
);

impl MTLPipelineBufferDescriptorArray {
    /// Returns the buffer descriptor at `buffer_index` for individual buffer descriptor access.
    pub fn get(
        &self,
        buffer_index: usize,
    ) -> Retained<MTLPipelineBufferDescriptor> {
        unsafe { msg_send![self, objectAtIndexedSubscript: buffer_index] }
    }
    /// Sets the buffer descriptor at `buffer_index`.
    ///
    /// This always uses copy semantics. It is safe to pass `None` at any legal index, which resets that buffer
    /// descriptor to default values.
    pub fn set(
        &self,
        buffer_index: usize,
        buffer: Option<&MTLPipelineBufferDescriptor>,
    ) {
        unsafe {
            let _: () = msg_send![self, setObject: buffer, atIndexedSubscript: buffer_index];
        }
    }
}
