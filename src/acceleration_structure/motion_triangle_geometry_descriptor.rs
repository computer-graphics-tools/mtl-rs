use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
    runtime::{NSObject, ProtocolObject},
};
use objc2_foundation::{CopyingHelper, NSArray, NSCopying, NSObjectProtocol};

use crate::{
    MTLAccelerationStructureGeometryDescriptor, MTLAttributeFormat, MTLBuffer, MTLIndexType,
    MTLMatrixLayout, MTLMotionKeyframeData,
};

extern_class!(
    /// Descriptor for motion triangle geometry
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metal/mtlaccelerationstructuremotiontrianglegeometrydescriptor?language=objc)
    #[unsafe(super(MTLAccelerationStructureGeometryDescriptor, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLAccelerationStructureMotionTriangleGeometryDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for MTLAccelerationStructureMotionTriangleGeometryDescriptor {}
);

unsafe impl CopyingHelper for MTLAccelerationStructureMotionTriangleGeometryDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLAccelerationStructureMotionTriangleGeometryDescriptor {}
);

impl MTLAccelerationStructureMotionTriangleGeometryDescriptor {
    extern_methods!(
        /// Vertex buffer containing triangle vertices similar to what MTLAccelerationStructureTriangleGeometryDescriptor has but array of the values.
        #[unsafe(method(vertexBuffers))]
        #[unsafe(method_family = none)]
        pub fn vertex_buffers(&self) -> Retained<NSArray<MTLMotionKeyframeData>>;

        /// Setter for [`vertexBuffers`][Self::vertexBuffers].
        ///
        /// This is [copied][objc2_foundation::NSCopying::copy] when set.
        #[unsafe(method(setVertexBuffers:))]
        #[unsafe(method_family = none)]
        pub fn set_vertex_buffers(&self, vertex_buffers: &NSArray<MTLMotionKeyframeData>);

        /// Format type of the vertex buffers across all keyframes.
        /// Defaults to MTLAttributeFormatFloat3 (packed).
        #[unsafe(method(vertexFormat))]
        #[unsafe(method_family = none)]
        pub fn vertex_format(&self) -> MTLAttributeFormat;

        /// Setter for [`vertexFormat`][Self::vertexFormat].
        #[unsafe(method(setVertexFormat:))]
        #[unsafe(method_family = none)]
        pub fn set_vertex_format(&self, vertex_format: MTLAttributeFormat);

        /// Stride, in bytes, between vertices in each keyframe's vertex buffer. Must be a multiple of the vertex format data type size and must be aligned to
        /// the vertex format data type's alignment. Defaults to 0, which will result in a stride of the vertex format data size.
        #[unsafe(method(vertexStride))]
        #[unsafe(method_family = none)]
        pub fn vertex_stride(&self) -> usize;

        /// Setter for [`vertexStride`][Self::vertexStride].
        #[unsafe(method(setVertexStride:))]
        #[unsafe(method_family = none)]
        pub fn set_vertex_stride(&self, vertex_stride: usize);

        /// Optional index buffer containing references to vertices in the vertex buffer. May be nil.
        #[unsafe(method(indexBuffer))]
        #[unsafe(method_family = none)]
        pub fn index_buffer(&self) -> Option<Retained<ProtocolObject<dyn MTLBuffer>>>;

        /// Setter for [`indexBuffer`][Self::indexBuffer].
        #[unsafe(method(setIndexBuffer:))]
        #[unsafe(method_family = none)]
        pub fn set_index_buffer(&self, index_buffer: Option<&ProtocolObject<dyn MTLBuffer>>);

        /// Index buffer offset. Must be a multiple of the index data type size and must be aligned to both
        /// the index data type's alignment and the platform's buffer offset alignment.
        #[unsafe(method(indexBufferOffset))]
        #[unsafe(method_family = none)]
        pub fn index_buffer_offset(&self) -> usize;

        /// Setter for [`indexBufferOffset`][Self::indexBufferOffset].
        #[unsafe(method(setIndexBufferOffset:))]
        #[unsafe(method_family = none)]
        pub fn set_index_buffer_offset(&self, index_buffer_offset: usize);

        /// Index type
        #[unsafe(method(indexType))]
        #[unsafe(method_family = none)]
        pub fn index_type(&self) -> MTLIndexType;

        /// Setter for [`indexType`][Self::indexType].
        #[unsafe(method(setIndexType:))]
        #[unsafe(method_family = none)]
        pub fn set_index_type(&self, index_type: MTLIndexType);

        /// Number of triangles
        #[unsafe(method(triangleCount))]
        #[unsafe(method_family = none)]
        pub fn triangle_count(&self) -> usize;

        /// Setter for [`triangleCount`][Self::triangleCount].
        #[unsafe(method(setTriangleCount:))]
        #[unsafe(method_family = none)]
        pub fn set_triangle_count(&self, triangle_count: usize);

        /// Buffer containing packed float4x3 transformation matrix. Transform is applied to the vertex data when building the acceleration structure. Input vertex buffers are not modified.
        /// The transformation matrix is applied to all keyframes' vertex data.
        /// When set to nil, transformation matrix is not applied to vertex data.
        #[unsafe(method(transformationMatrixBuffer))]
        #[unsafe(method_family = none)]
        pub fn transformation_matrix_buffer(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn MTLBuffer>>>;

        /// Setter for [`transformationMatrixBuffer`][Self::transformationMatrixBuffer].
        #[unsafe(method(setTransformationMatrixBuffer:))]
        #[unsafe(method_family = none)]
        pub fn set_transformation_matrix_buffer(
            &self,
            transformation_matrix_buffer: Option<&ProtocolObject<dyn MTLBuffer>>,
        );

        /// Transformation matrix buffer offset. Must be a multiple of 4 bytes. Defaults to 0.
        #[unsafe(method(transformationMatrixBufferOffset))]
        #[unsafe(method_family = none)]
        pub fn transformation_matrix_buffer_offset(&self) -> usize;

        /// Setter for [`transformationMatrixBufferOffset`][Self::transformationMatrixBufferOffset].
        #[unsafe(method(setTransformationMatrixBufferOffset:))]
        #[unsafe(method_family = none)]
        pub fn set_transformation_matrix_buffer_offset(
            &self,
            transformation_matrix_buffer_offset: usize,
        );

        /// Matrix layout for the transformation matrix in the transformation
        /// matrix buffer. Defaults to MTLMatrixLayoutColumnMajor.
        #[unsafe(method(transformationMatrixLayout))]
        #[unsafe(method_family = none)]
        pub fn transformation_matrix_layout(&self) -> MTLMatrixLayout;

        /// Setter for [`transformationMatrixLayout`][Self::transformationMatrixLayout].
        #[unsafe(method(setTransformationMatrixLayout:))]
        #[unsafe(method_family = none)]
        pub fn set_transformation_matrix_layout(
            &self,
            transformation_matrix_layout: MTLMatrixLayout,
        );

        #[unsafe(method(descriptor))]
        #[unsafe(method_family = none)]
        pub fn descriptor() -> Retained<Self>;
    );
}

/// Methods declared on superclass `NSObject`.
impl MTLAccelerationStructureMotionTriangleGeometryDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub fn new() -> Retained<Self>;
    );
}
