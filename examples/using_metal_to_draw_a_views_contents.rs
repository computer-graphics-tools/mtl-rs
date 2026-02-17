/// Port of Apple's "Using Metal to draw a view's contents" sample.
///
/// Demonstrates the minimal setup for a Metal render pass: creating a render
/// pass descriptor, setting a clear color, and encoding an empty render pass.
///
/// Apple sample: https://developer.apple.com/documentation/metal/using-metal-to-draw-a-view-s-contents
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

    // Create a small offscreen texture to serve as the render target.
    let descriptor =
        MTLTextureDescriptor::texture_2d_descriptor_with_pixel_format_width_height_mipmapped(
            MTLPixelFormat::BGRA8Unorm,
            320,
            240,
            false,
        );
    descriptor.set_usage(MTLTextureUsage::RENDER_TARGET);
    descriptor.set_storage_mode(MTLStorageMode::Private);
    let texture = context
        .device
        .new_texture_with_descriptor(&descriptor)
        .ok_or("Failed to create render target texture")?;

    // Set up a render pass that clears to a cycling color (matching Apple's
    // sample which cycles through red values over time).
    let pass = MTLRenderPassDescriptor::render_pass_descriptor();
    let color = pass.color_attachments().object_at_indexed_subscript(0);
    color.set_texture(Some(&texture));
    color.set_load_action(MTLLoadAction::Clear);
    color.set_clear_color(MTLClearColor {
        red: 0.25,
        green: 0.5,
        blue: 1.0,
        alpha: 1.0,
    });
    color.set_store_action(MTLStoreAction::Store);

    // Encode and commit an empty render pass (just the clear).
    let command_buffer = context
        .queue
        .command_buffer()
        .ok_or("Failed to create command buffer")?;
    let encoder = command_buffer
        .render_command_encoder_with_descriptor(&pass)
        .ok_or("Failed to create render command encoder")?;
    encoder.end_encoding();
    command_buffer.commit();
    command_buffer.wait_until_completed();

    println!(
        "Using Metal to draw a view's contents: rendered a clear pass (0.25, 0.5, 1.0) to a 320x240 texture."
    );
    Ok(())
}
