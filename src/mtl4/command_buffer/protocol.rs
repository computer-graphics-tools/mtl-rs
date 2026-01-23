use core::ops::Range;

use objc2::{Message, extern_protocol, msg_send, rc::Retained, runtime::ProtocolObject};
use objc2_foundation::{NSObjectProtocol, NSRange, NSString, NSUInteger};

use crate::*;

extern_protocol!(
    /// Records a sequence of GPU commands.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metal/mtl4commandbuffer?language=objc)
    pub unsafe trait MTL4CommandBuffer: NSObjectProtocol {
        /// Returns the GPU device that this command buffer belongs to.
        #[unsafe(method(device))]
        #[unsafe(method_family = none)]
        fn device(&self) -> Retained<ProtocolObject<dyn MTLDevice>>;

        /// Prepares a command buffer for encoding.
        #[unsafe(method(beginCommandBufferWithAllocator:))]
        #[unsafe(method_family = none)]
        fn begin_command_buffer_with_allocator(
            &self,
            allocator: &ProtocolObject<dyn MTL4CommandAllocator>,
        );

        /// Prepares a command buffer for encoding with additional options.
        #[unsafe(method(beginCommandBufferWithAllocator:options:))]
        #[unsafe(method_family = none)]
        fn begin_command_buffer_with_allocator_options(
            &self,
            allocator: &ProtocolObject<dyn MTL4CommandAllocator>,
            options: &MTL4CommandBufferOptions,
        );

        /// Closes a command buffer to prepare it for submission to a command queue.
        #[unsafe(method(endCommandBuffer))]
        #[unsafe(method_family = none)]
        fn end_command_buffer(&self);

        /// Creates a render command encoder from a render pass descriptor.
        #[unsafe(method(renderCommandEncoderWithDescriptor:))]
        #[unsafe(method_family = none)]
        fn render_command_encoder_with_descriptor(
            &self,
            descriptor: &MTL4RenderPassDescriptor,
        ) -> Option<Retained<ProtocolObject<dyn MTL4RenderCommandEncoder>>>;

        /// Creates a render command encoder from a render pass descriptor with additional options.
        #[unsafe(method(renderCommandEncoderWithDescriptor:options:))]
        #[unsafe(method_family = none)]
        fn render_command_encoder_with_descriptor_options(
            &self,
            descriptor: &MTL4RenderPassDescriptor,
            options: MTL4RenderEncoderOptions,
        ) -> Option<Retained<ProtocolObject<dyn MTL4RenderCommandEncoder>>>;

        /// Creates a compute command encoder.
        #[unsafe(method(computeCommandEncoder))]
        #[unsafe(method_family = none)]
        fn compute_command_encoder(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn MTL4ComputeCommandEncoder>>>;

        /// Creates a machine learning command encoder.
        #[unsafe(method(machineLearningCommandEncoder))]
        #[unsafe(method_family = none)]
        fn machine_learning_command_encoder(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn MTL4MachineLearningCommandEncoder>>>;

        /// Marks a residency set as part of the command buffer's execution.
        #[unsafe(method(useResidencySet:))]
        #[unsafe(method_family = none)]
        fn use_residency_set(&self, residency_set: &ProtocolObject<dyn MTLResidencySet>);

        /// Marks an array of residency sets as part of the command buffer's execution.
        ///
        /// # Safety
        ///
        /// `residency_sets` must be a valid pointer.
        #[unsafe(method(useResidencySets:count:))]
        #[unsafe(method_family = none)]
        fn use_residency_sets_count(
            &self,
            residency_sets: core::ptr::NonNull<
                core::ptr::NonNull<ProtocolObject<dyn MTLResidencySet>>,
            >,
            count: NSUInteger,
        );

        /// Pops the latest string from the stack of debug groups for this command buffer.
        #[unsafe(method(popDebugGroup))]
        #[unsafe(method_family = none)]
        fn pop_debug_group(&self);

        /// Writes a GPU timestamp into the given counter heap.
        #[unsafe(method(writeTimestampIntoHeap:atIndex:))]
        #[unsafe(method_family = none)]
        fn write_timestamp_into_heap_at_index(
            &self,
            counter_heap: &ProtocolObject<dyn MTL4CounterHeap>,
            index: NSUInteger,
        );

        /// Encodes a command that resolves an opaque counter heap into a buffer.
        #[unsafe(method(resolveCounterHeap:withRange:intoBuffer:waitFence:updateFence:))]
        #[unsafe(method_family = none)]
        fn resolve_counter_heap_with_range_into_buffer_wait_fence_update_fence(
            &self,
            counter_heap: &ProtocolObject<dyn MTL4CounterHeap>,
            range: NSRange,
            buffer_range: MTL4BufferRange,
            fence_to_wait: Option<&ProtocolObject<dyn MTLFence>>,
            fence_to_update: Option<&ProtocolObject<dyn MTLFence>>,
        );
    }
);

#[allow(unused)]
pub trait MTL4CommandBufferExt: MTL4CommandBuffer + Message {
    /// Optional label.
    fn label(&self) -> Option<String>;
    /// Set optional label.
    fn set_label(&self, label: Option<&str>);
    /// Push a debug group with a Rust string.
    fn push_debug_group(&self, label: &str);
    /// Convenience: index as usize.
    fn write_timestamp_into_heap_at_index_usize(
        &self,
        counter_heap: &ProtocolObject<dyn MTL4CounterHeap>,
        index: usize,
    );
    /// Resolve counter heap using a Rust Range<usize>.
    fn resolve_counter_heap_into_buffer_with_range(
        &self,
        counter_heap: &ProtocolObject<dyn MTL4CounterHeap>,
        range: Range<usize>,
        buffer_range: MTL4BufferRange,
        fence_to_wait: Option<&ProtocolObject<dyn MTLFence>>,
        fence_to_update: Option<&ProtocolObject<dyn MTLFence>>,
    );
}

impl MTL4CommandBufferExt for ProtocolObject<dyn MTL4CommandBuffer> {
    fn label(&self) -> Option<String> {
        let s: Option<Retained<NSString>> = unsafe { msg_send![self, label] };
        s.map(|v| v.to_string())
    }

    fn set_label(&self, label: Option<&str>) {
        unsafe {
            let _: () = msg_send![self, setLabel: label.map(NSString::from_str).as_deref()];
        }
    }

    fn push_debug_group(&self, label: &str) {
        unsafe {
            let _: () = msg_send![self, pushDebugGroup: &*NSString::from_str(label)];
        }
    }

    fn write_timestamp_into_heap_at_index_usize(
        &self,
        counter_heap: &ProtocolObject<dyn MTL4CounterHeap>,
        index: usize,
    ) {
        unsafe {
            let _: () = msg_send![self, writeTimestampIntoHeap: counter_heap, atIndex: index];
        }
    }

    fn resolve_counter_heap_into_buffer_with_range(
        &self,
        counter_heap: &ProtocolObject<dyn MTL4CounterHeap>,
        range: Range<usize>,
        buffer_range: MTL4BufferRange,
        fence_to_wait: Option<&ProtocolObject<dyn MTLFence>>,
        fence_to_update: Option<&ProtocolObject<dyn MTLFence>>,
    ) {
        let ns_range = NSRange {
            location: range.start,
            length: range.end.saturating_sub(range.start),
        };
        unsafe {
            let _: () = msg_send![
                self,
                resolveCounterHeap: counter_heap,
                withRange: ns_range,
                intoBuffer: buffer_range,
                waitFence: fence_to_wait,
                updateFence: fence_to_update
            ];
        }
    }
}
