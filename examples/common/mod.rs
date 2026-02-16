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
        let device =
            create_system_default_device().ok_or_else(|| "Metal device not found".to_owned())?;
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

pub fn resource_path(name: &str) -> String {
    format!("{}/examples/resources/{name}", env!("CARGO_MANIFEST_DIR"))
}

/// TGA image data loaded into 32-bit BGRA format (matching MTLPixelFormatBGRA8Unorm).
/// Port of Apple's AAPLImage class from the CreatingAndSamplingTextures sample.
pub struct TgaImage {
    pub width: usize,
    pub height: usize,
    /// Pixel data in BGRA8 format, 4 bytes per pixel.
    pub data: Vec<u8>,
}

impl TgaImage {
    pub fn load(path: &str) -> Result<Self, String> {
        let file_data = std::fs::read(path).map_err(|e| format!("Failed to read {path}: {e}"))?;
        if file_data.len() < 18 {
            return Err("TGA file too small for header".to_owned());
        }

        let id_size = file_data[0] as usize;
        let color_map_type = file_data[1];
        let image_type = file_data[2];
        let width = u16::from_le_bytes([file_data[12], file_data[13]]) as usize;
        let height = u16::from_le_bytes([file_data[14], file_data[15]]) as usize;
        let bits_per_pixel = file_data[16];
        let descriptor = file_data[17];
        let top_origin = (descriptor & 0x20) != 0;
        let right_origin = (descriptor & 0x10) != 0;

        if color_map_type != 0 {
            return Err("TGA color-mapped images not supported".to_owned());
        }
        if image_type != 2 {
            return Err(format!(
                "Only uncompressed BGR(A) TGA supported (type 2), got {image_type}"
            ));
        }
        if bits_per_pixel != 24 && bits_per_pixel != 32 {
            return Err(format!("Unsupported bits per pixel: {bits_per_pixel}"));
        }

        let src_bytes_per_pixel = (bits_per_pixel / 8) as usize;
        let data_offset = 18 + id_size;
        let src_data = &file_data[data_offset..];
        let expected_size = width * height * src_bytes_per_pixel;
        if src_data.len() < expected_size {
            return Err("TGA pixel data truncated".to_owned());
        }

        // Convert to BGRA8.
        let mut bgra = vec![0u8; width * height * 4];
        for y in 0..height {
            let src_row = if top_origin { y } else { height - 1 - y };
            for x in 0..width {
                let src_col = if right_origin { width - 1 - x } else { x };
                let src_idx = (src_row * width + src_col) * src_bytes_per_pixel;
                let dst_idx = (y * width + x) * 4;
                bgra[dst_idx] = src_data[src_idx]; // B
                bgra[dst_idx + 1] = src_data[src_idx + 1]; // G
                bgra[dst_idx + 2] = src_data[src_idx + 2]; // R
                bgra[dst_idx + 3] = if src_bytes_per_pixel == 4 {
                    src_data[src_idx + 3]
                } else {
                    255
                };
            }
        }

        Ok(Self {
            width,
            height,
            data: bgra,
        })
    }
}
