use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
};
use objc2_foundation::{CopyingHelper, NSCopying, NSObject, NSObjectProtocol, NSUInteger};

use crate::*;

extern_class!(
    /// Describes motion triangle geometry, suitable for motion ray tracing.
    ///
    /// Use a ``MTLResidencySet`` to mark residency of all buffers this descriptor references when you build this
    /// acceleration structure.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metal/mtl4accelerationstructuremotiontrianglegeometrydescriptor?language=objc)
    #[unsafe(super(MTL4AccelerationStructureGeometryDescriptor, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTL4AccelerationStructureMotionTriangleGeometryDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for MTL4AccelerationStructureMotionTriangleGeometryDescriptor {}
);

unsafe impl CopyingHelper for MTL4AccelerationStructureMotionTriangleGeometryDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTL4AccelerationStructureMotionTriangleGeometryDescriptor {}
);

impl MTL4AccelerationStructureMotionTriangleGeometryDescriptor {
    extern_methods!(
        /// Assigns a buffer where each entry contains a reference to a vertex buffer.
        ///
        /// This property references a buffer that conceptually represents an array with one entry for each keyframe in the
        /// motion animation. Each one of these entries consists of a ``MTL4BufferRange`` that, in turn, references a
        /// vertex buffer containing the vertex data for the keyframe.
        ///
        /// You are responsible for ensuring the buffer address is not zero for the top-level buffer, as well as for all
        /// the vertex buffers it references.
        #[unsafe(method(vertexBuffers))]
        #[unsafe(method_family = none)]
        pub fn vertex_buffers(&self) -> MTL4BufferRange;

        /// Setter for [`vertexBuffers`][Self::vertexBuffers].
        #[unsafe(method(setVertexBuffers:))]
        #[unsafe(method_family = none)]
        pub fn set_vertex_buffers(&self, vertex_buffers: MTL4BufferRange);

        /// Defines the format of the vertices in the vertex buffers.
        ///
        /// All keyframes share the same vertex format. Defaults to `MTLAttributeFormatFloat3`, corresponding to three packed
        /// floating point numbers.
        #[unsafe(method(vertexFormat))]
        #[unsafe(method_family = none)]
        pub fn vertex_format(&self) -> MTLAttributeFormat;

        /// Setter for [`vertexFormat`][Self::vertexFormat].
        #[unsafe(method(setVertexFormat:))]
        #[unsafe(method_family = none)]
        pub fn set_vertex_format(&self, vertex_format: MTLAttributeFormat);

        /// Sets the stride, in bytes, between vertices in all the vertex buffer.
        ///
        /// All keyframes share the same vertex stride. This stride needs to be a multiple of the size of the vertex format you
        /// provide in the ``vertexFormat`` property.
        ///
        /// Similarly, you are responsible for ensuring this stride matches the vertex format data type's alignment.
        ///
        /// Defaults to `0`, which signals the stride matches the size of the ``vertexFormat`` data.
        #[unsafe(method(vertexStride))]
        #[unsafe(method_family = none)]
        pub fn vertex_stride(&self) -> NSUInteger;

        /// Setter for [`vertexStride`][Self::vertexStride].
        #[unsafe(method(setVertexStride:))]
        #[unsafe(method_family = none)]
        pub fn set_vertex_stride(&self, vertex_stride: NSUInteger);

        /// Assigns an optional index buffer containing references to vertices in the vertex buffers you reference through the
        /// vertex buffers property.
        ///
        /// You can set this property to `0`, the default, to avoid specifying an index buffer. All keyframes share the same
        /// index buffer.
        #[unsafe(method(indexBuffer))]
        #[unsafe(method_family = none)]
        pub fn index_buffer(&self) -> MTL4BufferRange;

        /// Setter for [`indexBuffer`][Self::indexBuffer].
        #[unsafe(method(setIndexBuffer:))]
        #[unsafe(method_family = none)]
        pub fn set_index_buffer(&self, index_buffer: MTL4BufferRange);

        /// Specifies the size of the indices the `indexBuffer` contains, which is typically either 16 or 32-bits for each index.
        #[unsafe(method(indexType))]
        #[unsafe(method_family = none)]
        pub fn index_type(&self) -> MTLIndexType;

        /// Setter for [`indexType`][Self::indexType].
        #[unsafe(method(setIndexType:))]
        #[unsafe(method_family = none)]
        pub fn set_index_type(&self, index_type: MTLIndexType);

        /// Declares the number of triangles in the vertex buffers that the buffer in the vertex buffers property references.
        ///
        /// All keyframes share the same triangle count.
        #[unsafe(method(triangleCount))]
        #[unsafe(method_family = none)]
        pub fn triangle_count(&self) -> NSUInteger;

        /// Setter for [`triangleCount`][Self::triangleCount].
        #[unsafe(method(setTriangleCount:))]
        #[unsafe(method_family = none)]
        pub fn set_triangle_count(&self, triangle_count: NSUInteger);

        /// Assings an optional reference to a buffer containing a `float4x3` transformation matrix.
        ///
        /// When the buffer address is non-zero, Metal applies this transform to the vertex data positions when building
        /// the acceleration structure. All keyframes share the same transformation matrix.
        ///
        /// Building an acceleration structure with a descriptor that specifies this property doesn't modify the contents of
        /// the input `vertexBuffer`.
        #[unsafe(method(transformationMatrixBuffer))]
        #[unsafe(method_family = none)]
        pub fn transformation_matrix_buffer(&self) -> MTL4BufferRange;

        /// Setter for [`transformationMatrixBuffer`][Self::transformationMatrixBuffer].
        #[unsafe(method(setTransformationMatrixBuffer:))]
        #[unsafe(method_family = none)]
        pub fn set_transformation_matrix_buffer(
            &self,
            transformation_matrix_buffer: MTL4BufferRange,
        );

        /// Configures the layout for the transformation matrix in the transformation matrix buffer.
        ///
        /// You can provide matrices in column-major or row-major form, and this property allows you to control
        /// how Metal interprets them.
        ///
        /// Defaults to `MTLMatrixLayoutColumnMajor`.
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
    );
}

/// Methods declared on superclass `NSObject`.
impl MTL4AccelerationStructureMotionTriangleGeometryDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub fn new() -> Retained<Self>;
    );
}
