use objc2::{
    extern_class, extern_conformance, extern_methods, msg_send,
    rc::{Allocated, Retained},
    runtime::{NSObject, ProtocolObject},
};
use objc2_foundation::{CopyingHelper, NSArray, NSCopying, NSObjectProtocol, NSString};

use crate::{
    MTLBinaryArchive, MTLFunction, MTLLinkedFunctions, MTLPipelineBufferDescriptorArray, MTLPixelFormat,
    MTLRenderPipelineColorAttachmentDescriptorArray, MTLShaderValidation, MTLSize,
};

extern_class!(
    /// Describes a render pipeline that uses object, mesh, and fragment shader stages.
    ///
    /// Availability: macOS 13.0+, iOS 16.0+
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLMeshRenderPipelineDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for MTLMeshRenderPipelineDescriptor {}
);

unsafe impl CopyingHelper for MTLMeshRenderPipelineDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLMeshRenderPipelineDescriptor {}
);

impl MTLMeshRenderPipelineDescriptor {
    extern_methods!(
        /// The optional object shader function.
        #[unsafe(method(objectFunction))]
        #[unsafe(method_family = none)]
        pub fn object_function(&self) -> Option<Retained<ProtocolObject<dyn MTLFunction>>>;

        /// Setter for [`object_function`][Self::object_function].
        #[unsafe(method(setObjectFunction:))]
        #[unsafe(method_family = none)]
        pub fn set_object_function(
            &self,
            function: Option<&ProtocolObject<dyn MTLFunction>>,
        );

        /// The optional mesh shader function.
        #[unsafe(method(meshFunction))]
        #[unsafe(method_family = none)]
        pub fn mesh_function(&self) -> Option<Retained<ProtocolObject<dyn MTLFunction>>>;

        /// Setter for [`mesh_function`][Self::mesh_function].
        #[unsafe(method(setMeshFunction:))]
        #[unsafe(method_family = none)]
        pub fn set_mesh_function(
            &self,
            function: Option<&ProtocolObject<dyn MTLFunction>>,
        );

        /// The optional fragment shader function.
        #[unsafe(method(fragmentFunction))]
        #[unsafe(method_family = none)]
        pub fn fragment_function(&self) -> Option<Retained<ProtocolObject<dyn MTLFunction>>>;

        /// Setter for [`fragment_function`][Self::fragment_function].
        #[unsafe(method(setFragmentFunction:))]
        #[unsafe(method_family = none)]
        pub fn set_fragment_function(
            &self,
            function: Option<&ProtocolObject<dyn MTLFunction>>,
        );

        /// The maximum total threads in an object-shader threadgroup.
        #[unsafe(method(maxTotalThreadsPerObjectThreadgroup))]
        #[unsafe(method_family = none)]
        pub fn max_total_threads_per_object_threadgroup(&self) -> usize;

        /// Setter for [`max_total_threads_per_object_threadgroup`][Self::max_total_threads_per_object_threadgroup].
        #[unsafe(method(setMaxTotalThreadsPerObjectThreadgroup:))]
        #[unsafe(method_family = none)]
        pub fn set_max_total_threads_per_object_threadgroup(
            &self,
            value: usize,
        );

        /// The maximum total threads in a mesh-shader threadgroup.
        #[unsafe(method(maxTotalThreadsPerMeshThreadgroup))]
        #[unsafe(method_family = none)]
        pub fn max_total_threads_per_mesh_threadgroup(&self) -> usize;

        /// Setter for [`max_total_threads_per_mesh_threadgroup`][Self::max_total_threads_per_mesh_threadgroup].
        #[unsafe(method(setMaxTotalThreadsPerMeshThreadgroup:))]
        #[unsafe(method_family = none)]
        pub fn set_max_total_threads_per_mesh_threadgroup(
            &self,
            value: usize,
        );

        /// Whether object threadgroup sizes are multiples of the object SIMD width.
        #[unsafe(method(objectThreadgroupSizeIsMultipleOfThreadExecutionWidth))]
        #[unsafe(method_family = none)]
        pub fn object_threadgroup_size_is_multiple_of_thread_execution_width(&self) -> bool;

        /// Setter for [`object_threadgroup_size_is_multiple_of_thread_execution_width`][Self::object_threadgroup_size_is_multiple_of_thread_execution_width].
        #[unsafe(method(setObjectThreadgroupSizeIsMultipleOfThreadExecutionWidth:))]
        #[unsafe(method_family = none)]
        pub fn set_object_threadgroup_size_is_multiple_of_thread_execution_width(
            &self,
            value: bool,
        );

        /// Whether mesh threadgroup sizes are multiples of the mesh SIMD width.
        #[unsafe(method(meshThreadgroupSizeIsMultipleOfThreadExecutionWidth))]
        #[unsafe(method_family = none)]
        pub fn mesh_threadgroup_size_is_multiple_of_thread_execution_width(&self) -> bool;

        /// Setter for [`mesh_threadgroup_size_is_multiple_of_thread_execution_width`][Self::mesh_threadgroup_size_is_multiple_of_thread_execution_width].
        #[unsafe(method(setMeshThreadgroupSizeIsMultipleOfThreadExecutionWidth:))]
        #[unsafe(method_family = none)]
        pub fn set_mesh_threadgroup_size_is_multiple_of_thread_execution_width(
            &self,
            value: bool,
        );

        /// The size, in bytes, of the object-to-mesh payload buffer.
        #[unsafe(method(payloadMemoryLength))]
        #[unsafe(method_family = none)]
        pub fn payload_memory_length(&self) -> usize;

        /// Setter for [`payload_memory_length`][Self::payload_memory_length].
        #[unsafe(method(setPayloadMemoryLength:))]
        #[unsafe(method_family = none)]
        pub fn set_payload_memory_length(
            &self,
            value: usize,
        );

        /// The maximum total threadgroups in a mesh grid.
        #[unsafe(method(maxTotalThreadgroupsPerMeshGrid))]
        #[unsafe(method_family = none)]
        pub fn max_total_threadgroups_per_mesh_grid(&self) -> usize;

        /// Setter for [`max_total_threadgroups_per_mesh_grid`][Self::max_total_threadgroups_per_mesh_grid].
        #[unsafe(method(setMaxTotalThreadgroupsPerMeshGrid:))]
        #[unsafe(method_family = none)]
        pub fn set_max_total_threadgroups_per_mesh_grid(
            &self,
            value: usize,
        );

        /// Buffer mutability descriptors for the object stage.
        #[unsafe(method(objectBuffers))]
        #[unsafe(method_family = none)]
        pub fn object_buffers(&self) -> Retained<MTLPipelineBufferDescriptorArray>;

        /// Buffer mutability descriptors for the mesh stage.
        #[unsafe(method(meshBuffers))]
        #[unsafe(method_family = none)]
        pub fn mesh_buffers(&self) -> Retained<MTLPipelineBufferDescriptorArray>;

        /// Buffer mutability descriptors for the fragment stage.
        #[unsafe(method(fragmentBuffers))]
        #[unsafe(method_family = none)]
        pub fn fragment_buffers(&self) -> Retained<MTLPipelineBufferDescriptorArray>;

        /// The number of samples for each fragment.
        #[unsafe(method(rasterSampleCount))]
        #[unsafe(method_family = none)]
        pub fn raster_sample_count(&self) -> usize;

        /// Setter for [`raster_sample_count`][Self::raster_sample_count].
        #[unsafe(method(setRasterSampleCount:))]
        #[unsafe(method_family = none)]
        pub fn set_raster_sample_count(
            &self,
            value: usize,
        );

        /// Whether fragment alpha is converted to a sample mask.
        #[unsafe(method(isAlphaToCoverageEnabled))]
        #[unsafe(method_family = none)]
        pub fn is_alpha_to_coverage_enabled(&self) -> bool;

        /// Setter for [`is_alpha_to_coverage_enabled`][Self::is_alpha_to_coverage_enabled].
        #[unsafe(method(setAlphaToCoverageEnabled:))]
        #[unsafe(method_family = none)]
        pub fn set_alpha_to_coverage_enabled(
            &self,
            enabled: bool,
        );

        /// Whether fragment alpha is forced to one.
        #[unsafe(method(isAlphaToOneEnabled))]
        #[unsafe(method_family = none)]
        pub fn is_alpha_to_one_enabled(&self) -> bool;

        /// Setter for [`is_alpha_to_one_enabled`][Self::is_alpha_to_one_enabled].
        #[unsafe(method(setAlphaToOneEnabled:))]
        #[unsafe(method_family = none)]
        pub fn set_alpha_to_one_enabled(
            &self,
            enabled: bool,
        );

        /// Whether the pipeline rasterizes primitives.
        #[unsafe(method(isRasterizationEnabled))]
        #[unsafe(method_family = none)]
        pub fn is_rasterization_enabled(&self) -> bool;

        /// Setter for [`is_rasterization_enabled`][Self::is_rasterization_enabled].
        #[unsafe(method(setRasterizationEnabled:))]
        #[unsafe(method_family = none)]
        pub fn set_rasterization_enabled(
            &self,
            enabled: bool,
        );

        /// The maximum vertex amplification count.
        #[unsafe(method(maxVertexAmplificationCount))]
        #[unsafe(method_family = none)]
        pub fn max_vertex_amplification_count(&self) -> usize;

        /// Setter for [`max_vertex_amplification_count`][Self::max_vertex_amplification_count].
        #[unsafe(method(setMaxVertexAmplificationCount:))]
        #[unsafe(method_family = none)]
        pub fn set_max_vertex_amplification_count(
            &self,
            value: usize,
        );

        /// Color attachment descriptors for the render pass.
        #[unsafe(method(colorAttachments))]
        #[unsafe(method_family = none)]
        pub fn color_attachments(&self) -> Retained<MTLRenderPipelineColorAttachmentDescriptorArray>;

        /// The depth attachment pixel format.
        #[unsafe(method(depthAttachmentPixelFormat))]
        #[unsafe(method_family = none)]
        pub fn depth_attachment_pixel_format(&self) -> MTLPixelFormat;

        /// Setter for [`depth_attachment_pixel_format`][Self::depth_attachment_pixel_format].
        #[unsafe(method(setDepthAttachmentPixelFormat:))]
        #[unsafe(method_family = none)]
        pub fn set_depth_attachment_pixel_format(
            &self,
            format: MTLPixelFormat,
        );

        /// The stencil attachment pixel format.
        #[unsafe(method(stencilAttachmentPixelFormat))]
        #[unsafe(method_family = none)]
        pub fn stencil_attachment_pixel_format(&self) -> MTLPixelFormat;

        /// Setter for [`stencil_attachment_pixel_format`][Self::stencil_attachment_pixel_format].
        #[unsafe(method(setStencilAttachmentPixelFormat:))]
        #[unsafe(method_family = none)]
        pub fn set_stencil_attachment_pixel_format(
            &self,
            format: MTLPixelFormat,
        );

        /// Whether the pipeline supports indirect command buffers.
        #[unsafe(method(supportIndirectCommandBuffers))]
        #[unsafe(method_family = none)]
        pub fn support_indirect_command_buffers(&self) -> bool;

        /// Setter for [`support_indirect_command_buffers`][Self::support_indirect_command_buffers].
        #[unsafe(method(setSupportIndirectCommandBuffers:))]
        #[unsafe(method_family = none)]
        pub fn set_support_indirect_command_buffers(
            &self,
            enabled: bool,
        );

        /// Functions linked with the object stage.
        #[unsafe(method(objectLinkedFunctions))]
        #[unsafe(method_family = none)]
        pub fn object_linked_functions(&self) -> Retained<MTLLinkedFunctions>;

        /// Setter for [`object_linked_functions`][Self::object_linked_functions].
        #[unsafe(method(setObjectLinkedFunctions:))]
        #[unsafe(method_family = none)]
        pub fn set_object_linked_functions(
            &self,
            functions: Option<&MTLLinkedFunctions>,
        );

        /// Functions linked with the mesh stage.
        #[unsafe(method(meshLinkedFunctions))]
        #[unsafe(method_family = none)]
        pub fn mesh_linked_functions(&self) -> Retained<MTLLinkedFunctions>;

        /// Setter for [`mesh_linked_functions`][Self::mesh_linked_functions].
        #[unsafe(method(setMeshLinkedFunctions:))]
        #[unsafe(method_family = none)]
        pub fn set_mesh_linked_functions(
            &self,
            functions: Option<&MTLLinkedFunctions>,
        );

        /// Functions linked with the fragment stage.
        #[unsafe(method(fragmentLinkedFunctions))]
        #[unsafe(method_family = none)]
        pub fn fragment_linked_functions(&self) -> Retained<MTLLinkedFunctions>;

        /// Setter for [`fragment_linked_functions`][Self::fragment_linked_functions].
        #[unsafe(method(setFragmentLinkedFunctions:))]
        #[unsafe(method_family = none)]
        pub fn set_fragment_linked_functions(
            &self,
            functions: Option<&MTLLinkedFunctions>,
        );

        /// Restores all properties to their defaults.
        #[unsafe(method(reset))]
        #[unsafe(method_family = none)]
        pub fn reset(&self);

        /// The shader-validation mode.
        #[unsafe(method(shaderValidation))]
        #[unsafe(method_family = none)]
        pub fn shader_validation(&self) -> MTLShaderValidation;

        /// Setter for [`shader_validation`][Self::shader_validation].
        #[unsafe(method(setShaderValidation:))]
        #[unsafe(method_family = none)]
        pub fn set_shader_validation(
            &self,
            validation: MTLShaderValidation,
        );

        /// The required object threads-per-threadgroup size.
        #[unsafe(method(requiredThreadsPerObjectThreadgroup))]
        #[unsafe(method_family = none)]
        pub fn required_threads_per_object_threadgroup(&self) -> MTLSize;

        /// Setter for [`required_threads_per_object_threadgroup`][Self::required_threads_per_object_threadgroup].
        #[unsafe(method(setRequiredThreadsPerObjectThreadgroup:))]
        #[unsafe(method_family = none)]
        pub fn set_required_threads_per_object_threadgroup(
            &self,
            size: MTLSize,
        );

        /// The required mesh threads-per-threadgroup size.
        #[unsafe(method(requiredThreadsPerMeshThreadgroup))]
        #[unsafe(method_family = none)]
        pub fn required_threads_per_mesh_threadgroup(&self) -> MTLSize;

        /// Setter for [`required_threads_per_mesh_threadgroup`][Self::required_threads_per_mesh_threadgroup].
        #[unsafe(method(setRequiredThreadsPerMeshThreadgroup:))]
        #[unsafe(method_family = none)]
        pub fn set_required_threads_per_mesh_threadgroup(
            &self,
            size: MTLSize,
        );
    );

