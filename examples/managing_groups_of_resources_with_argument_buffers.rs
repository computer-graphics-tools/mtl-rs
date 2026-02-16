mod common;

use metal::prelude::*;
use metal::*;

use common::ExampleContext;

fn main() {
    if let Err(error) = run() {
        eprintln!("Example failed: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let context = ExampleContext::new()?;

    let descriptor = MTLArgumentDescriptor::new();
    descriptor.set_data_type(MTLDataType::Texture);
    descriptor.set_access(MTLBindingAccess::ReadOnly);
    descriptor.set_texture_type(MTLTextureType::Type2D);
    descriptor.set_index(0);

    let encoder = context
        .device
        .new_argument_encoder_with_arguments(&[&descriptor])
        .ok_or_else(|| "Failed to create argument encoder".to_owned())?;

    let argument_buffer = context
        .device
        .new_buffer(
            encoder.encoded_length(),
            MTLResourceOptions::STORAGE_MODE_SHARED,
        )
        .ok_or_else(|| "Failed to allocate argument buffer".to_owned())?;
    encoder.set_argument_buffer(Some(&*argument_buffer), 0);

    let texture_desc =
        MTLTextureDescriptor::texture_2d_descriptor_with_pixel_format_width_height_mipmapped(
            MTLPixelFormat::RGBA8Unorm,
            8,
            8,
            false,
        );
    texture_desc.set_usage(MTLTextureUsage::SHADER_READ);
    texture_desc.set_storage_mode(MTLStorageMode::Shared);
    let texture = context
        .device
        .new_texture_with_descriptor(&texture_desc)
        .ok_or_else(|| "Failed to create texture for argument buffer".to_owned())?;
    encoder.set_texture(Some(&*texture), 0);

    println!("Argument buffers: encoded a texture into an argument buffer.");
    Ok(())
}
