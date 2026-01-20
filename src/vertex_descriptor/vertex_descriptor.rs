use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
    runtime::NSObject,
};
use objc2_foundation::{CopyingHelper, NSCopying, NSObjectProtocol};

use super::{
    vertex_buffer_layout_descriptor_array::MTLVertexBufferLayoutDescriptorArray,
    vertex_format::MTLVertexFormat,
};

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlvertexdescriptor?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLVertexDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for MTLVertexDescriptor {}
);

unsafe impl CopyingHelper for MTLVertexDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLVertexDescriptor {}
);

impl MTLVertexDescriptor {
    extern_methods!(
        /// Returns a new default vertex descriptor.
        #[unsafe(method(vertexDescriptor))]
        #[unsafe(method_family = none)]
        pub fn vertex_descriptor() -> Retained<Self>;

        /// Describes the layout of vertex buffer data for each buffer binding.
        #[unsafe(method(layouts))]
        #[unsafe(method_family = none)]
        pub fn layouts(&self) -> Retained<MTLVertexBufferLayoutDescriptorArray>;

        /// Describes the per-attribute format/location mapping.
        #[unsafe(method(attributes))]
        #[unsafe(method_family = none)]
        pub fn attributes(&self) -> Retained<MTLVertexAttributeDescriptorArray>;

        /// Reset to the default (empty) descriptor.
        #[unsafe(method(reset))]
        #[unsafe(method_family = none)]
        pub fn reset(&self);
    );
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlvertexattributedescriptor?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLVertexAttributeDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for MTLVertexAttributeDescriptor {}
);

unsafe impl CopyingHelper for MTLVertexAttributeDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLVertexAttributeDescriptor {}
);

impl MTLVertexAttributeDescriptor {
    extern_methods!(
        /// The attribute data format.
        #[unsafe(method(format))]
        #[unsafe(method_family = none)]
        pub fn format(&self) -> MTLVertexFormat;

        /// Setter for [`format`][Self::format].
        #[unsafe(method(setFormat:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_format(&self, format: MTLVertexFormat);

        /// Byte offset of this attribute within the vertex.
        #[unsafe(method(offset))]
        #[unsafe(method_family = none)]
        pub fn offset(&self) -> usize;

        /// Setter for [`offset`][Self::offset].
        #[unsafe(method(setOffset:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_offset(&self, offset: usize);

        /// The index of the buffer from which this attribute reads.
        #[unsafe(method(bufferIndex))]
        #[unsafe(method_family = none)]
        pub fn buffer_index(&self) -> usize;

        /// Setter for [`bufferIndex`][Self::bufferIndex].
        #[unsafe(method(setBufferIndex:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_buffer_index(&self, buffer_index: usize);
    );
}

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlvertexattributedescriptorarray?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLVertexAttributeDescriptorArray;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLVertexAttributeDescriptorArray {}
);

impl MTLVertexAttributeDescriptorArray {
    extern_methods!(
        #[unsafe(method(objectAtIndexedSubscript:))]
        #[unsafe(method_family = none)]
        pub unsafe fn object_at_indexed_subscript(
            &self,
            index: usize,
        ) -> Retained<MTLVertexAttributeDescriptor>;

        #[unsafe(method(setObject:atIndexedSubscript:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_object_at_indexed_subscript(
            &self,
            attribute_desc: Option<&MTLVertexAttributeDescriptor>,
            index: usize,
        );
    );
}

/// Methods declared on superclass `NSObject`.
impl MTLVertexDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
