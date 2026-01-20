use objc2::{
    extern_class, extern_conformance, extern_methods, msg_send,
    rc::{Allocated, Retained},
};
use objc2_foundation::{
    CopyingHelper, NSCopying, NSObject, NSObjectProtocol, NSString, NSUInteger,
};

use crate::*;

extern_class!(
    /// Base class for all Metal 4 acceleration structure geometry descriptors.
    ///
    /// Don't use this class directly. Use one of the derived classes instead.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metal/mtl4accelerationstructuregeometrydescriptor?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTL4AccelerationStructureGeometryDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for MTL4AccelerationStructureGeometryDescriptor {}
);

unsafe impl CopyingHelper for MTL4AccelerationStructureGeometryDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTL4AccelerationStructureGeometryDescriptor {}
);

impl MTL4AccelerationStructureGeometryDescriptor {
    extern_methods!(
        /// Sets the offset that this geometry contributes to determining the intersection function to invoke when a ray intersects it.
        ///
        /// When you perform a ray tracing operation in the Metal Shading Language, and provide the ray intersector object
        /// with an instance of ``MTLIntersectionFunctionTable``, Metal adds this offset to the instance offset from structs such
        /// as:
        ///
        /// - ``MTLAccelerationStructureInstanceDescriptor``
        /// - ``MTLAccelerationStructureUserIDInstanceDescriptor``
        /// - ``MTLAccelerationStructureMotionInstanceDescriptor``
        /// - ``MTLIndirectAccelerationStructureInstanceDescriptor``
        /// - ``MTLIndirectAccelerationStructureMotionInstanceDescriptor``
        ///
        /// The sum of these offsets provides an index into the intersection function table that the ray tracing system uses
        /// to retrieve and invoke the function at this index, allowing you to customize the intersection evaluation process.
        #[unsafe(method(intersectionFunctionTableOffset))]
        #[unsafe(method_family = none)]
        pub unsafe fn intersection_function_table_offset(&self) -> NSUInteger;

        /// Setter for [`intersectionFunctionTableOffset`][Self::intersectionFunctionTableOffset].
        #[unsafe(method(setIntersectionFunctionTableOffset:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_intersection_function_table_offset(
            &self,
            intersection_function_table_offset: NSUInteger,
        );

        /// Provides a hint to Metal that this geometry is opaque, potentially accelerating the ray/primitive intersection process.
        #[unsafe(method(opaque))]
        #[unsafe(method_family = none)]
        pub unsafe fn opaque(&self) -> bool;

        /// Setter for [`opaque`][Self::opaque].
        #[unsafe(method(setOpaque:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_opaque(&self, opaque: bool);

        /// A boolean value that indicates whether the ray-tracing system in Metal allows the invocation of intersection functions
        /// more than once per ray-primitive intersection.
        ///
        /// The property's default value is
        /// <doc
        /// ://com.apple.documentation/documentation/swift/true>.
        #[unsafe(method(allowDuplicateIntersectionFunctionInvocation))]
        #[unsafe(method_family = none)]
        pub unsafe fn allow_duplicate_intersection_function_invocation(&self) -> bool;

        /// Setter for [`allowDuplicateIntersectionFunctionInvocation`][Self::allowDuplicateIntersectionFunctionInvocation].
        #[unsafe(method(setAllowDuplicateIntersectionFunctionInvocation:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_allow_duplicate_intersection_function_invocation(
            &self,
            allow_duplicate_intersection_function_invocation: bool,
        );

        /// Assigns optional buffer containing data to associate with each primitive in this geometry.
        ///
        /// You can use zero as the buffer address in this buffer range.
        #[unsafe(method(primitiveDataBuffer))]
        #[unsafe(method_family = none)]
        pub unsafe fn primitive_data_buffer(&self) -> MTL4BufferRange;

        /// Setter for [`primitiveDataBuffer`][Self::primitiveDataBuffer].
        #[unsafe(method(setPrimitiveDataBuffer:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_primitive_data_buffer(&self, primitive_data_buffer: MTL4BufferRange);

        /// Defines the stride, in bytes, between each primitive's data in the primitive data buffer ``primitiveDataBuffer`` references.
        ///
        /// You are responsible for ensuring the stride is at least ``primitiveDataElementSize`` in size and a multiple of 4 bytes.
        ///
        /// This property defaults to `0` bytes,  which indicates the stride is equal to ``primitiveDataElementSize``.
        #[unsafe(method(primitiveDataStride))]
        #[unsafe(method_family = none)]
        pub unsafe fn primitive_data_stride(&self) -> NSUInteger;

        /// Setter for [`primitiveDataStride`][Self::primitiveDataStride].
        #[unsafe(method(setPrimitiveDataStride:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_primitive_data_stride(&self, primitive_data_stride: NSUInteger);

        /// Sets the size, in bytes, of the data for each primitive in the primitive data buffer ``primitiveDataBuffer`` references.
        ///
        /// This size needs to be at most ``primitiveDataStride`` in size and a multiple of 4 bytes.
        ///
        /// This property defaults to 0 bytes.
        #[unsafe(method(primitiveDataElementSize))]
        #[unsafe(method_family = none)]
        pub unsafe fn primitive_data_element_size(&self) -> NSUInteger;

        /// Setter for [`primitiveDataElementSize`][Self::primitiveDataElementSize].
        #[unsafe(method(setPrimitiveDataElementSize:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_primitive_data_element_size(
            &self,
            primitive_data_element_size: NSUInteger,
        );
    );
}

/// Methods declared on superclass `NSObject`.
impl MTL4AccelerationStructureGeometryDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

impl MTL4AccelerationStructureGeometryDescriptor {
    /// Assigns an optional label you can assign to this geometry for debugging purposes.
    pub fn label(&self) -> Option<String> {
        let label: Option<Retained<NSString>> = unsafe { msg_send![self, label] };
        label.map(|s| s.to_string())
    }

    /// Setter for label.
    pub fn set_label(&self, label: Option<&str>) {
        unsafe {
            let _: () = msg_send![self, setLabel: label.map(NSString::from_str).as_deref()];
        }
    }
}
