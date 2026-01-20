use objc2::{Message, extern_protocol, msg_send, rc::Retained, runtime::ProtocolObject};
use objc2_foundation::{NSObjectProtocol, NSString};

use crate::{
    MTLDevice, MTLResourceID,
    MTLIntersectionFunctionTableDescriptor,
    MTLVisibleFunctionTable, MTLVisibleFunctionTableDescriptor,
};

extern_protocol!(
    /// A handle to compiled code for a compute function.
    pub unsafe trait MTLComputePipelineState: NSObjectProtocol + Send + Sync {
        /// The device this resource was created against.
        #[unsafe(method(device))]
        #[unsafe(method_family = none)]
        fn device(&self) -> Retained<ProtocolObject<dyn MTLDevice>>;

        /// The maximum total number of threads that can be in a single compute threadgroup.
        #[unsafe(method(maxTotalThreadsPerThreadgroup))]
        #[unsafe(method_family = none)]
        fn max_total_threads_per_threadgroup(&self) -> usize;

        /// For most efficient execution, the threadgroup size should be a multiple of this.
        #[unsafe(method(threadExecutionWidth))]
        #[unsafe(method_family = none)]
        fn thread_execution_width(&self) -> usize;

        /// The length in bytes of threadgroup memory that is statically allocated.
        #[unsafe(method(staticThreadgroupMemoryLength))]
        #[unsafe(method_family = none)]
        fn static_threadgroup_memory_length(&self) -> usize;

        /// Handle of the GPU resource suitable for storing in an Argument Buffer
        #[unsafe(method(gpuResourceID))]
        #[unsafe(method_family = none)]
        unsafe fn gpu_resource_id(&self) -> MTLResourceID;

        /// Provides access to this compute pipeline's reflection.
        /// Reflection is `None` if you create the pipeline state directly from the `MTLDevice` protocol.
        #[unsafe(method(reflection))]
        #[unsafe(method_family = none)]
        fn reflection(&self) -> Option<Retained<super::MTLComputePipelineReflection>>;

        /// Allocate a visible function table for the pipeline with the provided descriptor.
        #[unsafe(method(newVisibleFunctionTableWithDescriptor:))]
        #[unsafe(method_family = new)]
        fn new_visible_function_table_with_descriptor(
            &self,
            descriptor: &MTLVisibleFunctionTableDescriptor,
        ) -> Option<Retained<ProtocolObject<dyn MTLVisibleFunctionTable>>>;

        /// Allocate an intersection function table for the pipeline with the provided descriptor.
        #[unsafe(method(newIntersectionFunctionTableWithDescriptor:))]
        #[unsafe(method_family = new)]
        fn new_intersection_function_table_with_descriptor(
            &self,
            descriptor: &MTLIntersectionFunctionTableDescriptor,
        ) -> Option<Retained<ProtocolObject<dyn crate::MTLIntersectionFunctionTable>>>;
    }
);

#[allow(unused)]
pub trait MTLComputePipelineStateExt: MTLComputePipelineState + Message {
    fn label(&self) -> Option<String>;
}

impl MTLComputePipelineStateExt for ProtocolObject<dyn MTLComputePipelineState> {
    fn label(&self) -> Option<String> {
        let label: Option<Retained<NSString>> = unsafe { msg_send![self, label] };
        label.map(|s| s.to_string())
    }
}
