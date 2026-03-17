use core::ops::Range;

use objc2::{Message, extern_protocol, msg_send, runtime::ProtocolObject};
use objc2_foundation::NSRange;

use crate::util::option_ref_ptr_cast_const;
use super::MTLIntersectionFunctionSignature;
use crate::{MTLBuffer, MTLFunctionHandle, MTLResource, MTLVisibleFunctionTable, types::MTLResourceID};

extern_protocol!(
    /// Intersection function table
    ///
    /// Availability: macOS 11.0+, iOS 14.0+, tvOS 16.0+
    pub unsafe trait MTLIntersectionFunctionTable: MTLResource {
        #[unsafe(method(setBuffer:offset:atIndex:))]
        #[unsafe(method_family = none)]
        fn set_buffer(
            &self,
            buffer: Option<&ProtocolObject<dyn MTLBuffer>>,
            offset: usize,
            index: usize,
        );

        /// Handle of the GPU resource suitable for storing in an Argument Buffer
        ///
        /// Availability: macOS 13.0+, iOS 16.0+
        #[unsafe(method(gpuResourceID))]
        #[unsafe(method_family = none)]
        fn gpu_resource_id(&self) -> MTLResourceID;

        #[unsafe(method(setFunction:atIndex:))]
        #[unsafe(method_family = none)]
        fn set_function_at_index(
            &self,
            function: Option<&ProtocolObject<dyn MTLFunctionHandle>>,
            index: usize,
        );

        #[unsafe(method(setOpaqueTriangleIntersectionFunctionWithSignature:atIndex:))]
        #[unsafe(method_family = none)]
        fn set_opaque_triangle_intersection_function_with_signature_at_index(
            &self,
            signature: MTLIntersectionFunctionSignature,
            index: usize,
        );

        #[unsafe(method(setOpaqueTriangleIntersectionFunctionWithSignature:withRange:))]
        #[unsafe(method_family = none)]
        fn set_opaque_triangle_intersection_function_with_signature_with_range(
            &self,
            signature: MTLIntersectionFunctionSignature,
            range: NSRange,
        );

        #[unsafe(method(setOpaqueCurveIntersectionFunctionWithSignature:atIndex:))]
        #[unsafe(method_family = none)]
        fn set_opaque_curve_intersection_function_with_signature_at_index(
            &self,
            signature: MTLIntersectionFunctionSignature,
            index: usize,
        );

        #[unsafe(method(setOpaqueCurveIntersectionFunctionWithSignature:withRange:))]
        #[unsafe(method_family = none)]
        fn set_opaque_curve_intersection_function_with_signature_with_range(
            &self,
            signature: MTLIntersectionFunctionSignature,
            range: NSRange,
        );

        #[unsafe(method(setVisibleFunctionTable:atBufferIndex:))]
        #[unsafe(method_family = none)]
        fn set_visible_function_table_at_buffer_index(
            &self,
            function_table: Option<&ProtocolObject<dyn MTLVisibleFunctionTable>>,
            buffer_index: usize,
        );
    }
);

pub trait MTLIntersectionFunctionTableExt: MTLIntersectionFunctionTable + Message {
    /// Set an array of buffers at the given bind point index range.
    fn set_buffers(
        &self,
        buffers: &[Option<&ProtocolObject<dyn MTLBuffer>>],
        offsets: &[usize],
        range: Range<usize>,
    ) where
        Self: Sized,
    {
        assert_eq!(buffers.len(), offsets.len());
        let ptr = option_ref_ptr_cast_const(buffers.as_ptr());
        unsafe { msg_send![self, setBuffers: ptr, offsets: offsets.as_ptr(), withRange: NSRange::from(range)] }
    }

    /// Set an array of functions at the given index range.
    fn set_functions(
        &self,
        functions: &[Option<&ProtocolObject<dyn MTLFunctionHandle>>],
        range: Range<usize>,
    ) where
        Self: Sized,
    {
        let ptr = option_ref_ptr_cast_const(functions.as_ptr());
        unsafe { msg_send![self, setFunctions: ptr, withRange: NSRange::from(range)] }
    }

    /// Set an array of visible function tables at the given buffer index range.
    fn set_visible_function_tables(
        &self,
        tables: &[Option<&ProtocolObject<dyn MTLVisibleFunctionTable>>],
        range: Range<usize>,
    ) where
        Self: Sized,
    {
        let ptr = option_ref_ptr_cast_const(tables.as_ptr());
        unsafe { msg_send![self, setVisibleFunctionTables: ptr, withBufferRange: NSRange::from(range)] }
    }
}

impl<T: MTLIntersectionFunctionTable + Message> MTLIntersectionFunctionTableExt for T {}
