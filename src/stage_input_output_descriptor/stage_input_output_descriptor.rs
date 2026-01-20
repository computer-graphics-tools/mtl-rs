use objc2::{extern_class, extern_conformance, extern_methods, rc::Retained, runtime::NSObject};
use objc2_foundation::{CopyingHelper, NSCopying, NSObjectProtocol};

use super::{MTLAttributeDescriptorArray, MTLBufferLayoutDescriptorArray};

extern_class!(
    /// Stage input/output descriptor
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLStageInputOutputDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for MTLStageInputOutputDescriptor {}
);

unsafe impl CopyingHelper for MTLStageInputOutputDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLStageInputOutputDescriptor {}
);

impl MTLStageInputOutputDescriptor {
    extern_methods!(
        #[unsafe(method(stageInputOutputDescriptor))]
        #[unsafe(method_family = none)]
        pub fn stage_input_output_descriptor() -> Retained<MTLStageInputOutputDescriptor>;

        #[unsafe(method(layouts))]
        #[unsafe(method_family = none)]
        pub fn layouts(&self) -> Retained<MTLBufferLayoutDescriptorArray>;

        #[unsafe(method(attributes))]
        #[unsafe(method_family = none)]
        pub fn attributes(&self) -> Retained<MTLAttributeDescriptorArray>;

        #[unsafe(method(indexBufferIndex))]
        #[unsafe(method_family = none)]
        pub fn index_buffer_index(&self) -> usize;

        /// Setter for [`index_buffer_index`][Self::index_buffer_index].
        #[unsafe(method(setIndexBufferIndex:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_index_buffer_index(&self, index_buffer_index: usize);

        #[unsafe(method(reset))]
        #[unsafe(method_family = none)]
        pub fn reset(&self);
    );
}
