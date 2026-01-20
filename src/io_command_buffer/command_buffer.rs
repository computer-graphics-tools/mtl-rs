use core::{ffi::c_void, ptr::NonNull};

use objc2::{Message, extern_protocol, msg_send, rc::Retained, runtime::ProtocolObject};
use objc2_foundation::{NSError, NSObjectProtocol, NSString};

use crate::{
    MTLBuffer, MTLIOCommandBufferCompletedHandler, MTLIOFileHandle, MTLIOStatus, MTLOrigin,
    MTLSharedEvent, MTLSize, MTLTexture,
};

extern_protocol!(
    /// Represents a list of IO commands for a queue to execute.
    ///
    /// Availability: macOS 13.0+, iOS 16.0+
    pub unsafe trait MTLIOCommandBuffer: NSObjectProtocol {
        /// Encodes a command that loads from a handle and offset into a memory location.
        ///
        /// Safety: `pointer` must be valid for writes of `size` bytes.
        #[unsafe(method(loadBytes:size:sourceHandle:sourceHandleOffset:))]
        #[unsafe(method_family = none)]
        unsafe fn load_bytes_size_source_handle_source_handle_offset(
            &self,
            pointer: NonNull<c_void>,
            size: usize,
            source_handle: &ProtocolObject<dyn MTLIOFileHandle>,
            source_handle_offset: usize,
        );

        /// Encodes a command that loads from a handle and offset into a buffer and an offset.
        #[unsafe(method(loadBuffer:offset:size:sourceHandle:sourceHandleOffset:))]
        #[unsafe(method_family = none)]
        unsafe fn load_buffer_offset_size_source_handle_source_handle_offset(
            &self,
            buffer: &ProtocolObject<dyn MTLBuffer>,
            offset: usize,
            size: usize,
            source_handle: &ProtocolObject<dyn MTLIOFileHandle>,
            source_handle_offset: usize,
        );

        /// Encodes a command that loads a region from a handle into a texture.
        #[unsafe(method(loadTexture:slice:level:size:sourceBytesPerRow:sourceBytesPerImage:destinationOrigin:sourceHandle:sourceHandleOffset:))]
        #[unsafe(method_family = none)]
        unsafe fn load_texture_slice_level_size_source_bytes_per_row_source_bytes_per_image_destination_origin_source_handle_source_handle_offset(
            &self,
            texture: &ProtocolObject<dyn MTLTexture>,
            slice: usize,
            level: usize,
            size: MTLSize,
            source_bytes_per_row: usize,
            source_bytes_per_image: usize,
            destination_origin: MTLOrigin,
            source_handle: &ProtocolObject<dyn MTLIOFileHandle>,
            source_handle_offset: usize,
        );

        /// Encodes a command that writes the status of this command buffer upon completion to a buffer at a given offset.
        #[unsafe(method(copyStatusToBuffer:offset:))]
        #[unsafe(method_family = none)]
        unsafe fn copy_status_to_buffer_offset(
            &self,
            buffer: &ProtocolObject<dyn MTLBuffer>,
            offset: usize,
        );

        /// Commit so it can be executed as soon as possible.
        #[unsafe(method(commit))]
        #[unsafe(method_family = none)]
        unsafe fn commit(&self);

        /// Synchronously wait for completion.
        #[unsafe(method(waitUntilCompleted))]
        #[unsafe(method_family = none)]
        unsafe fn wait_until_completed(&self);

        /// Request cancellation of an in-flight command buffer.
        #[unsafe(method(tryCancel))]
        #[unsafe(method_family = none)]
        unsafe fn try_cancel(&self);

        /// Add a barrier to order previously encoded commands before subsequent ones start.
        #[unsafe(method(addBarrier))]
        #[unsafe(method_family = none)]
        unsafe fn add_barrier(&self);

        /// Pop the latest named string off of the stack.
        #[unsafe(method(popDebugGroup))]
        #[unsafe(method_family = none)]
        unsafe fn pop_debug_group(&self);

        /// Completion status of the command buffer.
        #[unsafe(method(status))]
        #[unsafe(method_family = none)]
        unsafe fn status(&self) -> MTLIOStatus;

        /// If an error occurred during execution, the NSError may contain more details.
        #[unsafe(method(error))]
        #[unsafe(method_family = none)]
        unsafe fn error(&self) -> Option<Retained<NSError>>;

        /// Append this command buffer to the end of its command queue.
        #[unsafe(method(enqueue))]
        #[unsafe(method_family = none)]
        unsafe fn enqueue(&self);

        /// Pauses execution until the specified shared event reaches a given value.
        #[unsafe(method(waitForEvent:value:))]
        #[unsafe(method_family = none)]
        unsafe fn wait_for_event_value(
            &self,
            event: &ProtocolObject<dyn MTLSharedEvent>,
            value: u64,
        );

        /// Signals a shared event with a given value.
        #[unsafe(method(signalEvent:value:))]
        #[unsafe(method_family = none)]
        unsafe fn signal_event_value(&self, event: &ProtocolObject<dyn MTLSharedEvent>, value: u64);
    }
);

#[allow(unused)]
pub trait MTLIOCommandBufferExt: MTLIOCommandBuffer + Message {
    /// Push a new named string onto a stack of string labels.
    fn push_debug_group(&self, name: &str);
    /// Optional label.
    fn label(&self) -> Option<String>;
    /// Setter for [`label`][Self::label].
    fn set_label(&self, label: Option<&str>);
    /// Add a block to be called when this command buffer has completed execution.
    ///
    /// Availability: macOS 13.0+, iOS 16.0+
    fn add_completed_handler(&self, handler: &MTLIOCommandBufferCompletedHandler);
}

impl MTLIOCommandBufferExt for ProtocolObject<dyn MTLIOCommandBuffer> {
    fn push_debug_group(&self, name: &str) {
        unsafe {
            let _: () = msg_send![self, pushDebugGroup: &*NSString::from_str(name)];
        }
    }

    fn label(&self) -> Option<String> {
        let s: Option<Retained<NSString>> = unsafe { msg_send![self, label] };
        s.map(|v| v.to_string())
    }

    fn set_label(&self, label: Option<&str>) {
        unsafe {
            let _: () = msg_send![self, setLabel: label.map(NSString::from_str).as_deref()];
        }
    }

    fn add_completed_handler(&self, handler: &MTLIOCommandBufferCompletedHandler) {
        unsafe {
            let _: () = msg_send![self, addCompletedHandler: &**handler];
        }
    }
}
