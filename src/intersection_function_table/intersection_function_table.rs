use core::ptr::NonNull;

use objc2::{extern_protocol, runtime::ProtocolObject};
use objc2_foundation::NSRange;

use super::MTLIntersectionFunctionSignature;
use crate::{
    MTLBuffer, MTLFunctionHandle, MTLResource, MTLVisibleFunctionTable, types::MTLResourceID,
};

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

        /// Safety: `buffers` and `offsets` must be valid pointers.
        #[unsafe(method(setBuffers:offsets:withRange:))]
        #[unsafe(method_family = none)]
        fn set_buffers(
            &self,
            buffers: NonNull<*const ProtocolObject<dyn MTLBuffer>>,
            offsets: NonNull<usize>,
            range: NSRange,
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

        /// Safety: `functions` must be a valid pointer.
        #[unsafe(method(setFunctions:withRange:))]
        #[unsafe(method_family = none)]
        fn set_functions_with_range(
            &self,
            functions: NonNull<*const ProtocolObject<dyn MTLFunctionHandle>>,
            range: NSRange,
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

        /// Safety: `function_tables` must be a valid pointer.
        #[unsafe(method(setVisibleFunctionTables:withBufferRange:))]
        #[unsafe(method_family = none)]
        fn set_visible_function_tables_with_buffer_range(
            &self,
            function_tables: NonNull<*const ProtocolObject<dyn MTLVisibleFunctionTable>>,
            buffer_range: NSRange,
        );
    }
);
