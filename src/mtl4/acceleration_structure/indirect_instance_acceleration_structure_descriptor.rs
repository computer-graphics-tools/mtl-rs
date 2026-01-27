use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
};
use objc2_foundation::{CopyingHelper, NSCopying, NSObject, NSObjectProtocol, NSUInteger};

use crate::*;

extern_class!(
    /// Descriptor for an "indirect" instance acceleration structure that allows providing the instance count and
    /// motion transform count indirectly, through buffer references.
    ///
    /// An instance acceleration structure references other acceleration structures, and provides the ability to
    /// "instantiate" them multiple times, each one with potentially a different transformation matrix.
    ///
    /// You specify the properties of the instances in the acceleration structure this descriptor builds by providing a
    /// buffer of `structs` via its ``instanceDescriptorBuffer`` property.
    ///
    /// Compared to ``MTL4InstanceAccelerationStructureDescriptor``, this descriptor allows you to provide the number
    /// of instances it references indirectly through a buffer reference, as well as the number of motion transforms.
    ///
    /// This enables you to determine these counts indirectly in the GPU timeline via a compute pipeline.
    /// Metal needs only to know the maximum possible number of instances and motion transforms to support,
    /// which you specify via the ``maxInstanceCount`` and ``maxMotionTransformCount`` properties.
    ///
    /// Use a ``MTLResidencySet`` to mark residency of all buffers and acceleration structures this descriptor references
    /// when you build this acceleration structure.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metal/mtl4indirectinstanceaccelerationstructuredescriptor?language=objc)
    #[unsafe(super(
        MTL4AccelerationStructureDescriptor,
        MTLAccelerationStructureDescriptor,
        NSObject
    ))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTL4IndirectInstanceAccelerationStructureDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for MTL4IndirectInstanceAccelerationStructureDescriptor {}
);

unsafe impl CopyingHelper for MTL4IndirectInstanceAccelerationStructureDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTL4IndirectInstanceAccelerationStructureDescriptor {}
);

