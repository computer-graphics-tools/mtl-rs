/// Port of Apple's "Reading pixel data from a drawable texture" sample.
///
/// Demonstrates rendering to a texture and then reading the pixel data back
/// to the CPU using a blit command encoder to copy from texture to buffer.
///
/// Apple sample: https://developer.apple.com/documentation/metal/reading-pixel-data-from-a-drawable-texture
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

    let width: usize = 64;
    let height: usize = 64;
    let bytes_per_pixel: usize = 4;
    let bytes_per_row = width * bytes_per_pixel;

    // Create a render target texture.
    let tex_desc =
        MTLTextureDescriptor::texture_2d_descriptor_with_pixel_format_width_height_mipmapped(
            MTLPixelFormat::BGRA8Unorm,
            width,
            height,
            false,
        );
    tex_desc.set_usage(MTLTextureUsage::RENDER_TARGET);
    tex_desc.set_storage_mode(MTLStorageMode::Private);
    let texture = context
        .device
        .new_texture_with_descriptor(&tex_desc)
        .ok_or("Failed to create texture")?;

    // Create a shared buffer to receive the pixel data.
    let buffer_size = bytes_per_row * height;
    let readback_buffer = context
        .device
        .new_buffer(buffer_size, MTLResourceOptions::STORAGE_MODE_SHARED)
        .ok_or("Failed to create readback buffer")?;

    // Render a clear pass with a known color.
    let pass = MTLRenderPassDescriptor::render_pass_descriptor();
    let color = pass.color_attachments().object_at_indexed_subscript(0);
    color.set_texture(Some(&texture));
    color.set_load_action(MTLLoadAction::Clear);
    color.set_clear_color(MTLClearColor {
        red: 0.0,
        green: 0.0,
        blue: 1.0,
        alpha: 1.0,
    }); // Blue
    color.set_store_action(MTLStoreAction::Store);

    let command_buffer = context
        .queue
        .command_buffer()
        .ok_or("Failed to create command buffer")?;

    let render_encoder = command_buffer
        .render_command_encoder_with_descriptor(&pass)
        .ok_or("Failed to create render encoder")?;
    render_encoder.end_encoding();

    // Use a blit encoder to copy the texture contents to the readback buffer.
    let blit_encoder = command_buffer
        .blit_command_encoder()
        .ok_or("Failed to create blit encoder")?;
    blit_encoder.copy_from_texture_to_buffer(
        &texture,
        0,
        0,
        MTLOrigin { x: 0, y: 0, z: 0 },
        MTLSize::new(width, height, 1),
        &readback_buffer,
        0,
        bytes_per_row,
        bytes_per_row * height,
    );
    blit_encoder.end_encoding();

    command_buffer.commit();
    command_buffer.wait_until_completed();

    // Read back and verify the pixel data.
    let pixel_data: &[u8] = unsafe {
        std::slice::from_raw_parts(
            readback_buffer.contents().as_ptr() as *const u8,
            buffer_size,
        )
    };

    // Check the first pixel (BGRA format): should be blue = (255, 0, 0, 255).
    let b = pixel_data[0];
    let g = pixel_data[1];
    let r = pixel_data[2];
    let a = pixel_data[3];
    println!(
        "Reading pixel data: first pixel BGRA = ({b}, {g}, {r}, {a}) — expected (255, 0, 0, 255) for blue clear color."
    );

    if b == 255 && g == 0 && r == 0 && a == 255 {
        println!("Pixel readback verified successfully.");
    } else {
        println!("Warning: pixel values differ from expected.");
    }

    Ok(())
}
