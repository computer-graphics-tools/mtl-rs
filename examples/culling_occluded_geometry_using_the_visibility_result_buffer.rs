/// Port of Apple's "Culling occluded geometry using the visibility result buffer" sample.
///
/// Demonstrates creating a visibility result buffer for occlusion queries.
/// The GPU writes fragment counts or boolean visibility results that the CPU
/// reads back to decide whether to render occluded objects.
///
/// Apple sample: https://developer.apple.com/documentation/metal/culling-occluded-geometry-using-the-visibility-result-buffer
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

    // Create a visibility result buffer. Each query result is 8 bytes (u64).
    let num_queries: usize = 16;
    let buffer_size = num_queries * std::mem::size_of::<u64>();
    let visibility_buffer = context
        .device
        .new_buffer(buffer_size, MTLResourceOptions::STORAGE_MODE_SHARED)
        .ok_or("Failed to create visibility result buffer")?;

    // Create a render target texture.
    let tex_desc =
        MTLTextureDescriptor::texture_2d_descriptor_with_pixel_format_width_height_mipmapped(
            MTLPixelFormat::BGRA8Unorm,
            256,
            256,
            false,
        );
    tex_desc.set_usage(MTLTextureUsage::RENDER_TARGET);
    tex_desc.set_storage_mode(MTLStorageMode::Private);
    let texture = context
        .device
        .new_texture_with_descriptor(&tex_desc)
        .ok_or("Failed to create texture")?;

    // Configure a render pass with the visibility result buffer.
    let pass = MTLRenderPassDescriptor::render_pass_descriptor();
    pass.set_visibility_result_buffer(Some(&visibility_buffer));
    let color = pass.color_attachments().object_at_indexed_subscript(0);
    color.set_texture(Some(&texture));
    color.set_load_action(MTLLoadAction::Clear);
    color.set_clear_color(MTLClearColor {
        red: 0.0,
        green: 0.0,
        blue: 0.0,
        alpha: 1.0,
    });
    color.set_store_action(MTLStoreAction::Store);

    // Encode a render pass and set visibility result mode.
    let command_buffer = context
        .queue
        .command_buffer()
        .ok_or("Failed to create command buffer")?;
    let encoder = command_buffer
        .render_command_encoder_with_descriptor(&pass)
        .ok_or("Failed to create render encoder")?;

    // Set counting mode at offset 0 (would count fragments passing depth/stencil).
    encoder.set_visibility_result_mode(MTLVisibilityResultMode::Counting, 0);
    encoder.end_encoding();
    command_buffer.commit();
    command_buffer.wait_until_completed();

    // Read back the visibility result.
    let results: &[u64] = unsafe {
        std::slice::from_raw_parts(
            visibility_buffer.contents().as_ptr() as *const u64,
            num_queries,
        )
    };
    println!("Culling occluded geometry using the visibility result buffer:");
    println!("  Created visibility buffer for {num_queries} queries.");
    println!("  Visibility result mode: Counting");
    println!("  Query 0 result: {} fragments passed", results[0]);
    Ok(())
}
