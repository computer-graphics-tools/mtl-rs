use objc2::{
    extern_class, extern_conformance, extern_methods, msg_send,
    rc::{Allocated, Retained},
    runtime::{NSObject, ProtocolObject},
};
use objc2_foundation::{CopyingHelper, NSArray, NSCopying, NSObjectProtocol, NSString};

use crate::{
    MTLBinaryArchive, MTLDynamicLibrary, MTLLinkedFunctions, MTLPipelineBufferDescriptorArray, MTLPixelFormat,
    MTLPrimitiveTopologyClass, MTLRenderPipelineColorAttachmentDescriptorArray, MTLShaderValidation,
    MTLTessellationControlPointIndexType, MTLTessellationFactorFormat, MTLTessellationFactorStepFunction,
    MTLTessellationPartitionMode, MTLVertexDescriptor, MTLWinding, library::MTLFunction,
};

extern_class!(
    /// Descriptor for creating a `RenderPipelineState`.
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLRenderPipelineDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for MTLRenderPipelineDescriptor {}
);

unsafe impl CopyingHelper for MTLRenderPipelineDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLRenderPipelineDescriptor {}
);

impl MTLRenderPipelineDescriptor {
    extern_methods!(
        /// The vertex function for the pipeline.
        #[unsafe(method(vertexFunction))]
        #[unsafe(method_family = none)]
        pub fn vertex_function(&self) -> Option<Retained<ProtocolObject<dyn MTLFunction>>>;

        /// Setter for [`vertex_function`][Self::vertex_function].
        #[unsafe(method(setVertexFunction:))]
        #[unsafe(method_family = none)]
        pub fn set_vertex_function(
            &self,
            function: Option<&ProtocolObject<dyn MTLFunction>>,
        );

        /// The fragment function for the pipeline.
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

        /// The vertex buffer descriptors for the pipeline.
        #[unsafe(method(vertexBuffers))]
        #[unsafe(method_family = none)]
        pub fn vertex_buffers(&self) -> Retained<MTLPipelineBufferDescriptorArray>;

        /// The fragment buffer descriptors for the pipeline.
        #[unsafe(method(fragmentBuffers))]
        #[unsafe(method_family = none)]
        pub fn fragment_buffers(&self) -> Retained<MTLPipelineBufferDescriptorArray>;

        /// The set of functions to be linked with the pipeline and accessed from the vertex function.
        #[unsafe(method(vertexLinkedFunctions))]
        #[unsafe(method_family = none)]
        pub fn vertex_linked_functions(&self) -> Retained<MTLLinkedFunctions>;

        /// Setter for [`vertex_linked_functions`][Self::vertex_linked_functions].
        #[unsafe(method(setVertexLinkedFunctions:))]
        #[unsafe(method_family = none)]
        pub fn set_vertex_linked_functions(
            &self,
            functions: Option<&MTLLinkedFunctions>,
        );

        /// The set of functions to be linked with the pipeline and accessed from the fragment function.
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

        /// Whether the pipeline supports adding binary functions to the vertex stage.
        #[unsafe(method(supportAddingVertexBinaryFunctions))]
        #[unsafe(method_family = none)]
        pub fn support_adding_vertex_binary_functions(&self) -> bool;

        /// Setter for [`support_adding_vertex_binary_functions`][Self::support_adding_vertex_binary_functions].
        #[unsafe(method(setSupportAddingVertexBinaryFunctions:))]
        #[unsafe(method_family = none)]
        pub fn set_support_adding_vertex_binary_functions(
            &self,
            val: bool,
        );

        /// Whether the pipeline supports adding binary functions to the fragment stage.
        #[unsafe(method(supportAddingFragmentBinaryFunctions))]
        #[unsafe(method_family = none)]
        pub fn support_adding_fragment_binary_functions(&self) -> bool;

        /// Setter for [`support_adding_fragment_binary_functions`][Self::support_adding_fragment_binary_functions].
        #[unsafe(method(setSupportAddingFragmentBinaryFunctions:))]
        #[unsafe(method_family = none)]
        pub fn set_support_adding_fragment_binary_functions(
            &self,
            val: bool,
        );

        /// The maximum call stack depth for the vertex function.
        #[unsafe(method(maxVertexCallStackDepth))]
        #[unsafe(method_family = none)]
        pub fn max_vertex_call_stack_depth(&self) -> usize;

        /// Setter for [`max_vertex_call_stack_depth`][Self::max_vertex_call_stack_depth].
        #[unsafe(method(setMaxVertexCallStackDepth:))]
        #[unsafe(method_family = none)]
        pub fn set_max_vertex_call_stack_depth(
            &self,
            depth: usize,
        );

        /// The maximum call stack depth for the fragment function.
        #[unsafe(method(maxFragmentCallStackDepth))]
        #[unsafe(method_family = none)]
        pub fn max_fragment_call_stack_depth(&self) -> usize;

        /// Setter for [`max_fragment_call_stack_depth`][Self::max_fragment_call_stack_depth].
        #[unsafe(method(setMaxFragmentCallStackDepth:))]
        #[unsafe(method_family = none)]
        pub fn set_max_fragment_call_stack_depth(
            &self,
            depth: usize,
        );

        /// The legacy sample count property.
        #[deprecated = "use raster_sample_count"]
        #[unsafe(method(sampleCount))]
        #[unsafe(method_family = none)]
        pub fn sample_count(&self) -> usize;

        /// Setter for [`sample_count`][Self::sample_count].
        #[deprecated = "use set_raster_sample_count"]
        #[unsafe(method(setSampleCount:))]
        #[unsafe(method_family = none)]
        pub fn set_sample_count(
            &self,
            sample_count: usize,
        );

        /// The number of samples for each pixel.
        #[unsafe(method(rasterSampleCount))]
        #[unsafe(method_family = none)]
        pub fn raster_sample_count(&self) -> usize;

        /// Setter for [`raster_sample_count`][Self::raster_sample_count].
        #[unsafe(method(setRasterSampleCount:))]
        #[unsafe(method_family = none)]
        pub fn set_raster_sample_count(
            &self,
            raster_sample_count: usize,
        );

        /// Whether to enable alpha-to-coverage.
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

        /// Whether to enable alpha-to-one.
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

        /// Whether rasterization is enabled.
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

        /// The array of color attachment descriptors.
        #[unsafe(method(colorAttachments))]
        #[unsafe(method_family = none)]
        pub fn color_attachments(&self) -> Retained<MTLRenderPipelineColorAttachmentDescriptorArray>;

        /// The pixel format of the depth attachment.
        #[unsafe(method(depthAttachmentPixelFormat))]
        #[unsafe(method_family = none)]
        pub fn depth_attachment_pixel_format(&self) -> MTLPixelFormat;

        /// Setter for [`depth_attachment_pixel_format`][Self::depth_attachment_pixel_format].
        #[unsafe(method(setDepthAttachmentPixelFormat:))]
        #[unsafe(method_family = none)]
        pub fn set_depth_attachment_pixel_format(
            &self,
            fmt: MTLPixelFormat,
        );

        /// The pixel format of the stencil attachment.
        #[unsafe(method(stencilAttachmentPixelFormat))]
        #[unsafe(method_family = none)]
        pub fn stencil_attachment_pixel_format(&self) -> MTLPixelFormat;

        /// Setter for [`stencil_attachment_pixel_format`][Self::stencil_attachment_pixel_format].
        #[unsafe(method(setStencilAttachmentPixelFormat:))]
        #[unsafe(method_family = none)]
        pub fn set_stencil_attachment_pixel_format(
            &self,
            fmt: MTLPixelFormat,
        );

        /// The input primitive topology.
        #[unsafe(method(inputPrimitiveTopology))]
        #[unsafe(method_family = none)]
        pub fn input_primitive_topology(&self) -> MTLPrimitiveTopologyClass;

        /// Setter for [`input_primitive_topology`][Self::input_primitive_topology].
        #[unsafe(method(setInputPrimitiveTopology:))]
        #[unsafe(method_family = none)]
        pub fn set_input_primitive_topology(
            &self,
            topo: MTLPrimitiveTopologyClass,
        );

        /// The tessellation partition mode.
        #[unsafe(method(tessellationPartitionMode))]
        #[unsafe(method_family = none)]
        pub fn tessellation_partition_mode(&self) -> MTLTessellationPartitionMode;

        /// Setter for [`tessellation_partition_mode`][Self::tessellation_partition_mode].
        #[unsafe(method(setTessellationPartitionMode:))]
        #[unsafe(method_family = none)]
        pub fn set_tessellation_partition_mode(
            &self,
            mode: MTLTessellationPartitionMode,
        );

        /// The maximum tessellation factor.
        #[unsafe(method(maxTessellationFactor))]
        #[unsafe(method_family = none)]
        pub fn max_tessellation_factor(&self) -> usize;

        /// Setter for [`max_tessellation_factor`][Self::max_tessellation_factor].
        #[unsafe(method(setMaxTessellationFactor:))]
        #[unsafe(method_family = none)]
        pub fn set_max_tessellation_factor(
            &self,
            factor: usize,
        );

        /// Whether tessellation factors have a scale value.
        #[unsafe(method(isTessellationFactorScaleEnabled))]
        #[unsafe(method_family = none)]
        pub fn is_tessellation_factor_scale_enabled(&self) -> bool;

        /// Setter for [`is_tessellation_factor_scale_enabled`][Self::is_tessellation_factor_scale_enabled].
        #[unsafe(method(setTessellationFactorScaleEnabled:))]
        #[unsafe(method_family = none)]
        pub fn set_tessellation_factor_scale_enabled(
            &self,
            enabled: bool,
        );

        /// The tessellation factor format.
        #[unsafe(method(tessellationFactorFormat))]
        #[unsafe(method_family = none)]
        pub fn tessellation_factor_format(&self) -> MTLTessellationFactorFormat;

        /// Setter for [`tessellation_factor_format`][Self::tessellation_factor_format].
        #[unsafe(method(setTessellationFactorFormat:))]
        #[unsafe(method_family = none)]
        pub fn set_tessellation_factor_format(
            &self,
            format: MTLTessellationFactorFormat,
        );

        /// The tessellation control-point index type.
        #[unsafe(method(tessellationControlPointIndexType))]
        #[unsafe(method_family = none)]
        pub fn tessellation_control_point_index_type(&self) -> MTLTessellationControlPointIndexType;

        /// Setter for [`tessellation_control_point_index_type`][Self::tessellation_control_point_index_type].
        #[unsafe(method(setTessellationControlPointIndexType:))]
        #[unsafe(method_family = none)]
        pub fn set_tessellation_control_point_index_type(
            &self,
            index_type: MTLTessellationControlPointIndexType,
        );

        /// The tessellation factor step function.
        #[unsafe(method(tessellationFactorStepFunction))]
        #[unsafe(method_family = none)]
        pub fn tessellation_factor_step_function(&self) -> MTLTessellationFactorStepFunction;

        /// Setter for [`tessellation_factor_step_function`][Self::tessellation_factor_step_function].
        #[unsafe(method(setTessellationFactorStepFunction:))]
        #[unsafe(method_family = none)]
        pub fn set_tessellation_factor_step_function(
            &self,
            step_function: MTLTessellationFactorStepFunction,
        );

        /// The winding order of tessellated triangles.
        #[unsafe(method(tessellationOutputWindingOrder))]
        #[unsafe(method_family = none)]
        pub fn tessellation_output_winding_order(&self) -> MTLWinding;

        /// Setter for [`tessellation_output_winding_order`][Self::tessellation_output_winding_order].
        #[unsafe(method(setTessellationOutputWindingOrder:))]
        #[unsafe(method_family = none)]
        pub fn set_tessellation_output_winding_order(
            &self,
            winding: MTLWinding,
        );

        /// The vertex descriptor.
        #[unsafe(method(vertexDescriptor))]
        #[unsafe(method_family = none)]
        pub fn vertex_descriptor(&self) -> Option<Retained<MTLVertexDescriptor>>;

        /// Setter for [`vertex_descriptor`][Self::vertex_descriptor].
        #[unsafe(method(setVertexDescriptor:))]
        #[unsafe(method_family = none)]
        pub fn set_vertex_descriptor(
            &self,
            vertex_descriptor: Option<&MTLVertexDescriptor>,
        );

        /// Whether to support indirect command buffers.
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

        /// Resets the descriptor to its default values.
        #[unsafe(method(reset))]
        #[unsafe(method_family = none)]
        pub fn reset(&self);

        /// The shader validation mode.
        #[unsafe(method(shaderValidation))]
        #[unsafe(method_family = none)]
        pub fn shader_validation(&self) -> MTLShaderValidation;

        /// Setter for [`shader_validation`][Self::shader_validation].
        #[unsafe(method(setShaderValidation:))]
        #[unsafe(method_family = none)]
        pub fn set_shader_validation(
            &self,
            v: MTLShaderValidation,
        );
    );
}