impl MTL4IndirectInstanceAccelerationStructureDescriptor {
    extern_methods!(
        /// Assigns a reference to a buffer containing instance descriptors for acceleration structures to reference.
        ///
        /// This buffer conceptually represents an array of instance data. The specific format for the structs that comprise
        /// each entry depends on the value of the  ``instanceDescriptorType`` property.
        ///
        /// You are responsible for ensuring the buffer address the range contains is not zero.
        #[unsafe(method(instanceDescriptorBuffer))]
        #[unsafe(method_family = none)]
        pub fn instance_descriptor_buffer(&self) -> MTL4BufferRange;

        /// Setter for [`instanceDescriptorBuffer`][Self::instanceDescriptorBuffer].
        #[unsafe(method(setInstanceDescriptorBuffer:))]
        #[unsafe(method_family = none)]
        pub fn set_instance_descriptor_buffer(
            &self,
            instance_descriptor_buffer: MTL4BufferRange,
        );

        /// Sets the stride, in bytes, between instance descriptors in the instance descriptor buffer.
        ///
        /// You are responsible for ensuring this stride is at least the size of the structure type corresponding to the instance
        /// descriptor type and a multiple of 4 bytes.
        ///
        /// Defaults to `0`, indicating the instance descriptors are tightly packed.
        #[unsafe(method(instanceDescriptorStride))]
        #[unsafe(method_family = none)]
        pub fn instance_descriptor_stride(&self) -> NSUInteger;

        /// Setter for [`instanceDescriptorStride`][Self::instanceDescriptorStride].
        #[unsafe(method(setInstanceDescriptorStride:))]
        #[unsafe(method_family = none)]
        pub fn set_instance_descriptor_stride(&self, instance_descriptor_stride: NSUInteger);

        /// Controls the maximum number of instance descriptors the instance descriptor buffer can reference.
        ///
        /// You are responsible for ensuring that the final number of instances at build time, which you provide indirectly
        /// via a buffer reference in ``instanceCountBuffer``, is less than or equal to this number.
        #[unsafe(method(maxInstanceCount))]
        #[unsafe(method_family = none)]
        pub fn max_instance_count(&self) -> NSUInteger;

        /// Setter for [`maxInstanceCount`][Self::maxInstanceCount].
        #[unsafe(method(setMaxInstanceCount:))]
        #[unsafe(method_family = none)]
        pub fn set_max_instance_count(&self, max_instance_count: NSUInteger);

        /// Provides a reference to a buffer containing the number of instances in the instance descriptor buffer, formatted as a
        /// 32-bit unsigned integer.
        ///
        /// You are responsible for ensuring that the final number of instances at build time, which you provide indirectly
        /// via this buffer reference , is less than or equal to the value of property ``maxInstanceCount``.
        #[unsafe(method(instanceCountBuffer))]
        #[unsafe(method_family = none)]
        pub fn instance_count_buffer(&self) -> MTL4BufferRange;

        /// Setter for [`instanceCountBuffer`][Self::instanceCountBuffer].
        #[unsafe(method(setInstanceCountBuffer:))]
        #[unsafe(method_family = none)]
        pub fn set_instance_count_buffer(&self, instance_count_buffer: MTL4BufferRange);

        /// Controls the type of instance descriptor that the instance descriptor buffer references.
        ///
        /// This value determines the layout Metal expects for the structs the instance descriptor buffer contains.
        ///
        /// Defaults to `MTLAccelerationStructureInstanceDescriptorTypeIndirect`. Valid values for this property are
        /// `MTLAccelerationStructureInstanceDescriptorTypeIndirect` or `MTLAccelerationStructureInstanceDescriptorTypeIndirectMotion`.
        #[unsafe(method(instanceDescriptorType))]
        #[unsafe(method_family = none)]
        pub fn instance_descriptor_type(
            &self,
        ) -> MTLAccelerationStructureInstanceDescriptorType;

        /// Setter for [`instanceDescriptorType`][Self::instanceDescriptorType].
        #[unsafe(method(setInstanceDescriptorType:))]
        #[unsafe(method_family = none)]
        pub fn set_instance_descriptor_type(
            &self,
            instance_descriptor_type: MTLAccelerationStructureInstanceDescriptorType,
        );

        /// A buffer containing transformation information for instance motion keyframes, formatted according
        /// to the motion transform type.
        ///
        /// Each instance can have a different number of keyframes that you configure via individual instance
        /// descriptors.
        ///
        /// You are responsible for ensuring the buffer address the range references is not zero when using motion instance descriptors.
        #[unsafe(method(motionTransformBuffer))]
        #[unsafe(method_family = none)]
        pub fn motion_transform_buffer(&self) -> MTL4BufferRange;

        /// Setter for [`motionTransformBuffer`][Self::motionTransformBuffer].
        #[unsafe(method(setMotionTransformBuffer:))]
        #[unsafe(method_family = none)]
        pub fn set_motion_transform_buffer(&self, motion_transform_buffer: MTL4BufferRange);

        /// Controls the maximum number of motion transforms in the motion transform buffer.
        ///
        /// You are responsible for ensuring that final number of motion transforms at build time that the buffer
        /// ``motionTransformCountBuffer`` references is less than or equal to this number.
        #[unsafe(method(maxMotionTransformCount))]
        #[unsafe(method_family = none)]
        pub fn max_motion_transform_count(&self) -> NSUInteger;

        /// Setter for [`maxMotionTransformCount`][Self::maxMotionTransformCount].
        #[unsafe(method(setMaxMotionTransformCount:))]
        #[unsafe(method_family = none)]
        pub fn set_max_motion_transform_count(&self, max_motion_transform_count: NSUInteger);

        /// Associates a buffer reference containing the number of motion transforms in the motion transform buffer, formatted as a
        /// 32-bit unsigned integer.
        ///
        /// You are responsible for ensuring that the final number of motion transforms at build time in the buffer this property
        /// references is less than or equal to the value of property ``maxMotionTransformCount``.
        #[unsafe(method(motionTransformCountBuffer))]
        #[unsafe(method_family = none)]
        pub fn motion_transform_count_buffer(&self) -> MTL4BufferRange;

        /// Setter for [`motionTransformCountBuffer`][Self::motionTransformCountBuffer].
        #[unsafe(method(setMotionTransformCountBuffer:))]
        #[unsafe(method_family = none)]
        pub fn set_motion_transform_count_buffer(
            &self,
            motion_transform_count_buffer: MTL4BufferRange,
        );

        /// Specifies the layout for the transformation matrices in the instance descriptor buffer and the motion transformation matrix buffer.
        ///
        /// Metal interprets the value of this property as the layout for the buffers that both ``instanceDescriptorBuffer`` and
        /// ``motionTransformBuffer`` reference.
        ///
        /// Defaults to `MTLMatrixLayoutColumnMajor`.
        #[unsafe(method(instanceTransformationMatrixLayout))]
        #[unsafe(method_family = none)]
        pub fn instance_transformation_matrix_layout(&self) -> MTLMatrixLayout;

        /// Setter for [`instanceTransformationMatrixLayout`][Self::instanceTransformationMatrixLayout].
        #[unsafe(method(setInstanceTransformationMatrixLayout:))]
        #[unsafe(method_family = none)]
        pub fn set_instance_transformation_matrix_layout(
            &self,
            instance_transformation_matrix_layout: MTLMatrixLayout,
        );

        /// Sets the type of motion transforms, either as a matrix or individual components.
        ///
        /// Defaults to `MTLTransformTypePackedFloat4x3`. Using a `MTLTransformTypeComponent` allows you to represent the
        /// rotation by a quaternion (instead as of part of the matrix), allowing for correct motion interpolation.
        #[unsafe(method(motionTransformType))]
        #[unsafe(method_family = none)]
        pub fn motion_transform_type(&self) -> MTLTransformType;

        /// Setter for [`motionTransformType`][Self::motionTransformType].
        #[unsafe(method(setMotionTransformType:))]
        #[unsafe(method_family = none)]
        pub fn set_motion_transform_type(&self, motion_transform_type: MTLTransformType);

        /// Sets the stride for motion transform.
        ///
        /// Defaults to `0`, indicating that transforms are tightly packed according to the motion transform type.
        #[unsafe(method(motionTransformStride))]
        #[unsafe(method_family = none)]
        pub fn motion_transform_stride(&self) -> NSUInteger;

        /// Setter for [`motionTransformStride`][Self::motionTransformStride].
        #[unsafe(method(setMotionTransformStride:))]
        #[unsafe(method_family = none)]
        pub fn set_motion_transform_stride(&self, motion_transform_stride: NSUInteger);
    );
}

/// Methods declared on superclass `NSObject`.
impl MTL4IndirectInstanceAccelerationStructureDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub fn new() -> Retained<Self>;
    );
}
