use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
};
use objc2_foundation::{CopyingHelper, NSCopying, NSObject, NSObjectProtocol, NSUInteger};

use crate::*;

extern_class!(
    /// Describes motion curve geometry, suitable for motion ray tracing.
    ///
    /// Use a ``MTLResidencySet`` to mark residency of all buffers this descriptor references when you build this
    /// acceleration structure.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metal/mtl4accelerationstructuremotioncurvegeometrydescriptor?language=objc)
    #[unsafe(super(MTL4AccelerationStructureGeometryDescriptor, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTL4AccelerationStructureMotionCurveGeometryDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for MTL4AccelerationStructureMotionCurveGeometryDescriptor {}
);

unsafe impl CopyingHelper for MTL4AccelerationStructureMotionCurveGeometryDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTL4AccelerationStructureMotionCurveGeometryDescriptor {}
);

impl MTL4AccelerationStructureMotionCurveGeometryDescriptor {
    extern_methods!(
        /// Assigns a reference to a buffer where each entry contains a reference to a buffer of control points.
        ///
        /// This property references a buffer that conceptually represents an array with one entry for each keyframe in the
        /// motion animation. Each one of these entries consists of a ``MTL4BufferRange`` that, in turn, references a
        /// buffer containing the control points corresponding to the keyframe.
        ///
        /// You are responsible for ensuring the buffer address is not zero for the top-level buffer, as well as for all
        /// the vertex buffers it references.
        #[unsafe(method(controlPointBuffers))]
        #[unsafe(method_family = none)]
        pub unsafe fn control_point_buffers(&self) -> MTL4BufferRange;

        /// Setter for [`controlPointBuffers`][Self::controlPointBuffers].
        #[unsafe(method(setControlPointBuffers:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_control_point_buffers(&self, control_point_buffers: MTL4BufferRange);

        /// Specifies the number of control points in the buffers the control point buffers reference.
        ///
        /// All keyframes have the same number of control points.
        #[unsafe(method(controlPointCount))]
        #[unsafe(method_family = none)]
        pub unsafe fn control_point_count(&self) -> NSUInteger;

        /// Setter for [`controlPointCount`][Self::controlPointCount].
        #[unsafe(method(setControlPointCount:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_control_point_count(&self, control_point_count: NSUInteger);

        /// Sets the stride, in bytes, between control points in the control point buffer.
        ///
        /// All keyframes share the same control point stride.
        ///
        /// You are responsible for ensuring this stride is a multiple of the control point format's element size, and
        /// at a minimum exactly the control point format's size.
        ///
        /// This property defaults to `0`, indicating that the control points are tightly-packed.
        #[unsafe(method(controlPointStride))]
        #[unsafe(method_family = none)]
        pub unsafe fn control_point_stride(&self) -> NSUInteger;

        /// Setter for [`controlPointStride`][Self::controlPointStride].
        #[unsafe(method(setControlPointStride:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_control_point_stride(&self, control_point_stride: NSUInteger);

        /// Declares the format of the control points in the buffers that the control point buffers reference.
        ///
        /// All keyframes share the same control point format. Defaults to `MTLAttributeFormatFloat3`, representing 3 floating
        /// point values tightly packed.
        #[unsafe(method(controlPointFormat))]
        #[unsafe(method_family = none)]
        pub unsafe fn control_point_format(&self) -> MTLAttributeFormat;

        /// Setter for [`controlPointFormat`][Self::controlPointFormat].
        #[unsafe(method(setControlPointFormat:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_control_point_format(&self, control_point_format: MTLAttributeFormat);

        /// Assigns a reference to a buffer containing, in turn, references to curve radii buffers.
        ///
        /// This property references a buffer that conceptually represents an array with one entry for each keyframe in the
        /// motion animation. Each one of these entries consists of a ``MTL4BufferRange`` that, in turn, references a
        /// buffer containing the radii corresponding to the keyframe.
        ///
        /// Metal interpolates curve radii according to the basis function you specify via ``curveBasis``.
        ///
        /// You are responsible for ensuring the type of each radius matches the type property ``radiusFormat`` specifies,
        /// that each radius is at least zero, and that the buffer address of the top-level buffer, as well as of buffer
        /// it references, is not zero.
        #[unsafe(method(radiusBuffers))]
        #[unsafe(method_family = none)]
        pub unsafe fn radius_buffers(&self) -> MTL4BufferRange;

        /// Setter for [`radiusBuffers`][Self::radiusBuffers].
        #[unsafe(method(setRadiusBuffers:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_radius_buffers(&self, radius_buffers: MTL4BufferRange);

        /// Sets the format of the radii in the radius buffer.
        ///
        /// Defaults to  `MTLAttributeFormatFloat`. All keyframes share the same radius format.
        #[unsafe(method(radiusFormat))]
        #[unsafe(method_family = none)]
        pub unsafe fn radius_format(&self) -> MTLAttributeFormat;

        /// Setter for [`radiusFormat`][Self::radiusFormat].
        #[unsafe(method(setRadiusFormat:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_radius_format(&self, radius_format: MTLAttributeFormat);

        /// Sets the stride, in bytes, between radii in the radius buffer.
        ///
        /// You are responsible for ensuring this property is set to a multiple of the size corresponding to the ``radiusFormat``.
        /// All keyframes share the same radius stride.
        ///
        /// This property defaults to `0` bytes, indicating that the radii are tightly packed.
        #[unsafe(method(radiusStride))]
        #[unsafe(method_family = none)]
        pub unsafe fn radius_stride(&self) -> NSUInteger;

        /// Setter for [`radiusStride`][Self::radiusStride].
        #[unsafe(method(setRadiusStride:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_radius_stride(&self, radius_stride: NSUInteger);

        /// Assigns an optional index buffer containing references to control points in the control point buffers.
        ///
        /// All keyframes share the same index buffer, with each index representing the first control point of a curve segment.
        ///
        /// You are responsible for ensuring the buffer address of the range is not zero.
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

        /// Declares the number of curve segments.
        ///
        /// All keyframes have the same number of curve segments.
        #[unsafe(method(segmentCount))]
        #[unsafe(method_family = none)]
        pub unsafe fn segment_count(&self) -> NSUInteger;

        /// Setter for [`segmentCount`][Self::segmentCount].
        #[unsafe(method(setSegmentCount:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_segment_count(&self, segment_count: NSUInteger);

        /// Controls the number of control points per curve segment.
        ///
        /// Valid values for this property are `2`, `3`, or `4`. All keyframes have the same number of control points per curve segment.
        #[unsafe(method(segmentControlPointCount))]
        #[unsafe(method_family = none)]
        pub unsafe fn segment_control_point_count(&self) -> NSUInteger;

        /// Setter for [`segmentControlPointCount`][Self::segmentControlPointCount].
        #[unsafe(method(setSegmentControlPointCount:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_segment_control_point_count(
            &self,
            segment_control_point_count: NSUInteger,
        );