impl MTLRenderPipelineDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub fn new() -> Retained<Self>;
    );
}

impl MTLRenderPipelineDescriptor {
    /// The set of dynamic libraries to be preloaded for the vertex stage.
    pub fn vertex_preloaded_libraries(&self) -> Box<[Retained<ProtocolObject<dyn MTLDynamicLibrary>>]> {
        let libraries: Retained<NSArray<ProtocolObject<dyn MTLDynamicLibrary>>> =
            unsafe { msg_send![self, vertexPreloadedLibraries] };
        libraries.to_vec().into_boxed_slice()
    }

    /// Setter for [`vertex_preloaded_libraries`][Self::vertex_preloaded_libraries].
    pub fn set_vertex_preloaded_libraries(
        &self,
        libraries: &[&ProtocolObject<dyn MTLDynamicLibrary>],
    ) {
        let libraries = NSArray::from_slice(libraries);
        unsafe {
            let _: () = msg_send![self, setVertexPreloadedLibraries: &*libraries];
        }
    }

    /// The set of dynamic libraries to be preloaded for the fragment stage.
    pub fn fragment_preloaded_libraries(&self) -> Box<[Retained<ProtocolObject<dyn MTLDynamicLibrary>>]> {
        let libraries: Retained<NSArray<ProtocolObject<dyn MTLDynamicLibrary>>> =
            unsafe { msg_send![self, fragmentPreloadedLibraries] };
        libraries.to_vec().into_boxed_slice()
    }

    /// Setter for [`fragment_preloaded_libraries`][Self::fragment_preloaded_libraries].
    pub fn set_fragment_preloaded_libraries(
        &self,
        libraries: &[&ProtocolObject<dyn MTLDynamicLibrary>],
    ) {
        let libraries = NSArray::from_slice(libraries);
        unsafe {
            let _: () = msg_send![self, setFragmentPreloadedLibraries: &*libraries];
        }
    }

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
        let s: Option<Retained<NSString>> = unsafe { msg_send![self, label] };
        s.map(|s| s.to_string())
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
