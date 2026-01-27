use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
    runtime::{NSObject, ProtocolObject},
};
use objc2_foundation::{CopyingHelper, NSCopying, NSObjectProtocol};

use crate::{
    MTLAccelerationStructureGeometryDescriptor, MTLAttributeFormat, MTLBuffer, MTLIndexType,
};

extern_class!(
    /// Acceleration structure geometry descriptor describing geometry
    /// made of curve primitives
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metal/mtlaccelerationstructurecurvegeometrydescriptor?language=objc)
    #[unsafe(super(MTLAccelerationStructureGeometryDescriptor, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLAccelerationStructureCurveGeometryDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for MTLAccelerationStructureCurveGeometryDescriptor {}
);

unsafe impl CopyingHelper for MTLAccelerationStructureCurveGeometryDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLAccelerationStructureCurveGeometryDescriptor {}
);

impl MTLAccelerationStructureCurveGeometryDescriptor {
    extern_methods!(
        /// Buffer containing curve control points. Each control point must
        /// be of the format specified by the control point format. Must not be
        /// nil when the acceleration structure is built.
        #[unsafe(method(controlPointBuffer))]
        #[unsafe(method_family = none)]
        pub fn control_point_buffer(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn MTLBuffer>>>;

        /// Setter for [`controlPointBuffer`][Self::controlPointBuffer].
        #[unsafe(method(setControlPointBuffer:))]
        #[unsafe(method_family = none)]
        pub fn set_control_point_buffer(
            &self,
            control_point_buffer: Option<&ProtocolObject<dyn MTLBuffer>>,
        );

        /// Control point buffer offset. Must be a multiple of the control
        /// point format's element size and must be aligned to the platform's
        /// buffer offset alignment.
        #[unsafe(method(controlPointBufferOffset))]
        #[unsafe(method_family = none)]
        pub fn control_point_buffer_offset(&self) -> usize;

        /// Setter for [`controlPointBufferOffset`][Self::controlPointBufferOffset].
        #[unsafe(method(setControlPointBufferOffset:))]
        #[unsafe(method_family = none)]
        pub fn set_control_point_buffer_offset(&self, control_point_buffer_offset: usize);

        /// Number of control points in the control point buffer
        #[unsafe(method(controlPointCount))]
        #[unsafe(method_family = none)]
        pub fn control_point_count(&self) -> usize;

        /// Setter for [`controlPointCount`][Self::controlPointCount].
        #[unsafe(method(setControlPointCount:))]
        #[unsafe(method_family = none)]
        pub fn set_control_point_count(&self, control_point_count: usize);

        /// Stride, in bytes, between control points in the control point
        /// buffer. Must be a multiple of the control point format's element size
        /// and must be at least the control point format's size. Defaults to 0
        /// bytes, indicating that the control points are tightly packed.
        #[unsafe(method(controlPointStride))]
        #[unsafe(method_family = none)]
        pub fn control_point_stride(&self) -> usize;

        /// Setter for [`controlPointStride`][Self::controlPointStride].
        #[unsafe(method(setControlPointStride:))]
        #[unsafe(method_family = none)]
        pub fn set_control_point_stride(&self, control_point_stride: usize);

        /// Format of the control points in the control point buffer.
        /// Defaults to MTLAttributeFormatFloat3 (packed).
        #[unsafe(method(controlPointFormat))]
        #[unsafe(method_family = none)]
        pub fn control_point_format(&self) -> MTLAttributeFormat;

        /// Setter for [`controlPointFormat`][Self::controlPointFormat].
        #[unsafe(method(setControlPointFormat:))]
        #[unsafe(method_family = none)]
        pub fn set_control_point_format(&self, control_point_format: MTLAttributeFormat);

        /// Buffer containing the curve radius for each control point. Each
        /// radius must be of the type specified by the radius format. Each radius
        /// must be at least zero. Must not be nil when the acceleration structure
        /// is built.
        #[unsafe(method(radiusBuffer))]
        #[unsafe(method_family = none)]
        pub fn radius_buffer(&self) -> Option<Retained<ProtocolObject<dyn MTLBuffer>>>;

        /// Setter for [`radiusBuffer`][Self::radiusBuffer].
        #[unsafe(method(setRadiusBuffer:))]
        #[unsafe(method_family = none)]
        pub fn set_radius_buffer(
            &self,
            radius_buffer: Option<&ProtocolObject<dyn MTLBuffer>>,
        );

        /// Radius buffer offset. Must be a multiple of the radius format
        /// size and must be aligned to the platform's buffer offset alignment.
        #[unsafe(method(radiusBufferOffset))]
        #[unsafe(method_family = none)]
        pub fn radius_buffer_offset(&self) -> usize;

        /// Setter for [`radiusBufferOffset`][Self::radiusBufferOffset].
        #[unsafe(method(setRadiusBufferOffset:))]
        #[unsafe(method_family = none)]
        pub fn set_radius_buffer_offset(&self, radius_buffer_offset: usize);

        /// Format of the radii in the radius buffer. Defaults to
        /// MTLAttributeFormatFloat.
        #[unsafe(method(radiusFormat))]
        #[unsafe(method_family = none)]
        pub fn radius_format(&self) -> MTLAttributeFormat;

        /// Setter for [`radiusFormat`][Self::radiusFormat].
        #[unsafe(method(setRadiusFormat:))]
        #[unsafe(method_family = none)]
        pub fn set_radius_format(&self, radius_format: MTLAttributeFormat);

        /// Stride, in bytes, between radii in the radius buffer. Must be
        /// a multiple of the radius format size. Defaults to 0 bytes, indicating
        /// that the radii are tightly packed.
        #[unsafe(method(radiusStride))]
        #[unsafe(method_family = none)]
        pub fn radius_stride(&self) -> usize;

        /// Setter for [`radiusStride`][Self::radiusStride].
        #[unsafe(method(setRadiusStride:))]
        #[unsafe(method_family = none)]
        pub fn set_radius_stride(&self, radius_stride: usize);

        /// Index buffer containing references to control points in the control
        /// point buffer. Must not be nil when the acceleration structure is built.
        #[unsafe(method(indexBuffer))]
        #[unsafe(method_family = none)]
        pub fn index_buffer(&self) -> Option<Retained<ProtocolObject<dyn MTLBuffer>>>;

        /// Setter for [`indexBuffer`][Self::indexBuffer].
        #[unsafe(method(setIndexBuffer:))]
        #[unsafe(method_family = none)]
        pub fn set_index_buffer(&self, index_buffer: Option<&ProtocolObject<dyn MTLBuffer>>);

        /// Index buffer offset. Must be a multiple of the index data type
        /// size and must be aligned to both the index data type's alignment and
        /// the platform's buffer offset alignment.
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

        /// Number of curve segments
        #[unsafe(method(segmentCount))]
        #[unsafe(method_family = none)]
        pub fn segment_count(&self) -> usize;

        /// Setter for [`segmentCount`][Self::segmentCount].
        #[unsafe(method(setSegmentCount:))]
        #[unsafe(method_family = none)]
        pub fn set_segment_count(&self, segment_count: usize);

        /// Number of control points per curve segment. Must be 2, 3, or 4.
        #[unsafe(method(segmentControlPointCount))]
        #[unsafe(method_family = none)]
        pub fn segment_control_point_count(&self) -> usize;

        /// Setter for [`segmentControlPointCount`][Self::segmentControlPointCount].
        #[unsafe(method(setSegmentControlPointCount:))]
        #[unsafe(method_family = none)]
        pub fn set_segment_control_point_count(&self, segment_control_point_count: usize);

        /// Curve type. Defaults to MTLCurveTypeRound.
        #[unsafe(method(curveType))]
        #[unsafe(method_family = none)]
        pub fn curve_type(&self) -> crate::MTLCurveType;

        /// Setter for [`curveType`][Self::curveType].
        #[unsafe(method(setCurveType:))]
        #[unsafe(method_family = none)]
        pub fn set_curve_type(&self, curve_type: crate::MTLCurveType);

        /// Curve basis. Defaults to MTLCurveBasisBSpline.
        #[unsafe(method(curveBasis))]
        #[unsafe(method_family = none)]
        pub fn curve_basis(&self) -> crate::MTLCurveBasis;

        /// Setter for [`curveBasis`][Self::curveBasis].
        #[unsafe(method(setCurveBasis:))]
        #[unsafe(method_family = none)]
        pub fn set_curve_basis(&self, curve_basis: crate::MTLCurveBasis);

        /// Type of curve end caps. Defaults to MTLCurveEndCapsNone.
        #[unsafe(method(curveEndCaps))]
        #[unsafe(method_family = none)]
        pub fn curve_end_caps(&self) -> crate::MTLCurveEndCaps;

        /// Setter for [`curveEndCaps`][Self::curveEndCaps].
        #[unsafe(method(setCurveEndCaps:))]
        #[unsafe(method_family = none)]
        pub fn set_curve_end_caps(&self, curve_end_caps: crate::MTLCurveEndCaps);

        #[unsafe(method(descriptor))]
        #[unsafe(method_family = none)]
        pub fn descriptor() -> Retained<Self>;
    );
}

/// Methods declared on superclass `NSObject`.
impl MTLAccelerationStructureCurveGeometryDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub fn new() -> Retained<Self>;
    );
}
