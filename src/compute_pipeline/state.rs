use objc2::{Message, extern_protocol, msg_send, rc::Retained, runtime::ProtocolObject};
use objc2_foundation::{NSArray, NSString};

use crate::{
    MTL4BinaryFunction, MTLAllocation, MTLDevice, MTLFunction, MTLFunctionHandle, MTLIntersectionFunctionTable,
    MTLIntersectionFunctionTableDescriptor, MTLResourceID, MTLShaderValidation, MTLSize, MTLVisibleFunctionTable,
    MTLVisibleFunctionTableDescriptor, MetalError,
};

extern_protocol!(
    /// A handle to compiled code for a compute function.
    ///
    /// # Safety
    ///
    /// Implementors must be valid Objective-C objects that conform to the
    /// `MTLComputePipelineState` protocol.
    #[expect(
        clippy::missing_safety_doc,
        reason = "extern_protocol does not attach this safety section to its generated unsafe trait"
    )]
    ///
    /// # Thread safety
    ///
    /// [Apple's declaration](https://developer.apple.com/documentation/metal/mtlcomputepipelinestate):
    ///
    /// > `protocol MTLComputePipelineState : MTLAllocation, Sendable`
    ///
    /// The `Send` and `Sync` bounds rely on this guarantee.
    pub unsafe trait MTLComputePipelineState: MTLAllocation + Send + Sync {
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
        fn gpu_resource_id(&self) -> MTLResourceID;

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
        ) -> Option<Retained<ProtocolObject<dyn MTLIntersectionFunctionTable>>>;

        /// Returns a handle for a binary function linked into this pipeline.
        #[unsafe(method(functionHandleWithBinaryFunction:))]
        #[unsafe(method_family = none)]
        fn function_handle_with_binary_function(
            &self,
            function: &ProtocolObject<dyn MTL4BinaryFunction>,
        ) -> Option<Retained<ProtocolObject<dyn MTLFunctionHandle>>>;

        /// Returns the length, in bytes, of imageblock memory for the given dimensions.
        #[unsafe(method(imageblockMemoryLengthForDimensions:))]
        #[unsafe(method_family = none)]
        fn imageblock_memory_length_for_dimensions(
            &self,
            imageblock_dimensions: MTLSize,
        ) -> usize;

        /// Whether this pipeline state can be used through an indirect command buffer.
        #[unsafe(method(supportIndirectCommandBuffers))]
        #[unsafe(method_family = none)]
        fn support_indirect_command_buffers(&self) -> bool;

        /// Returns a handle for a function linked into this pipeline.
        #[unsafe(method(functionHandleWithFunction:))]
        #[unsafe(method_family = none)]
        fn function_handle_with_function(
            &self,
            function: &ProtocolObject<dyn MTLFunction>,
        ) -> Option<Retained<ProtocolObject<dyn MTLFunctionHandle>>>;

        /// The shader-validation mode used to create this pipeline.
        #[unsafe(method(shaderValidation))]
        #[unsafe(method_family = none)]
        fn shader_validation(&self) -> MTLShaderValidation;

        /// The required size of every compute threadgroup.
        #[unsafe(method(requiredThreadsPerThreadgroup))]
        #[unsafe(method_family = none)]
        fn required_threads_per_threadgroup(&self) -> MTLSize;
    }
);

pub trait MTLComputePipelineStateExt: MTLComputePipelineState + Message {
    fn label(&self) -> Option<String>;

    /// Returns a handle to the named function in this pipeline, for use with function tables.
    fn function_handle_with_name(
        &self,
        name: &str,
    ) -> Option<Retained<ProtocolObject<dyn MTLFunctionHandle>>>;

    /// Creates a pipeline state by adding MTL4 binary functions.
    fn new_compute_pipeline_state_with_binary_functions(
        &self,
        additional_binary_functions: &[&ProtocolObject<dyn MTL4BinaryFunction>],
    ) -> Result<Retained<ProtocolObject<dyn MTLComputePipelineState>>, MetalError>;

    /// Creates a pipeline state by adding legacy binary functions.
    fn new_compute_pipeline_state_with_additional_binary_functions(
        &self,
        functions: &[&ProtocolObject<dyn MTLFunction>],
    ) -> Result<Retained<ProtocolObject<dyn MTLComputePipelineState>>, MetalError>;
}

impl MTLComputePipelineStateExt for ProtocolObject<dyn MTLComputePipelineState> {
    fn label(&self) -> Option<String> {
        let label: Option<Retained<NSString>> = unsafe { msg_send![self, label] };
        label.map(|s| s.to_string())
    }

    fn function_handle_with_name(
        &self,
        name: &str,
    ) -> Option<Retained<ProtocolObject<dyn MTLFunctionHandle>>> {
        let name = NSString::from_str(name);
        unsafe { msg_send![self, functionHandleWithName: &*name] }
    }

    fn new_compute_pipeline_state_with_binary_functions(
        &self,
        additional_binary_functions: &[&ProtocolObject<dyn MTL4BinaryFunction>],
    ) -> Result<Retained<ProtocolObject<dyn MTLComputePipelineState>>, MetalError> {
        let additional_binary_functions = NSArray::from_slice(additional_binary_functions);
        unsafe {
            msg_send![
                self,
                newComputePipelineStateWithBinaryFunctions: &*additional_binary_functions,
                error: _
            ]
        }
        .map_err(MetalError::from_nserror)
    }

    fn new_compute_pipeline_state_with_additional_binary_functions(
        &self,
        functions: &[&ProtocolObject<dyn MTLFunction>],
    ) -> Result<Retained<ProtocolObject<dyn MTLComputePipelineState>>, MetalError> {
        let functions = NSArray::from_slice(functions);
        unsafe {
            msg_send![
                self,
                newComputePipelineStateWithAdditionalBinaryFunctions: &*functions,
                error: _
            ]
        }
        .map_err(MetalError::from_nserror)
    }
}