    /// Binary archives to search for compiled pipeline code.
    pub fn binary_archives(&self) -> Option<Box<[Retained<ProtocolObject<dyn MTLBinaryArchive>>]>> {
        let archives: Option<Retained<NSArray<ProtocolObject<dyn MTLBinaryArchive>>>> =
            unsafe { msg_send![self, binaryArchives] };
        archives.map(|archives| archives.to_vec().into_boxed_slice())
    }

    /// Setter for [`binary_archives`][Self::binary_archives].
    pub fn set_binary_archives(
        &self,
        archives: Option<&[&ProtocolObject<dyn MTLBinaryArchive>]>,
    ) {
        let archives = archives.map(NSArray::from_slice);
        unsafe {
            let _: () = msg_send![self, setBinaryArchives: archives.as_deref()];
        }
    }

    /// The optional descriptor label.
    pub fn label(&self) -> Option<String> {
        let label: Option<Retained<NSString>> = unsafe { msg_send![self, label] };
        label.map(|label| label.to_string())
    }

    /// Sets the descriptor label.
    pub fn set_label(
        &self,
        label: Option<&str>,
    ) {
        unsafe {
            let _: () = msg_send![self, setLabel: label.map(NSString::from_str).as_deref()];
        }
    }
}

impl MTLMeshRenderPipelineDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub fn new() -> Retained<Self>;
    );
}
