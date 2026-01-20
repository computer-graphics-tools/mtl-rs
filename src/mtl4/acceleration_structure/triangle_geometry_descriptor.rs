use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
};
use objc2_foundation::{CopyingHelper, NSCopying, NSObject, NSObjectProtocol, NSUInteger};

use crate::*;

extern_class!(
    /// Describes triangle geometry suitable for ray tracing.
    ///
    /// Use a ``MTLResidencySet`` to mark residency of all buffers this descriptor references when you build this
    /// acceleration structure.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metal/mtl4accelerationstructuretrianglegeometrydescriptor?language=objc)
    #[unsafe(super(MTL4AccelerationStructureGeometryDescriptor, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTL4AccelerationStructureTriangleGeometryDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for MTL4AccelerationStructureTriangleGeometryDescriptor {}
);

unsafe impl CopyingHelper for MTL4AccelerationStructureTriangleGeometryDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTL4AccelerationStructureTriangleGeometryDescriptor {}
);

impl MTL4AccelerationStructureTriangleGeometryDescriptor {
    extern_methods!(
        /// Associates a vertex buffer containing triangle vertices.
        ///
        /// You are responsible for ensuring that the format of all vertex positions match the ``vertexFormat`` property, and
        /// that the buffer address for the buffer range is not zero.
        #[unsafe(method(vertexBuffer))]
        #[unsafe(method_family = none)]
        pub unsafe fn vertex_buffer(&self) -> MTL4BufferRange;

        /// Setter for [`vertexBuffer`][Self::vertexBuffer].
        #[unsafe(method(setVertexBuffer:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_vertex_buffer(&self, vertex_buffer: MTL4BufferRange);

        /// Describes the format of the vertices in the vertex buffer.
        ///
        /// This property controls the format of the position attribute of the vertices the ``vertexBuffer`` references.
        ///
        /// The format defaults to `MTLAttributeFormatFloat3`, corresponding to three packed floating point numbers.
        #[unsafe(method(vertexFormat))]
        #[unsafe(method_family = none)]
        pub unsafe fn vertex_format(&self) -> MTLAttributeFormat;

        /// Setter for [`vertexFormat`][Self::vertexFormat].
        #[unsafe(method(setVertexFormat:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_vertex_format(&self, vertex_format: MTLAttributeFormat);

        /// Sets the stride, in bytes, between vertices in the vertex buffer.
        ///
        /// The stride you specify needs to be a multiple of the size of the vertex format you provide in the ``vertexFormat``
        /// property. Similarly, you are responsible for ensuring this stride matches the vertex format data type's alignment.
        ///
        /// Defaults to `0`, which signals the stride matches the size of the ``vertexFormat`` data.
        #[unsafe(method(vertexStride))]
        #[unsafe(method_family = none)]
        pub unsafe fn vertex_stride(&self) -> NSUInteger;

        /// Setter for [`vertexStride`][Self::vertexStride].
        #[unsafe(method(setVertexStride:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_vertex_stride(&self, vertex_stride: NSUInteger);

        /// Sets an optional index buffer containing references to vertices in the `vertexBuffer`.
        ///
        /// You can set this property to `0`, the default, to avoid specifying an index buffer.
        #[unsafe(method(indexBuffer))]
        #[unsafe(method_family = none)]
        pub unsafe fn index_buffer(&self) -> MTL4BufferRange;

        /// Setter for [`indexBuffer`][Self::indexBuffer].
        #[unsafe(method(setIndexBuffer:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_index_buffer(&self, index_buffer: MTL4BufferRange);

        /// Configures the size of the indices the `indexBuffer` contains, which is typically either 16 or 32-bits for each index.
        #[unsafe(method(indexType))]
        #[unsafe(method_family = none)]
        pub unsafe fn index_type(&self) -> MTLIndexType;

        /// Setter for [`indexType`][Self::indexType].
        #[unsafe(method(setIndexType:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_index_type(&self, index_type: MTLIndexType);

        /// Declares the number of triangles in this geometry descriptor.
        #[unsafe(method(triangleCount))]
        #[unsafe(method_family = none)]
        pub unsafe fn triangle_count(&self) -> NSUInteger;

        /// Setter for [`triangleCount`][Self::triangleCount].
        #[unsafe(method(setTriangleCount:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_triangle_count(&self, triangle_count: NSUInteger);

        /// Assigns an optional reference to a buffer containing a `float4x3` transformation matrix.
        ///
        /// When the buffer address is non-zero, Metal applies this transform to the vertex data positions when building
        /// the acceleration structure.
        ///
        /// Building an acceleration structure with a descriptor that specifies this property doesn't modify the contents of
        /// the input `vertexBuffer`.
        #[unsafe(method(transformationMatrixBuffer))]
        #[unsafe(method_family = none)]
        pub unsafe fn transformation_matrix_buffer(&self) -> MTL4BufferRange;

        /// Setter for [`transformationMatrixBuffer`][Self::transformationMatrixBuffer].
        #[unsafe(method(setTransformationMatrixBuffer:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_transformation_matrix_buffer(
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
        pub unsafe fn transformation_matrix_layout(&self) -> MTLMatrixLayout;

        /// Setter for [`transformationMatrixLayout`][Self::transformationMatrixLayout].
        #[unsafe(method(setTransformationMatrixLayout:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_transformation_matrix_layout(
            &self,
            transformation_matrix_layout: MTLMatrixLayout,
        );
    );
}

/// Methods declared on superclass `NSObject`.
impl MTL4AccelerationStructureTriangleGeometryDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
