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

    let heap_desc = MTLHeapDescriptor::new();
    heap_desc.set_size(1 << 20);
    heap_desc.set_storage_mode(MTLStorageMode::Private);
    let heap = context
        .device
        .new_heap_with_descriptor(&heap_desc)
        .ok_or_else(|| "Failed to create heap".to_owned())?;

    let texture_desc =
        MTLTextureDescriptor::texture_2d_descriptor_with_pixel_format_width_height_mipmapped(
            MTLPixelFormat::RGBA8Unorm,
            16,
            16,
            false,
        );
    texture_desc.set_usage(MTLTextureUsage::SHADER_READ);
    let _texture = heap
        .new_texture(&texture_desc)
        .ok_or_else(|| "Failed to allocate texture from heap".to_owned())?;

    let arg_desc = MTLArgumentDescriptor::new();
    arg_desc.set_data_type(MTLDataType::Texture);
    arg_desc.set_access(MTLBindingAccess::ReadOnly);
    arg_desc.set_texture_type(MTLTextureType::Type2D);
    arg_desc.set_index(0);

    let encoder = context
        .device
        .new_argument_encoder_with_arguments(&[&arg_desc])
        .ok_or_else(|| "Failed to create argument encoder".to_owned())?;

    let argument_buffer = context
        .device
        .new_buffer(
            encoder.encoded_length(),
            MTLResourceOptions::STORAGE_MODE_SHARED,
        )
        .ok_or_else(|| "Failed to allocate argument buffer".to_owned())?;
    encoder.set_argument_buffer(Some(&*argument_buffer), 0);

    println!(
        "Argument buffers with heaps: allocated texture from heap and set up argument encoder."
    );
    Ok(())
}
