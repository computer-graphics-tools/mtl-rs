use objc2::{Message, extern_protocol, msg_send, rc::Retained, runtime::ProtocolObject};
use objc2_foundation::NSString;

use crate::{
    MTL4BinaryFunction, MTL4PipelineDescriptor, MTL4RenderPipelineBinaryFunctionsDescriptor, MTLAllocation, MTLDevice,
    MTLFunction, MTLFunctionHandle, MTLIntersectionFunctionTable, MTLIntersectionFunctionTableDescriptor,
    MTLRenderPipelineFunctionsDescriptor, MTLRenderPipelineReflection, MTLRenderStages, MTLResourceID,
    MTLShaderValidation, MTLSize, MTLVisibleFunctionTable, MTLVisibleFunctionTableDescriptor, MetalError,
};

extern_protocol!(
    /// A compiled render pipeline.
    ///
    /// # Safety
    ///
    /// Implementors must be valid Objective-C objects that conform to the
    /// `MTLRenderPipelineState` protocol.
    #[expect(
        clippy::missing_safety_doc,
        reason = "extern_protocol does not attach this safety section to its generated unsafe trait"
    )]
    ///
    /// # Thread safety
    ///
    /// [Apple's declaration](https://developer.apple.com/documentation/metal/mtlrenderpipelinestate):
    ///
    /// > `protocol MTLRenderPipelineState : MTLAllocation, Sendable`
    ///
    /// The `Send` and `Sync` bounds rely on this guarantee.
    pub unsafe trait MTLRenderPipelineState: MTLAllocation + Send + Sync {
        /// The device that created this pipeline state.
        #[unsafe(method(device))]
        #[unsafe(method_family = none)]
        fn device(&self) -> Retained<ProtocolObject<dyn MTLDevice>>;

        /// Reflection information, if the pipeline was created with reflection enabled.
        #[unsafe(method(reflection))]
        #[unsafe(method_family = none)]
        fn reflection(&self) -> Option<Retained<MTLRenderPipelineReflection>>;

        /// Returns a handle for a binary function linked at a given stage.
        #[unsafe(method(functionHandleWithBinaryFunction:stage:))]
        #[unsafe(method_family = none)]
        fn function_handle_with_binary_function_stage(
            &self,
            function: &ProtocolObject<dyn MTL4BinaryFunction>,
            stage: MTLRenderStages,
        ) -> Option<Retained<ProtocolObject<dyn MTLFunctionHandle>>>;

        /// Creates a descriptor that can specialize this pipeline state.
        #[unsafe(method(newRenderPipelineDescriptorForSpecialization))]
        #[unsafe(method_family = new)]
        fn new_render_pipeline_descriptor_for_specialization(&self) -> Retained<MTL4PipelineDescriptor>;

        /// The maximum number of threads in a tile-shader threadgroup.
        #[unsafe(method(maxTotalThreadsPerThreadgroup))]
        #[unsafe(method_family = none)]
        fn max_total_threads_per_threadgroup(&self) -> usize;

        /// Whether the tile threadgroup size must match the tile size.
        #[unsafe(method(threadgroupSizeMatchesTileSize))]
        #[unsafe(method_family = none)]
        fn threadgroup_size_matches_tile_size(&self) -> bool;

        /// Imageblock memory length used by one sample.
        #[unsafe(method(imageblockSampleLength))]
        #[unsafe(method_family = none)]
        fn imageblock_sample_length(&self) -> usize;

        /// Returns the imageblock memory length for the given dimensions.
        #[unsafe(method(imageblockMemoryLengthForDimensions:))]
        #[unsafe(method_family = none)]
        fn imageblock_memory_length_for_dimensions(
            &self,
            imageblock_dimensions: MTLSize,
        ) -> usize;

        /// Whether the pipeline supports indirect command buffers.
        #[unsafe(method(supportIndirectCommandBuffers))]
        #[unsafe(method_family = none)]
        fn support_indirect_command_buffers(&self) -> bool;

        /// The maximum number of threads in an object-shader threadgroup.
        #[unsafe(method(maxTotalThreadsPerObjectThreadgroup))]
        #[unsafe(method_family = none)]
        fn max_total_threads_per_object_threadgroup(&self) -> usize;

        /// The maximum number of threads in a mesh-shader threadgroup.
        #[unsafe(method(maxTotalThreadsPerMeshThreadgroup))]
        #[unsafe(method_family = none)]
        fn max_total_threads_per_mesh_threadgroup(&self) -> usize;

        /// The SIMD-group width of the object shader.
        #[unsafe(method(objectThreadExecutionWidth))]
        #[unsafe(method_family = none)]
        fn object_thread_execution_width(&self) -> usize;

        /// The SIMD-group width of the mesh shader.
        #[unsafe(method(meshThreadExecutionWidth))]
        #[unsafe(method_family = none)]
        fn mesh_thread_execution_width(&self) -> usize;

        /// The maximum threadgroup count in a mesh grid.
        #[unsafe(method(maxTotalThreadgroupsPerMeshGrid))]
        #[unsafe(method_family = none)]
        fn max_total_threadgroups_per_mesh_grid(&self) -> usize;

        /// The GPU resource identifier for this pipeline state.
        #[unsafe(method(gpuResourceID))]
        #[unsafe(method_family = none)]
        fn gpu_resource_id(&self) -> MTLResourceID;

        /// Returns a handle for a function linked at the given stage.
        #[unsafe(method(functionHandleWithFunction:stage:))]
        #[unsafe(method_family = none)]
        fn function_handle_with_function_stage(
            &self,
            function: &ProtocolObject<dyn MTLFunction>,
            stage: MTLRenderStages,
        ) -> Option<Retained<ProtocolObject<dyn MTLFunctionHandle>>>;

        /// Creates a visible-function table for one pipeline stage.
        #[unsafe(method(newVisibleFunctionTableWithDescriptor:stage:))]
        #[unsafe(method_family = new)]
        fn new_visible_function_table_with_descriptor_stage(
            &self,
            descriptor: &MTLVisibleFunctionTableDescriptor,
            stage: MTLRenderStages,
        ) -> Option<Retained<ProtocolObject<dyn MTLVisibleFunctionTable>>>;

        /// Creates an intersection-function table for one pipeline stage.
        #[unsafe(method(newIntersectionFunctionTableWithDescriptor:stage:))]
        #[unsafe(method_family = new)]
        fn new_intersection_function_table_with_descriptor_stage(
            &self,
            descriptor: &MTLIntersectionFunctionTableDescriptor,
            stage: MTLRenderStages,
        ) -> Option<Retained<ProtocolObject<dyn MTLIntersectionFunctionTable>>>;

        /// The shader-validation mode used to create this pipeline.
        #[unsafe(method(shaderValidation))]
        #[unsafe(method_family = none)]
        fn shader_validation(&self) -> MTLShaderValidation;

        /// The required tile-shader threadgroup size.
        #[unsafe(method(requiredThreadsPerTileThreadgroup))]
        #[unsafe(method_family = none)]
        fn required_threads_per_tile_threadgroup(&self) -> MTLSize;

        /// The required object-shader threadgroup size.
        #[unsafe(method(requiredThreadsPerObjectThreadgroup))]
        #[unsafe(method_family = none)]
        fn required_threads_per_object_threadgroup(&self) -> MTLSize;

        /// The required mesh-shader threadgroup size.
        #[unsafe(method(requiredThreadsPerMeshThreadgroup))]
        #[unsafe(method_family = none)]
        fn required_threads_per_mesh_threadgroup(&self) -> MTLSize;
    }
);

