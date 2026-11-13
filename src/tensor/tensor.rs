use core::{ffi::c_void, ptr::NonNull};

use objc2::{Message, extern_protocol, msg_send, rc::Retained, runtime::ProtocolObject};
use objc2_foundation::NSArray;

use super::{MTLTensorAuxiliaryPlane, MTLTensorDataType, MTLTensorExtents, MTLTensorPlaneType, MTLTensorUsage};
use crate::{MTLBuffer, MTLResource, MTLResourceID};

extern_protocol!(
    /// A resource representing a multi-dimensional array that you can use with machine learning workloads.
    ///
    /// See also Apple's documentation: `https://developer.apple.com/documentation/metal/mtltensor?language=objc`
    ///
    /// # Safety
    ///
    /// Implementors must be valid Objective-C objects that conform to the `MTLTensor` protocol.
    #[expect(
        clippy::missing_safety_doc,
        reason = "extern_protocol does not attach this safety section to its generated unsafe trait"
    )]
    pub unsafe trait MTLTensor: MTLResource {
        /// A handle that represents the GPU resource, which you can store in an argument buffer.
        #[unsafe(method(gpuResourceID))]
        #[unsafe(method_family = none)]
        fn gpu_resource_id(&self) -> MTLResourceID;

        /// A buffer instance this tensor shares its storage with or nil if this tensor does not wrap an underlying buffer.
        #[unsafe(method(buffer))]
        #[unsafe(method_family = none)]
        fn buffer(&self) -> Option<Retained<ProtocolObject<dyn MTLBuffer>>>;

        /// An offset, in bytes, into the buffer instance this tensor shares its storage with, or zero if this tensor does not wrap an underlying buffer.
        #[unsafe(method(bufferOffset))]
        #[unsafe(method_family = none)]
        fn buffer_offset(&self) -> usize;

        /// An array of strides, in elements, one for each dimension of this tensor.
        ///
        /// This property only applies if this tensor shares its storage with a buffer, otherwise it's nil.
        #[unsafe(method(strides))]
        #[unsafe(method_family = none)]
        fn strides(&self) -> Option<Retained<MTLTensorExtents>>;

        /// An array of sizes, in elements, one for each dimension of this tensor.
        #[unsafe(method(dimensions))]
        #[unsafe(method_family = none)]
        fn dimensions(&self) -> Retained<MTLTensorExtents>;

        /// An underlying data format of this tensor.
        #[unsafe(method(dataType))]
        #[unsafe(method_family = none)]
        fn data_type(&self) -> MTLTensorDataType;

        /// A set of contexts in which you can use this tensor.
        #[unsafe(method(usage))]
        #[unsafe(method_family = none)]
        fn usage(&self) -> MTLTensorUsage;

        /// Replaces the contents of a slice of this tensor with data you provide.
        ///
        /// Parameters:
        ///  - sliceOrigin: An array of offsets, in elements, to the first element of the slice that this method writes data to.
        ///  - sliceDimensions: An array of sizes, in elements, of the slice this method writes data to.
        ///  - bytes: A pointer to bytes of data that this method copies into the slice you specify with `sliceOrigin` and `sliceDimensions`.
        ///  - strides: An array of strides, in elements, that describes the layout of the data in `bytes`. You are responsible for ensuring `strides` meets the following requirements:
        ///    - Elements of `strides` are in monotonically non-decreasing order.
        ///    - For any `i` larger than zero, `strides[i]` is greater than or equal to `strides[i-1] * dimensions[i-1]`.
        ///
        /// `bytes` must point to enough initialized, correctly aligned memory
        /// for the slice described by `slice_dimensions`, `strides`, and this
        /// tensor's data type. The memory must remain readable for the call.
        #[unsafe(method(replaceSliceOrigin:sliceDimensions:withBytes:strides:))]
        #[unsafe(method_family = none)]
        fn replace_slice_origin_slice_dimensions_with_bytes_strides(
            &self,
            slice_origin: &MTLTensorExtents,
            slice_dimensions: &MTLTensorExtents,
            bytes: NonNull<c_void>,
            strides: &MTLTensorExtents,
        );

        /// Copies the data corresponding to a slice of this tensor into a pointer you provide.
        ///
        /// Parameters:
        ///  - bytes: A pointer to bytes of data that this method copies into the slice you specify with `sliceOrigin` and `sliceDimensions`.
        ///  - strides: An array of strides, in elements, that describes the layout of the data in `bytes`. You are responsible for ensuring `strides` meets the following requirements:
        ///    - Elements of `strides` are in monotonically non-decreasing order.
        ///    - For any `i` larger than zero, `strides[i]` is greater than or equal to `strides[i-1] * dimensions[i-1]`.
        ///  - sliceOrigin: An array of offsets, in elements, to the first element of the slice that this method reads data from.
        ///  - sliceDimensions: An array of sizes, in elements, of the slice this method reads data from.
        ///
        /// `bytes` must point to enough writable, correctly aligned memory for
        /// the slice described by `slice_dimensions`, `strides`, and this
        /// tensor's data type. The memory must remain writable for the call.
        #[unsafe(method(getBytes:strides:fromSliceOrigin:sliceDimensions:))]
        #[unsafe(method_family = none)]
        fn get_bytes_strides_from_slice_origin_slice_dimensions(
            &self,
            bytes: NonNull<c_void>,
            strides: &MTLTensorExtents,
            slice_origin: &MTLTensorExtents,
            slice_dimensions: &MTLTensorExtents,
        );

        /// Copies a slice of one tensor plane into a pointer.
        ///
        /// Origins and dimensions for an auxiliary plane use that plane's coordinates after
        /// applying its block factors.
        ///
        /// `bytes` must point to enough writable, correctly aligned memory for
        /// the selected plane slice and remain writable for the call.
        #[unsafe(method(getBytes:strides:fromSliceOrigin:sliceDimensions:plane:))]
        #[unsafe(method_family = none)]
        fn get_bytes_strides_from_slice_origin_slice_dimensions_plane(
            &self,
            bytes: NonNull<c_void>,
            strides: &MTLTensorExtents,
            slice_origin: &MTLTensorExtents,
            slice_dimensions: &MTLTensorExtents,
            plane: MTLTensorPlaneType,
        );

        /// Replaces a slice of one tensor plane with data from a pointer.
        ///
        /// Origins and dimensions for an auxiliary plane use that plane's coordinates after
        /// applying its block factors.
        ///
        /// `bytes` must point to enough initialized, correctly aligned memory
        /// for the selected plane slice and remain readable for the call.
        #[unsafe(method(replaceSliceOrigin:sliceDimensions:plane:withBytes:strides:))]
        #[unsafe(method_family = none)]
        fn replace_slice_origin_slice_dimensions_plane_with_bytes_strides(
            &self,
            slice_origin: &MTLTensorExtents,
            slice_dimensions: &MTLTensorExtents,
            plane: MTLTensorPlaneType,
            bytes: NonNull<c_void>,
            strides: &MTLTensorExtents,
        );
    }
);

/// Rust-native collection accessors for tensors.
pub trait MTLTensorExt: MTLTensor + Message {
    /// The auxiliary planes configured on this tensor.
    ///
    /// The returned slice is empty for a single-plane tensor.
    fn auxiliary_planes(&self) -> Box<[Retained<ProtocolObject<dyn MTLTensorAuxiliaryPlane>>]>;
}

impl MTLTensorExt for ProtocolObject<dyn MTLTensor> {
    fn auxiliary_planes(&self) -> Box<[Retained<ProtocolObject<dyn MTLTensorAuxiliaryPlane>>]> {
        let planes: Retained<NSArray<ProtocolObject<dyn MTLTensorAuxiliaryPlane>>> =
            unsafe { msg_send![self, auxiliaryPlanes] };
        planes.to_vec().into_boxed_slice()
    }
}