        /// Controls the curve type.
        ///
        /// Defaults to `MTLCurveTypeRound`. All keyframes share the same curve type.
        #[unsafe(method(curveType))]
        #[unsafe(method_family = none)]
        pub unsafe fn curve_type(&self) -> MTLCurveType;

        /// Setter for [`curveType`][Self::curveType].
        #[unsafe(method(setCurveType:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_curve_type(&self, curve_type: MTLCurveType);

        /// Sets the curve basis function, determining how Metal interpolates the control points.
        ///
        /// Defaults to `MTLCurveBasisBSpline`. All keyframes share the same curve basis function.
        #[unsafe(method(curveBasis))]
        #[unsafe(method_family = none)]
        pub unsafe fn curve_basis(&self) -> MTLCurveBasis;

        /// Setter for [`curveBasis`][Self::curveBasis].
        #[unsafe(method(setCurveBasis:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_curve_basis(&self, curve_basis: MTLCurveBasis);

        /// Configures the type of curve end caps.
        ///
        /// Defaults to `MTLCurveEndCapsNone`. All keyframes share the same end cap type.
        #[unsafe(method(curveEndCaps))]
        #[unsafe(method_family = none)]
        pub unsafe fn curve_end_caps(&self) -> MTLCurveEndCaps;

        /// Setter for [`curveEndCaps`][Self::curveEndCaps].
        #[unsafe(method(setCurveEndCaps:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_curve_end_caps(&self, curve_end_caps: MTLCurveEndCaps);
    );
}

/// Methods declared on superclass `NSObject`.
impl MTL4AccelerationStructureMotionCurveGeometryDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
