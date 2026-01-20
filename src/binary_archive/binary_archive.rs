use objc2::{Message, extern_protocol, msg_send, rc::Retained, runtime::ProtocolObject};
use objc2_foundation::{NSError, NSObjectProtocol, NSString, NSURL};

use crate::{
    MTLComputePipelineDescriptor, MTLFunctionDescriptor, MTLLibrary, MTLRenderPipelineDescriptor,
    function_stitching::MTLStitchedLibraryDescriptor,
};

// Error domain symbol is declared in `binary_archive::types`.

extern_protocol!(
    /// A container of pipeline state descriptors and their associated compiled code.
    ///
    /// Availability: macOS 11.0+, iOS 14.0+
    pub unsafe trait MTLBinaryArchive: NSObjectProtocol {
        /// The device this resource was created against. This resource can only be used with this device.
        #[unsafe(method(device))]
        #[unsafe(method_family = none)]
        fn device(&self) -> Retained<ProtocolObject<dyn crate::MTLDevice>>;

        /// Add the function(s) from a compute pipeline state to the archive.
        ///
        /// If the function fails, `error` will be set to describe the failure. This can be (but is not required to be) an error from the `MTLBinaryArchiveDomain` domain.
        ///
        /// Functions referenced multiple times are silently accepted.
        #[unsafe(method(addComputePipelineFunctionsWithDescriptor:error:_))]
        #[unsafe(method_family = none)]
        fn add_compute_pipeline_functions(
            &self,
            descriptor: &MTLComputePipelineDescriptor,
        ) -> Result<(), Retained<NSError>>;

        /// Add the function(s) from a render pipeline state to the archive.
        ///
        /// If the function fails, `error` will be set to describe the failure. This can be (but is not required to be) an error from the `MTLBinaryArchiveDomain` domain.
        ///
        /// Functions referenced multiple times are silently accepted.
        #[unsafe(method(addRenderPipelineFunctionsWithDescriptor:error:_))]
        #[unsafe(method_family = none)]
        fn add_render_pipeline_functions(
            &self,
            descriptor: &MTLRenderPipelineDescriptor,
        ) -> Result<(), Retained<NSError>>;

        /// Add the function(s) from a tile render pipeline state to the archive.
        ///
        /// If the function fails, `error` will be set to describe the failure. This can be (but is not required to be) an error from the `MTLBinaryArchiveDomain` domain.
        ///
        /// Functions referenced multiple times are silently accepted.
        ///
        /// Availability: tvOS 14.5+
        #[unsafe(method(addTileRenderPipelineFunctionsWithDescriptor:error:_))]
        #[unsafe(method_family = none)]
        fn add_tile_render_pipeline_functions(
            &self,
            descriptor: &crate::render_pipeline::MTLTileRenderPipelineDescriptor,
        ) -> Result<(), Retained<NSError>>;

        /// Add the function(s) from a mesh render pipeline state to the archive.
        ///
        /// If the function fails, `error` will be set to describe the failure. This can be (but is not required to be) an error from the `MTLBinaryArchiveDomain` domain.
        ///
        /// Functions referenced multiple times are silently accepted.
        ///
        /// Availability: macOS 15.0+, iOS 18.0+
        #[unsafe(method(addMeshRenderPipelineFunctionsWithDescriptor:error:_))]
        #[unsafe(method_family = none)]
        fn add_mesh_render_pipeline_functions(
            &self,
            descriptor: &crate::MTL4MeshRenderPipelineDescriptor,
        ) -> Result<(), Retained<NSError>>;

        /// Add the function(s) from a stitched library to the archive.
        ///
        /// If the function fails, `error` will be set to describe the failure. This can be (but is not required to be) an error from the `MTLBinaryArchiveDomain` domain.
        ///
        /// Functions referenced multiple times are silently accepted.
        ///
        /// Availability: macOS 15.0+, iOS 18.0+
        #[unsafe(method(addLibraryWithDescriptor:error:_))]
        #[unsafe(method_family = none)]
        unsafe fn add_library_with_descriptor(
            &self,
            descriptor: &MTLStitchedLibraryDescriptor,
        ) -> Result<(), Retained<NSError>>;

        /// Write the contents of a MTLBinaryArchive to a file.
        ///
        /// Persisting the archive to a file allows opening the archive on a subsequent instance of the app, making available the contents without recompiling.
        ///
        /// If the function fails, this will be set to describe the failure. This can be (but is not required to be) an error from the `MTLBinaryArchiveDomain` domain. Other possible errors can be file access or I/O related.
        #[unsafe(method(serializeToURL:error:_))]
        #[unsafe(method_family = none)]
        fn serialize_to_url(&self, url: &NSURL) -> Result<(), Retained<NSError>>;

        /// Add a `visible` or `intersection` function to the archive.
        ///
        /// If the function fails, `error` will be set to describe the failure. This can be (but is not required to be) an error from the `MTLBinaryArchiveDomain` domain. Other possible errors can be file access or I/O related.
        ///
        /// Functions referenced multiple times are silently accepted.
        ///
        /// Availability: macOS 12.0+, iOS 15.0+
        #[unsafe(method(addFunctionWithDescriptor:library:error:_))]
        #[unsafe(method_family = none)]
        unsafe fn add_function_with_descriptor_library(
            &self,
            descriptor: &MTLFunctionDescriptor,
            library: &ProtocolObject<dyn MTLLibrary>,
        ) -> Result<(), Retained<NSError>>;
    }
);

#[allow(unused)]
pub trait MTLBinaryArchiveExt: MTLBinaryArchive + Message {
    /// A string to help identify this object.
    fn label(&self) -> Option<String>;

    /// Setter for `label`.
    fn set_label(&self, label: Option<&str>);
}

impl MTLBinaryArchiveExt for ProtocolObject<dyn MTLBinaryArchive> {
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
