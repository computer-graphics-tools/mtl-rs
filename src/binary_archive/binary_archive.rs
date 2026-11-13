use std::path::Path;

use objc2::{Message, extern_protocol, msg_send, rc::Retained, runtime::ProtocolObject};
use objc2_foundation::{NSError, NSObjectProtocol, NSString};

use crate::{
    MTLComputePipelineDescriptor, MTLDevice, MTLFunctionDescriptor, MTLLibrary, MTLMeshRenderPipelineDescriptor,
    MTLRenderPipelineDescriptor, MetalError, function_stitching::MTLStitchedLibraryDescriptor,
    render_pipeline::MTLTileRenderPipelineDescriptor, util::file_url,
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
        fn device(&self) -> Retained<ProtocolObject<dyn MTLDevice>>;
    }
);

#[allow(unused)]
pub trait MTLBinaryArchiveExt: MTLBinaryArchive + Message {
    /// A string to help identify this object.
    fn label(&self) -> Option<String>;

    /// Setter for `label`.
    fn set_label(
        &self,
        label: Option<&str>,
    );

    /// Add the function(s) from a compute pipeline state to the archive.
    fn add_compute_pipeline_functions(
        &self,
        descriptor: &MTLComputePipelineDescriptor,
    ) -> Result<(), MetalError>;

    /// Add the function(s) from a render pipeline state to the archive.
    fn add_render_pipeline_functions(
        &self,
        descriptor: &MTLRenderPipelineDescriptor,
    ) -> Result<(), MetalError>;

    /// Add the function(s) from a tile render pipeline state to the archive.
    ///
    /// Availability: tvOS 14.5+
    fn add_tile_render_pipeline_functions(
        &self,
        descriptor: &MTLTileRenderPipelineDescriptor,
    ) -> Result<(), MetalError>;

    /// Add the function(s) from a mesh render pipeline state to the archive.
    ///
    /// Availability: macOS 15.0+, iOS 18.0+
    fn add_mesh_render_pipeline_functions(
        &self,
        descriptor: &MTLMeshRenderPipelineDescriptor,
    ) -> Result<(), MetalError>;

    /// Add the function(s) from a stitched library to the archive.
    ///
    /// Availability: macOS 15.0+, iOS 18.0+
    fn add_library_with_descriptor(
        &self,
        descriptor: &MTLStitchedLibraryDescriptor,
    ) -> Result<(), MetalError>;

    /// Add a `visible` or `intersection` function to the archive.
    ///
    /// Availability: macOS 12.0+, iOS 15.0+
    fn add_function_with_descriptor_library(
        &self,
        descriptor: &MTLFunctionDescriptor,
        library: &ProtocolObject<dyn MTLLibrary>,
    ) -> Result<(), MetalError>;

    /// Write the contents of a `MTLBinaryArchive` to a file path.
    fn serialize_to_path(
        &self,
        path: &Path,
    ) -> Result<(), MetalError>;
}

impl MTLBinaryArchiveExt for ProtocolObject<dyn MTLBinaryArchive> {
    fn label(&self) -> Option<String> {
        let label: Option<Retained<NSString>> = unsafe { msg_send![self, label] };
        label.map(|s| s.to_string())
    }

    fn set_label(
        &self,
        label: Option<&str>,
    ) {
        unsafe {
            let _: () = msg_send![self, setLabel: label.map(NSString::from_str).as_deref()];
        }
    }

    fn add_compute_pipeline_functions(
        &self,
        descriptor: &MTLComputePipelineDescriptor,
    ) -> Result<(), MetalError> {
        let result: Result<(), Retained<NSError>> =
            unsafe { msg_send![self, addComputePipelineFunctionsWithDescriptor: descriptor, error: _] };
        result.map_err(MetalError::from_nserror)
    }

    fn add_render_pipeline_functions(
        &self,
        descriptor: &MTLRenderPipelineDescriptor,
    ) -> Result<(), MetalError> {
        let result: Result<(), Retained<NSError>> =
            unsafe { msg_send![self, addRenderPipelineFunctionsWithDescriptor: descriptor, error: _] };
        result.map_err(MetalError::from_nserror)
    }

    fn add_tile_render_pipeline_functions(
        &self,
        descriptor: &MTLTileRenderPipelineDescriptor,
    ) -> Result<(), MetalError> {
        let result: Result<(), Retained<NSError>> =
            unsafe { msg_send![self, addTileRenderPipelineFunctionsWithDescriptor: descriptor, error: _] };
        result.map_err(MetalError::from_nserror)
    }

    fn add_mesh_render_pipeline_functions(
        &self,
        descriptor: &MTLMeshRenderPipelineDescriptor,
    ) -> Result<(), MetalError> {
        let result: Result<(), Retained<NSError>> =
            unsafe { msg_send![self, addMeshRenderPipelineFunctionsWithDescriptor: descriptor, error: _] };
        result.map_err(MetalError::from_nserror)
    }

    fn add_library_with_descriptor(
        &self,
        descriptor: &MTLStitchedLibraryDescriptor,
    ) -> Result<(), MetalError> {
        let result: Result<(), Retained<NSError>> =
            unsafe { msg_send![self, addLibraryWithDescriptor: descriptor, error: _] };
        result.map_err(MetalError::from_nserror)
    }

    fn add_function_with_descriptor_library(
        &self,
        descriptor: &MTLFunctionDescriptor,
        library: &ProtocolObject<dyn MTLLibrary>,
    ) -> Result<(), MetalError> {
        let result: Result<(), Retained<NSError>> =
            unsafe { msg_send![self, addFunctionWithDescriptor: descriptor, library: library, error: _] };
        result.map_err(MetalError::from_nserror)
    }

    fn serialize_to_path(
        &self,
        path: &Path,
    ) -> Result<(), MetalError> {
        let url = file_url(path, "serializeToURL:error:")?;
        let result: Result<(), Retained<NSError>> = unsafe { msg_send![self, serializeToURL: &*url, error: _] };
        result.map_err(MetalError::from_nserror)
    }
}
