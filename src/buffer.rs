use std::{ops::Range, os::raw::c_void, ptr::NonNull};

use objc2::{Message, extern_protocol, msg_send, rc::Retained, runtime::ProtocolObject};
use objc2_foundation::{NSError, NSRange, NSString};

use crate::{
    MTLBufferSparseTier, MTLDevice, MTLResource, MTLTensor, MTLTexture, MTLTextureDescriptor,
    tensor::MTLTensorDescriptor,
};

extern_protocol!(
    /// A typeless allocation accessible by both the CPU and the GPU (MTLDevice) or by only the GPU when the storage mode is
    /// MTLResourceStorageModePrivate.
    ///
    ///
    /// Unlike in OpenGL and OpenCL, access to buffers is not synchronized.  The caller may use the CPU to modify the data at any time
    /// but is also responsible for ensuring synchronization and coherency.
    ///
    /// The contents become undefined if both the CPU and GPU write to the same buffer without a synchronizing action between those writes.
    /// This is true even when the regions written do not overlap.
    ///
    /// Availability: macOS 10.11+, iOS 8.0+
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metal/mtlbuffer?language=objc)
    pub unsafe trait MTLBuffer: MTLResource {
        /// The length of the buffer in bytes.
        #[unsafe(method(length))]
        #[unsafe(method_family = none)]
        fn length(&self) -> usize;

        /// Returns the data pointer of this buffer's shared copy.
        #[unsafe(method(contents))]
        #[unsafe(method_family = none)]
        fn contents(&self) -> NonNull<c_void>;

        /// Create a 2D texture or texture buffer that shares storage with this buffer.
        #[unsafe(method(newTextureWithDescriptor:offset:bytesPerRow:))]
        #[unsafe(method_family = new)]
        fn new_texture(
            &self,
            descriptor: &MTLTextureDescriptor,
            offset: usize,
            bytes_per_row: usize,
        ) -> Option<Retained<ProtocolObject<dyn MTLTexture>>>;

        /// Removes all debug markers from a buffer.
        #[unsafe(method(removeAllDebugMarkers))]
        #[unsafe(method_family = none)]
        fn remove_all_debug_markers(&self);

        /// For Metal buffer objects that are remote views, this returns the buffer associated with the storage on the originating device.
        #[unsafe(method(remoteStorageBuffer))]
        #[unsafe(method_family = none)]
        fn remote_storage_buffer(&self) -> Option<Retained<ProtocolObject<dyn MTLBuffer>>>;

        /// On Metal devices that support peer to peer transfers, this method is used to create a remote buffer view on another device
        /// within the peer group.  The receiver must use MTLStorageModePrivate or be backed by an IOSurface.
        #[unsafe(method(newRemoteBufferViewForDevice:))]
        #[unsafe(method_family = new)]
        fn new_remote_buffer_view_for_device(
            &self,
            device: &ProtocolObject<dyn MTLDevice>,
        ) -> Option<Retained<ProtocolObject<dyn MTLBuffer>>>;

        /// Represents the GPU virtual address of a buffer resource
        #[unsafe(method(gpuAddress))]
        #[unsafe(method_family = none)]
        fn gpu_address(&self) -> u64;

        /// Query support tier for sparse buffers.
        #[unsafe(method(sparseBufferTier))]
        #[unsafe(method_family = none)]
        fn sparse_buffer_tier(&self) -> MTLBufferSparseTier;

        /// Creates a tensor that shares storage with this buffer.
        ///
        /// - Parameters:
        ///   - descriptor: A description of the properties for the new tensor.
        ///   - offset: Offset into the buffer at which the data of the tensor begins.
        ///   - error: If an error occurs during creation, Metal populates this parameter to provide you information about it.
        ///
        /// If the descriptor specifies `TensorUsage::MACHINE_LEARNING` usage, you need to observe the following restrictions:
        /// * pass in `0` for the `offset` parameter
        /// * set the element stride the descriptor to `1`
        /// * ensure that number of bytes per row is a multiple of `64`
        /// * for dimensions greater than `2`, make sure `strides[dim] = strides[dim -1] * dimensions[dim - 1]`
        #[unsafe(method(newTensorWithDescriptor:offset:error:))]
        #[unsafe(method_family = new)]
        fn new_tensor_with_descriptor_offset_error(
            &self,
            descriptor: &MTLTensorDescriptor,
            offset: usize,
            error: *mut *mut NSError,
        ) -> Option<Retained<ProtocolObject<dyn MTLTensor>>>;
    }
);

pub trait BufferExt: MTLBuffer + Message {
    /// Inform the device of the range of a buffer that the CPU has modified, allowing the implementation to invalidate
    /// its caches of the buffer's content.
    ///
    /// When the application writes to a buffer's sysmem copy via
    /// _contents,_that range of the buffer immediately
    /// becomes undefined for any accesses by the GPU (MTLDevice).  To restore coherency, the buffer modification must be followed
    /// by -didModifyRange:, and then followed by a commit of the MTLCommandBuffer that will access the buffer.
    /// -didModifyRange does not make the contents coherent for any previously committed command buffers.
    /// Note: This method is only required if buffer is created with a storage mode of MTLResourceStorageModeManaged.
    /// It is not valid to invoke this method on buffers of other storage modes.
    ///
    /// Parameter `range`: The range of bytes that have been modified.
    fn did_modify_range(&self, range: Range<usize>);

    /// Adds a marker to a specific range in the buffer.
    /// When inspecting a buffer in the GPU debugging tools the marker will be shown.
    ///
    /// Parameter `marker`: A label used for the marker.
    ///
    /// Parameter `range`: The range of bytes the marker is using.
    fn add_debug_marker(&self, marker: &str, range: Range<usize>);
}

impl BufferExt for ProtocolObject<dyn MTLBuffer> {
    /// Inform the device of the range of a buffer that the CPU has modified, allowing the implementation to invalidate
    /// its caches of the buffer's content.
    ///
    /// When the application writes to a buffer's sysmem copy via `contents`, that range of the buffer immediately
    /// becomes undefined for any accesses by the GPU (MTLDevice). To restore coherency, the buffer modification must be followed
    /// by `didModifyRange:`, and then followed by a commit of the `MTLCommandBuffer` that will access the buffer.
    /// `didModifyRange:` does not make the contents coherent for any previously committed command buffers.
    ///
    /// Note: This method is only required if buffer is created with a storage mode of `MTLResourceStorageModeManaged`.
    /// It is not valid to invoke this method on buffers of other storage modes.
    ///
    /// Availability: macOS 10.11+, Mac Catalyst 13.0+ (unavailable on iOS)
    fn did_modify_range(&self, range: Range<usize>) {
        let _: () = unsafe {
            msg_send![
                self,
                didModifyRange: Into::<NSRange>::into(range),
            ]
        };
    }

    /// Adds a marker to a specific range in the buffer. When inspecting a buffer in GPU debugging tools, the marker will be shown.
    ///
    /// Parameter `marker`: A label used for the marker.
    /// Parameter `range`: The range of bytes the marker is using.
    ///
    /// Availability: macOS 10.12+, iOS 10.0+
    fn add_debug_marker(&self, marker: &str, range: Range<usize>) {
        let _: () = unsafe {
            msg_send![
                self,
                addDebugMarker: &*NSString::from_str(marker),
                range: Into::<NSRange>::into(range),
            ]
        };
    }
}