/// Rust-native string accessors for render pipeline states.
pub trait MTLRenderPipelineStateExt: MTLRenderPipelineState + Message {
    /// The optional label assigned to this pipeline state.
    fn label(&self) -> Option<String>;

    /// Returns a handle for a statically linked function with a given name and stage.
    fn function_handle_with_name_stage(
        &self,
        name: &str,
        stage: MTLRenderStages,
    ) -> Option<Retained<ProtocolObject<dyn MTLFunctionHandle>>>;

    /// Creates a pipeline state by adding MTL4 binary functions.
    fn new_render_pipeline_state_with_binary_functions(
        &self,
        binary_functions_descriptor: &MTL4RenderPipelineBinaryFunctionsDescriptor,
    ) -> Result<Retained<ProtocolObject<dyn MTLRenderPipelineState>>, MetalError>;

    /// Creates a pipeline state by adding legacy binary functions.
    fn new_render_pipeline_state_with_additional_binary_functions(
        &self,
        additional_binary_functions: &MTLRenderPipelineFunctionsDescriptor,
    ) -> Result<Retained<ProtocolObject<dyn MTLRenderPipelineState>>, MetalError>;
}

impl MTLRenderPipelineStateExt for ProtocolObject<dyn MTLRenderPipelineState> {
    fn label(&self) -> Option<String> {
        let label: Option<Retained<NSString>> = unsafe { msg_send![self, label] };
        label.map(|label| label.to_string())
    }

    fn function_handle_with_name_stage(
        &self,
        name: &str,
        stage: MTLRenderStages,
    ) -> Option<Retained<ProtocolObject<dyn MTLFunctionHandle>>> {
        let name = NSString::from_str(name);
        unsafe { msg_send![self, functionHandleWithName: &*name, stage: stage] }
    }

    fn new_render_pipeline_state_with_binary_functions(
        &self,
        binary_functions_descriptor: &MTL4RenderPipelineBinaryFunctionsDescriptor,
    ) -> Result<Retained<ProtocolObject<dyn MTLRenderPipelineState>>, MetalError> {
        unsafe { msg_send![self, newRenderPipelineStateWithBinaryFunctions: binary_functions_descriptor, error: _] }
            .map_err(MetalError::from_nserror)
    }

    fn new_render_pipeline_state_with_additional_binary_functions(
        &self,
        additional_binary_functions: &MTLRenderPipelineFunctionsDescriptor,
    ) -> Result<Retained<ProtocolObject<dyn MTLRenderPipelineState>>, MetalError> {
        unsafe {
            msg_send![
                self,
                newRenderPipelineStateWithAdditionalBinaryFunctions: additional_binary_functions,
                error: _
            ]
        }
        .map_err(MetalError::from_nserror)
    }
}
