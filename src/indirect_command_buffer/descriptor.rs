use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
    runtime::NSObject,
};
use objc2_foundation::{CopyingHelper, NSCopying, NSObjectProtocol};

use super::types::MTLIndirectCommandType;

extern_class!(
    /// Descriptor specifying limits and features for an indirect command buffer.
    ///
    /// Availability: macOS 10.14+, iOS 12.0+
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLIndirectCommandBufferDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for MTLIndirectCommandBufferDescriptor {}
);

unsafe impl CopyingHelper for MTLIndirectCommandBufferDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLIndirectCommandBufferDescriptor {}
);

impl MTLIndirectCommandBufferDescriptor {
    extern_methods!(
        /// Command types that may be encoded.
        ///
        /// Availability: macOS 10.14+, iOS 12.0+
        #[unsafe(method(commandTypes))]
        #[unsafe(method_family = none)]
        pub fn command_types(&self) -> MTLIndirectCommandType;

        /// Setter for [`command_types`][Self::command_types].
        #[unsafe(method(setCommandTypes:))]
        #[unsafe(method_family = none)]
        pub fn set_command_types(&self, command_types: MTLIndirectCommandType);

        /// Whether pipelines are inherited from the encoder.
        ///
        /// Availability: macOS 10.14+, iOS 13.0+
        #[unsafe(method(inheritPipelineState))]
        #[unsafe(method_family = none)]
        pub fn inherit_pipeline_state(&self) -> bool;

        /// Setter for [`inherit_pipeline_state`][Self::inherit_pipeline_state].
        #[unsafe(method(setInheritPipelineState:))]
        #[unsafe(method_family = none)]
        pub fn set_inherit_pipeline_state(&self, inherit_pipeline_state: bool);

        /// Whether argument buffers are inherited.
        #[unsafe(method(inheritBuffers))]
        #[unsafe(method_family = none)]
        pub fn inherit_buffers(&self) -> bool;

        /// Setter for [`inherit_buffers`][Self::inherit_buffers].
        #[unsafe(method(setInheritBuffers:))]
        #[unsafe(method_family = none)]
        pub fn set_inherit_buffers(&self, inherit_buffers: bool);

        /// Whether depth/stencil state is inherited from the encoder.
        ///
        /// Availability: macOS 26.0+, iOS 26.0+
        #[unsafe(method(inheritDepthStencilState))]
        #[unsafe(method_family = none)]
        pub fn inherit_depth_stencil_state(&self) -> bool;

        /// Setter for [`inherit_depth_stencil_state`][Self::inherit_depth_stencil_state].
        #[unsafe(method(setInheritDepthStencilState:))]
        #[unsafe(method_family = none)]
        pub fn set_inherit_depth_stencil_state(&self, inherit: bool);

        /// Whether depth bias is inherited from the encoder.
        ///
        /// Availability: macOS 26.0+, iOS 26.0+
        #[unsafe(method(inheritDepthBias))]
        #[unsafe(method_family = none)]
        pub fn inherit_depth_bias(&self) -> bool;

        /// Setter for [`inherit_depth_bias`][Self::inherit_depth_bias].
        #[unsafe(method(setInheritDepthBias:))]
        #[unsafe(method_family = none)]
        pub fn set_inherit_depth_bias(&self, inherit: bool);

        /// Whether depth clip mode is inherited from the encoder.
        ///
        /// Availability: macOS 26.0+, iOS 26.0+
        #[unsafe(method(inheritDepthClipMode))]
        #[unsafe(method_family = none)]
        pub fn inherit_depth_clip_mode(&self) -> bool;

        /// Setter for [`inherit_depth_clip_mode`][Self::inherit_depth_clip_mode].
        #[unsafe(method(setInheritDepthClipMode:))]
        #[unsafe(method_family = none)]
        pub fn set_inherit_depth_clip_mode(&self, inherit: bool);

        /// Whether cull mode is inherited from the encoder.
        ///
        /// Availability: macOS 26.0+, iOS 26.0+
        #[unsafe(method(inheritCullMode))]
        #[unsafe(method_family = none)]
        pub fn inherit_cull_mode(&self) -> bool;

        /// Setter for [`inherit_cull_mode`][Self::inherit_cull_mode].
        #[unsafe(method(setInheritCullMode:))]
        #[unsafe(method_family = none)]
        pub fn set_inherit_cull_mode(&self, inherit: bool);

        /// Whether front-facing winding is inherited from the encoder.
        ///
        /// Availability: macOS 26.0+, iOS 26.0+
        #[unsafe(method(inheritFrontFacingWinding))]
        #[unsafe(method_family = none)]
        pub fn inherit_front_facing_winding(&self) -> bool;

        /// Setter for [`inherit_front_facing_winding`][Self::inherit_front_facing_winding].
        #[unsafe(method(setInheritFrontFacingWinding:))]
        #[unsafe(method_family = none)]
        pub fn set_inherit_front_facing_winding(&self, inherit: bool);

        /// Whether triangle fill mode is inherited from the encoder.
        ///
        /// Availability: macOS 26.0+, iOS 26.0+
        #[unsafe(method(inheritTriangleFillMode))]
        #[unsafe(method_family = none)]
        pub fn inherit_triangle_fill_mode(&self) -> bool;

        /// Setter for [`inherit_triangle_fill_mode`][Self::inherit_triangle_fill_mode].
        #[unsafe(method(setInheritTriangleFillMode:))]
        #[unsafe(method_family = none)]
        pub fn set_inherit_triangle_fill_mode(&self, inherit: bool);

        /// Maximum vertex buffer bind index per command.
        #[unsafe(method(maxVertexBufferBindCount))]
        #[unsafe(method_family = none)]
        pub fn max_vertex_buffer_bind_count(&self) -> usize;

        /// Setter for [`max_vertex_buffer_bind_count`][Self::max_vertex_buffer_bind_count].
        #[unsafe(method(setMaxVertexBufferBindCount:))]
        #[unsafe(method_family = none)]
        pub fn set_max_vertex_buffer_bind_count(&self, count: usize);

        /// Maximum fragment buffer bind index per command.
        #[unsafe(method(maxFragmentBufferBindCount))]
        #[unsafe(method_family = none)]
        pub fn max_fragment_buffer_bind_count(&self) -> usize;

        /// Setter for [`max_fragment_buffer_bind_count`][Self::max_fragment_buffer_bind_count].
        #[unsafe(method(setMaxFragmentBufferBindCount:))]
        #[unsafe(method_family = none)]
        pub fn set_max_fragment_buffer_bind_count(&self, count: usize);

        /// Maximum kernel buffer bind index per command.
        ///
        /// Availability: macOS 11.0+, iOS 13.0+
        #[unsafe(method(maxKernelBufferBindCount))]
        #[unsafe(method_family = none)]
        pub fn max_kernel_buffer_bind_count(&self) -> usize;

        /// Setter for [`max_kernel_buffer_bind_count`][Self::max_kernel_buffer_bind_count].
        #[unsafe(method(setMaxKernelBufferBindCount:))]
        #[unsafe(method_family = none)]
        pub fn set_max_kernel_buffer_bind_count(&self, count: usize);

        /// Maximum kernel threadgroup memory bind count.
        ///
        /// Availability: macOS 14.0+, iOS 17.0+
        #[unsafe(method(maxKernelThreadgroupMemoryBindCount))]
        #[unsafe(method_family = none)]
        pub fn max_kernel_threadgroup_memory_bind_count(&self) -> usize;

        /// Setter for [`max_kernel_threadgroup_memory_bind_count`][Self::max_kernel_threadgroup_memory_bind_count].
        #[unsafe(method(setMaxKernelThreadgroupMemoryBindCount:))]
        #[unsafe(method_family = none)]
        pub fn set_max_kernel_threadgroup_memory_bind_count(&self, count: usize);

        /// Maximum object buffer bind index per render command.
        ///
        /// Availability: macOS 14.0+, iOS 17.0+
        #[unsafe(method(maxObjectBufferBindCount))]
        #[unsafe(method_family = none)]
        pub fn max_object_buffer_bind_count(&self) -> usize;

        /// Setter for [`max_object_buffer_bind_count`][Self::max_object_buffer_bind_count].
        #[unsafe(method(setMaxObjectBufferBindCount:))]
        #[unsafe(method_family = none)]
        pub fn set_max_object_buffer_bind_count(&self, count: usize);

        /// Maximum mesh buffer bind index per render command.
        ///
        /// Availability: macOS 14.0+, iOS 17.0+
        #[unsafe(method(maxMeshBufferBindCount))]
        #[unsafe(method_family = none)]
        pub fn max_mesh_buffer_bind_count(&self) -> usize;

        /// Setter for [`max_mesh_buffer_bind_count`][Self::max_mesh_buffer_bind_count].
        #[unsafe(method(setMaxMeshBufferBindCount:))]
        #[unsafe(method_family = none)]
        pub fn set_max_mesh_buffer_bind_count(&self, count: usize);

        /// Maximum object threadgroup memory bind count.
        ///
        /// Availability: macOS 14.0+, iOS 17.0+
        #[unsafe(method(maxObjectThreadgroupMemoryBindCount))]
        #[unsafe(method_family = none)]
        pub fn max_object_threadgroup_memory_bind_count(&self) -> usize;

        /// Setter for [`max_object_threadgroup_memory_bind_count`][Self::max_object_threadgroup_memory_bind_count].
        #[unsafe(method(setMaxObjectThreadgroupMemoryBindCount:))]
        #[unsafe(method_family = none)]
        pub fn set_max_object_threadgroup_memory_bind_count(&self, count: usize);

        /// Whether render/compute commands can use ray tracing.
        ///
        /// Availability: macOS 13.0+, iOS 16.0+
        #[unsafe(method(supportRayTracing))]
        #[unsafe(method_family = none)]
        pub fn support_ray_tracing(&self) -> bool;

        /// Setter for [`support_ray_tracing`][Self::support_ray_tracing].
        #[unsafe(method(setSupportRayTracing:))]
        #[unsafe(method_family = none)]
        pub fn set_support_ray_tracing(&self, support: bool);

        /// Allow setting dynamic attribute stride.
        ///
        /// Availability: macOS 14.0+, iOS 17.0+
        #[unsafe(method(supportDynamicAttributeStride))]
        #[unsafe(method_family = none)]
        pub fn support_dynamic_attribute_stride(&self) -> bool;

        /// Setter for [`support_dynamic_attribute_stride`][Self::support_dynamic_attribute_stride].
        #[unsafe(method(setSupportDynamicAttributeStride:))]
        #[unsafe(method_family = none)]
        pub fn set_support_dynamic_attribute_stride(&self, support: bool);

        /// Whether to support color attachment mapping.
        ///
        /// Availability: macOS 26.0+, iOS 26.0+
        #[unsafe(method(supportColorAttachmentMapping))]
        #[unsafe(method_family = none)]
        pub fn support_color_attachment_mapping(&self) -> bool;

        /// Setter for [`support_color_attachment_mapping`][Self::support_color_attachment_mapping].
        #[unsafe(method(setSupportColorAttachmentMapping:))]
        #[unsafe(method_family = none)]
        pub fn set_support_color_attachment_mapping(&self, support: bool);
    );
}

/// Methods declared on superclass `NSObject`.
impl MTLIndirectCommandBufferDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
