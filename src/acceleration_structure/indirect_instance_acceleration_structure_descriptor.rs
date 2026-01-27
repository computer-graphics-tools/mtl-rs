use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
    runtime::{NSObject, ProtocolObject},
};
use objc2_foundation::{CopyingHelper, NSCopying, NSObjectProtocol};

use crate::{
    MTLAccelerationStructureDescriptor, MTLAccelerationStructureInstanceDescriptorType, MTLBuffer,
    MTLMatrixLayout, MTLTransformType,
};

extern_class!(
    /// Descriptor for an instance acceleration structure built with an indirected buffer of instances.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metal/mtlindirectinstanceaccelerationstructuredescriptor?language=objc)
    #[unsafe(super(MTLAccelerationStructureDescriptor, NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLIndirectInstanceAccelerationStructureDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for MTLIndirectInstanceAccelerationStructureDescriptor {}
);

unsafe impl CopyingHelper for MTLIndirectInstanceAccelerationStructureDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLIndirectInstanceAccelerationStructureDescriptor {}
);

impl MTLIndirectInstanceAccelerationStructureDescriptor {
    extern_methods!(
        /// Buffer containing instance descriptors of the type specified by the instanceDescriptorType property
        #[unsafe(method(instanceDescriptorBuffer))]
        #[unsafe(method_family = none)]
        pub fn instance_descriptor_buffer(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn MTLBuffer>>>;

        /// Setter for [`instanceDescriptorBuffer`][Self::instanceDescriptorBuffer].
        #[unsafe(method(setInstanceDescriptorBuffer:))]
        #[unsafe(method_family = none)]
        pub fn set_instance_descriptor_buffer(
            &self,
            instance_descriptor_buffer: Option<&ProtocolObject<dyn MTLBuffer>>,
        );

        /// Offset into the instance descriptor buffer. Must be a multiple of 64 bytes and must be
        /// aligned to the platform's buffer offset alignment.
        #[unsafe(method(instanceDescriptorBufferOffset))]
        #[unsafe(method_family = none)]
        pub fn instance_descriptor_buffer_offset(&self) -> usize;

        /// Setter for [`instanceDescriptorBufferOffset`][Self::instanceDescriptorBufferOffset].
        #[unsafe(method(setInstanceDescriptorBufferOffset:))]
        #[unsafe(method_family = none)]
        pub fn set_instance_descriptor_buffer_offset(
            &self,
            instance_descriptor_buffer_offset: usize,
        );

        /// Stride, in bytes, between instance descriptors in the instance descriptor buffer. Must
        /// be at least the size of the instance descriptor type and must be a multiple of 4 bytes.
        /// Defaults to the size of the instance descriptor type.
        #[unsafe(method(instanceDescriptorStride))]
        #[unsafe(method_family = none)]
        pub fn instance_descriptor_stride(&self) -> usize;

        /// Setter for [`instanceDescriptorStride`][Self::instanceDescriptorStride].
        #[unsafe(method(setInstanceDescriptorStride:))]
        #[unsafe(method_family = none)]
        pub fn set_instance_descriptor_stride(&self, instance_descriptor_stride: usize);

        /// Maximum number of instance descriptors
        #[unsafe(method(maxInstanceCount))]
        #[unsafe(method_family = none)]
        pub fn max_instance_count(&self) -> usize;

        /// Setter for [`maxInstanceCount`][Self::maxInstanceCount].
        #[unsafe(method(setMaxInstanceCount:))]
        #[unsafe(method_family = none)]
        pub fn set_max_instance_count(&self, max_instance_count: usize);

        /// Buffer containing the instance count as a uint32_t value. Value at build time
        /// must be less than or equal to maxInstanceCount.
        #[unsafe(method(instanceCountBuffer))]
        #[unsafe(method_family = none)]
        pub fn instance_count_buffer(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn MTLBuffer>>>;

        /// Setter for [`instanceCountBuffer`][Self::instanceCountBuffer].
        #[unsafe(method(setInstanceCountBuffer:))]
        #[unsafe(method_family = none)]
        pub fn set_instance_count_buffer(
            &self,
            instance_count_buffer: Option<&ProtocolObject<dyn MTLBuffer>>,
        );

        /// Offset into the instance count buffer. Must be a multiple of 4 bytes and must be
        /// aligned to the platform's buffer offset alignment.
        #[unsafe(method(instanceCountBufferOffset))]
        #[unsafe(method_family = none)]
        pub fn instance_count_buffer_offset(&self) -> usize;

        /// Setter for [`instanceCountBufferOffset`][Self::instanceCountBufferOffset].
        #[unsafe(method(setInstanceCountBufferOffset:))]
        #[unsafe(method_family = none)]
        pub fn set_instance_count_buffer_offset(&self, instance_count_buffer_offset: usize);

        /// Type of instance descriptor in the instance descriptor buffer. Defaults to
        /// MTLAccelerationStructureInstanceDescriptorTypeIndirect. Must be
        /// MTLAccelerationStructureInstanceDescriptorTypeIndirect or
        /// MTLAccelerationStructureInstanceDescriptorTypeIndirectMotion.
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

        /// Buffer containing transformation information for motion
        #[unsafe(method(motionTransformBuffer))]
        #[unsafe(method_family = none)]
        pub fn motion_transform_buffer(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn MTLBuffer>>>;

        /// Setter for [`motionTransformBuffer`][Self::motionTransformBuffer].
        #[unsafe(method(setMotionTransformBuffer:))]
        #[unsafe(method_family = none)]
        pub fn set_motion_transform_buffer(
            &self,
            motion_transform_buffer: Option<&ProtocolObject<dyn MTLBuffer>>,
        );

        /// Offset into the instance motion descriptor buffer. Must be a multiple of 64 bytes and
        /// must be aligned to the platform's buffer offset alignment.
        #[unsafe(method(motionTransformBufferOffset))]
        #[unsafe(method_family = none)]
        pub fn motion_transform_buffer_offset(&self) -> usize;

        /// Setter for [`motionTransformBufferOffset`][Self::motionTransformBufferOffset].
        #[unsafe(method(setMotionTransformBufferOffset:))]
        #[unsafe(method_family = none)]
        pub fn set_motion_transform_buffer_offset(
            &self,
            motion_transform_buffer_offset: usize,
        );

        /// Maximum number of motion transforms
        #[unsafe(method(maxMotionTransformCount))]
        #[unsafe(method_family = none)]
        pub fn max_motion_transform_count(&self) -> usize;

        /// Setter for [`maxMotionTransformCount`][Self::maxMotionTransformCount].
        #[unsafe(method(setMaxMotionTransformCount:))]
        #[unsafe(method_family = none)]
        pub fn set_max_motion_transform_count(&self, max_motion_transform_count: usize);

        /// Buffer containing the motion transform count as a uint32_t value. Value at build time
        /// must be less than or equal to maxMotionTransformCount.
        #[unsafe(method(motionTransformCountBuffer))]
        #[unsafe(method_family = none)]
        pub fn motion_transform_count_buffer(
            &self,
        ) -> Option<Retained<ProtocolObject<dyn MTLBuffer>>>;

        /// Setter for [`motionTransformCountBuffer`][Self::motionTransformCountBuffer].
        #[unsafe(method(setMotionTransformCountBuffer:))]
        #[unsafe(method_family = none)]
        pub fn set_motion_transform_count_buffer(
            &self,
            motion_transform_count_buffer: Option<&ProtocolObject<dyn MTLBuffer>>,
        );

        /// Offset into the motion transform count buffer. Must be a multiple of 4 bytes and must be
        /// aligned to the platform's buffer offset alignment.
        #[unsafe(method(motionTransformCountBufferOffset))]
        #[unsafe(method_family = none)]
        pub fn motion_transform_count_buffer_offset(&self) -> usize;

        /// Setter for [`motionTransformCountBufferOffset`][Self::motionTransformCountBufferOffset].
        #[unsafe(method(setMotionTransformCountBufferOffset:))]
        #[unsafe(method_family = none)]
        pub fn set_motion_transform_count_buffer_offset(
            &self,
            motion_transform_count_buffer_offset: usize,
        );

        /// Matrix layout of the transformation matrices in the instance descriptors
        /// in the instance descriptor buffer and the transformation matrices in the
        /// transformation matrix buffer. Defaults to MTLMatrixLayoutColumnMajor.
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

        /// Type of motion transforms. Defaults to MTLTransformTypePackedFloat4x3.
        #[unsafe(method(motionTransformType))]
        #[unsafe(method_family = none)]
        pub fn motion_transform_type(&self) -> MTLTransformType;

        /// Setter for [`motionTransformType`][Self::motionTransformType].
        #[unsafe(method(setMotionTransformType:))]
        #[unsafe(method_family = none)]
        pub fn set_motion_transform_type(&self, motion_transform_type: MTLTransformType);

        /// Motion transform stride. Defaults to 0, indicating that transforms are tightly packed according to the
        /// motion transform type.
        #[unsafe(method(motionTransformStride))]
        #[unsafe(method_family = none)]
        pub fn motion_transform_stride(&self) -> usize;

        /// Setter for [`motionTransformStride`][Self::motionTransformStride].
        #[unsafe(method(setMotionTransformStride:))]
        #[unsafe(method_family = none)]
        pub fn set_motion_transform_stride(&self, motion_transform_stride: usize);

        #[unsafe(method(descriptor))]
        #[unsafe(method_family = none)]
        pub fn descriptor() -> Retained<Self>;
    );
}

/// Methods declared on superclass `NSObject`.
impl MTLIndirectInstanceAccelerationStructureDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub fn new() -> Retained<Self>;
    );
}
