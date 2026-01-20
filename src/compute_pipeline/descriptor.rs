use objc2::{
    extern_class, extern_conformance, extern_methods, msg_send,
    rc::{Allocated, Retained},
    runtime::{NSObject, ProtocolObject},
};
use objc2_foundation::{CopyingHelper, NSCopying, NSObjectProtocol, NSString};

use crate::{
    MTLLinkedFunctions, MTLPipelineBufferDescriptorArray, MTLStageInputOutputDescriptor,
    library::MTLFunction,
};

extern_class!(
    /// Descriptor for creating a `ComputePipelineState`.
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLComputePipelineDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for MTLComputePipelineDescriptor {}
);

unsafe impl CopyingHelper for MTLComputePipelineDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLComputePipelineDescriptor {}
);

impl MTLComputePipelineDescriptor {
    extern_methods!(
        /// The function to use with the `ComputePipelineState`.
        #[unsafe(method(computeFunction))]
        #[unsafe(method_family = none)]
        pub fn compute_function(&self) -> Option<Retained<ProtocolObject<dyn MTLFunction>>>;

        #[unsafe(method(setComputeFunction:))]
        #[unsafe(method_family = none)]
        pub fn set_compute_function(
            &self,
            compute_function: Option<&ProtocolObject<dyn MTLFunction>>,
        );

        /// Optional property. If not set, returns zero.
        #[unsafe(method(maxTotalThreadsPerThreadgroup))]
        #[unsafe(method_family = none)]
        pub fn max_total_threads_per_threadgroup(&self) -> usize;

        #[unsafe(method(setMaxTotalThreadsPerThreadgroup:))]
        #[unsafe(method_family = none)]
        pub fn set_max_total_threads_per_threadgroup(&self, value: usize);

        /// An `StageInputOutputDescriptor` to fetch data from buffers.
        #[unsafe(method(stageInputDescriptor))]
        #[unsafe(method_family = none)]
        pub fn stage_input_descriptor(&self) -> Option<Retained<MTLStageInputOutputDescriptor>>;

        /// This is copied when set.
        #[unsafe(method(setStageInputDescriptor:))]
        #[unsafe(method_family = none)]
        pub fn set_stage_input_descriptor(
            &self,
            descriptor: Option<&MTLStageInputOutputDescriptor>,
        );

        /// Optional properties for each buffer binding used by the compute function.
        #[unsafe(method(buffers))]
        #[unsafe(method_family = none)]
        pub fn buffers(&self) -> Retained<MTLPipelineBufferDescriptorArray>;

        /// This flag makes this pipeline usable with indirect command buffers.
        #[unsafe(method(supportIndirectCommandBuffers))]
        #[unsafe(method_family = none)]
        pub fn support_indirect_command_buffers(&self) -> bool;

        #[unsafe(method(setSupportIndirectCommandBuffers:))]
        #[unsafe(method_family = none)]
        pub fn set_support_indirect_command_buffers(&self, enabled: bool);

        /// Functions to be linked with the pipeline state and accessed from the compute function.
        #[unsafe(method(linkedFunctions))]
        #[unsafe(method_family = none)]
        pub fn linked_functions(&self) -> Option<Retained<MTLLinkedFunctions>>;

        #[unsafe(method(setLinkedFunctions:))]
        #[unsafe(method_family = none)]
        pub fn set_linked_functions(&self, linked: Option<&MTLLinkedFunctions>);

        /// Restore all compute pipeline descriptor properties to their default values.
        #[unsafe(method(reset))]
        #[unsafe(method_family = none)]
        pub fn reset(&self);

        /// Toggle whether Metal Shader Validation is enabled for the pipeline.
        #[unsafe(method(shaderValidation))]
        #[unsafe(method_family = none)]
        pub fn shader_validation(&self) -> crate::pipeline::MTLShaderValidation;

        #[unsafe(method(setShaderValidation:))]
        #[unsafe(method_family = none)]
        pub fn set_shader_validation(&self, value: crate::pipeline::MTLShaderValidation);

        /// Sets the required threads-per-threadgroup during dispatches.
        /// The `threadsPerThreadgroup` argument of any dispatch must match this value if it is set.
        /// Setting this to a size of 0 in every dimension disables this property.
        #[unsafe(method(requiredThreadsPerThreadgroup))]
        #[unsafe(method_family = none)]
        pub fn required_threads_per_threadgroup(&self) -> crate::types::MTLSize;

        #[unsafe(method(setRequiredThreadsPerThreadgroup:))]
        #[unsafe(method_family = none)]
        pub fn set_required_threads_per_threadgroup(&self, size: crate::types::MTLSize);
    );
}

impl MTLComputePipelineDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub fn new() -> Retained<Self>;
    );
}

#[allow(unused)]
impl MTLComputePipelineDescriptor {
    fn label(&self) -> Option<String> {
        let label: Option<Retained<NSString>> = unsafe { msg_send![self, label] };
        label.map(|s| s.to_string())
    }

    fn set_label(&self, label: Option<&str>) {
        unsafe {
            let _: () = msg_send![self, setLabel: label.map(NSString::from_str).as_deref()];
        }
    }
}
