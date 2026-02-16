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

    let texture_descriptor =
        MTLTextureDescriptor::texture_2d_descriptor_with_pixel_format_width_height_mipmapped(
            MTLPixelFormat::BGRA8Unorm,
            96,
            64,
            false,
        );
    texture_descriptor.set_usage(MTLTextureUsage::SHADER_READ | MTLTextureUsage::RENDER_TARGET);
    texture_descriptor.set_storage_mode(MTLStorageMode::Shared);
    let texture = context
        .device
        .new_texture_with_descriptor(&texture_descriptor)
        .ok_or_else(|| "Failed to allocate texture".to_owned())?;

    let pass = MTLRenderPassDescriptor::render_pass_descriptor();
    pass.set_render_target_width(96);
    pass.set_render_target_height(64);
    let color = pass.color_attachments().object_at_indexed_subscript(0);
    color.set_texture(Some(&*texture));
    color.set_load_action(MTLLoadAction::Load);
    color.set_store_action(MTLStoreAction::Store);

    println!("Render pass setup: configured 96x64 render pass descriptor.");
    Ok(())
}
