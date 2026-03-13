#![allow(dead_code)]

use metal::prelude::*;
use metal::*;
use objc2::{msg_send, rc::Retained, runtime::ProtocolObject};
use objc2_foundation::{NSError, NSString};

pub struct ExampleContext {
    pub device: Retained<ProtocolObject<dyn MTLDevice>>,
    pub queue: Retained<ProtocolObject<dyn MTLCommandQueue>>,
}

impl ExampleContext {
    pub fn new() -> Result<Self, String> {
        let device = <dyn MTLDevice>::system_default()
            .ok_or_else(|| "Metal device not found".to_owned())?;
        let queue = device
            .new_command_queue()
            .ok_or_else(|| "Failed to create command queue".to_owned())?;
        Ok(Self { device, queue })
    }
}

pub fn ns_error_message(error: *mut NSError) -> String {
    if let Some(error) = unsafe { Retained::retain(error) } {
        let description: Retained<NSString> = unsafe { msg_send![&*error, localizedDescription] };
        description.to_string()
    } else {
        "Unknown NSError".to_owned()
    }
}

pub fn retained_error_message(error: &NSError) -> String {
    let description: Retained<NSString> = unsafe { msg_send![error, localizedDescription] };
    description.to_string()
}

pub fn compile_library_from_source(
    device: &ProtocolObject<dyn MTLDevice>,
    source: &str,
) -> Result<Retained<ProtocolObject<dyn MTLLibrary>>, String> {
    let options = MTLCompileOptions::new();
    let source = NSString::from_str(source);
    let mut error: *mut NSError = std::ptr::null_mut();
    let library: Option<Retained<ProtocolObject<dyn MTLLibrary>>> = unsafe {
        msg_send![
            device,
            newLibraryWithSource: &*source,
            options: &*options,
            error: &mut error
        ]
    };

    library.ok_or_else(|| ns_error_message(error))
}

pub fn new_render_pipeline_state(
    device: &ProtocolObject<dyn MTLDevice>,
    descriptor: &MTLRenderPipelineDescriptor,
) -> Result<Retained<ProtocolObject<dyn MTLRenderPipelineState>>, String> {
    let mut error: *mut NSError = std::ptr::null_mut();
    let pipeline: Option<Retained<ProtocolObject<dyn MTLRenderPipelineState>>> = unsafe {
        msg_send![
            device,
            newRenderPipelineStateWithDescriptor: descriptor,
            error: &mut error
        ]
    };

    pipeline.ok_or_else(|| ns_error_message(error))
}

