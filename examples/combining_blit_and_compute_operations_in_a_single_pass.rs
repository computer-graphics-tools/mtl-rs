/// Port of Apple's "Combining blit and compute operations in a single pass" sample.
///
/// Demonstrates combining blit (copy) and compute operations. Creates a source
/// texture, uses a blit encoder to copy it to a destination, then uses a compute
/// encoder to process the result — all coordinated via command buffer ordering.
///
/// Apple sample: https://developer.apple.com/documentation/metal/combining-blit-and-compute-operations-in-a-single-pass
mod common;

use std::ptr::NonNull;

use metal::prelude::*;
use metal::*;

use common::{ExampleContext, compile_library_from_source};

fn main() {
    if let Err(error) = run() {
        eprintln!("Example failed: {error}");
        std::process::exit(1);
    }
}

const SHADER_SOURCE: &str = r#"
#include <metal_stdlib>
using namespace metal;

kernel void invert_colors(texture2d<half, access::read_write> tex [[texture(0)]],
                          uint2 gid [[thread_position_in_grid]])
{
    if (gid.x >= tex.get_width() || gid.y >= tex.get_height()) return;
    half4 color = tex.read(gid);
    tex.write(half4(1.0h - color.r, 1.0h - color.g, 1.0h - color.b, color.a), gid);
}
"#;

fn run() -> Result<(), String> {
    let context = ExampleContext::new()?;

    let width: usize = 64;
    let height: usize = 64;
    let bytes_per_row = width * 4;

    // Create source texture with shared storage so we can fill it from CPU.
    let src_desc =
        MTLTextureDescriptor::texture_2d_descriptor_with_pixel_format_width_height_mipmapped(
            MTLPixelFormat::BGRA8Unorm,
            width,
            height,
            false,
        );
    src_desc.set_usage(MTLTextureUsage::SHADER_READ);
    src_desc.set_storage_mode(MTLStorageMode::Shared);
    let src_texture = context
        .device
        .new_texture_with_descriptor(&src_desc)
        .ok_or("Failed to create source texture")?;

    // Fill source with a solid red color.
    let red_pixel: [u8; 4] = [0, 0, 255, 255]; // BGRA: red
    let row_data: Vec<u8> = red_pixel.repeat(width);
    let region = MTLRegion {
        origin: MTLOrigin { x: 0, y: 0, z: 0 },
        size: MTLSize::new(width, height, 1),
    };
    src_texture.replace_region(
        region,
        0,
        NonNull::new(row_data.as_ptr() as *mut _).unwrap(),
        bytes_per_row,
    );

    // Create destination texture for compute processing.
    let dst_desc =
        MTLTextureDescriptor::texture_2d_descriptor_with_pixel_format_width_height_mipmapped(
            MTLPixelFormat::BGRA8Unorm,
            width,
            height,
            false,
        );
    dst_desc.set_usage(MTLTextureUsage::SHADER_READ | MTLTextureUsage::SHADER_WRITE);
    dst_desc.set_storage_mode(MTLStorageMode::Shared);
    let dst_texture = context
        .device
        .new_texture_with_descriptor(&dst_desc)
        .ok_or("Failed to create destination texture")?;

    // Compile the compute shader.
    let library = compile_library_from_source(&context.device, SHADER_SOURCE)?;
    let function = library
        .new_function_with_name("invert_colors")
        .ok_or("Failed to find invert_colors function")?;
    let pipeline = context
        .device
        .new_compute_pipeline_state_with_function(&function)
        .map_err(|e| format!("Failed to create compute pipeline: {e}"))?;

    // Step 1: Blit (copy) source texture to destination.
    let command_buffer = context
        .queue
        .command_buffer()
        .ok_or("Failed to create command buffer")?;

    let blit = command_buffer
        .blit_command_encoder()
        .ok_or("Failed to create blit encoder")?;
    blit.copy_from_texture_to_texture(
        &src_texture,
        0,
        0,
        MTLOrigin { x: 0, y: 0, z: 0 },
        MTLSize::new(width, height, 1),
        &dst_texture,
        0,
        0,
        MTLOrigin { x: 0, y: 0, z: 0 },
    );
    blit.end_encoding();

    // Step 2: Compute pass to invert colors on the destination texture.
    let compute = command_buffer
        .compute_command_encoder()
        .ok_or("Failed to create compute encoder")?;
    compute.set_compute_pipeline_state(&pipeline);
    compute.set_texture(Some(&dst_texture), 0);

    let threadgroup_size = MTLSize::new(8, 8, 1);
    let grid_size = MTLSize::new(width, height, 1);
    compute.dispatch_threads(grid_size, threadgroup_size);
    compute.end_encoding();

    command_buffer.commit();
    command_buffer.wait_until_completed();

    // Verify: read back a pixel from the destination — should be inverted (cyan).
    let mut pixel: [u8; 4] = [0; 4];
    let one_pixel_region = MTLRegion {
        origin: MTLOrigin { x: 0, y: 0, z: 0 },
        size: MTLSize::new(1, 1, 1),
    };
    dst_texture.get_bytes(
        NonNull::new(pixel.as_mut_ptr() as *mut _).unwrap(),
        bytes_per_row,
        one_pixel_region,
        0,
    );
    // BGRA: inverted red (0,0,255,255) → cyan (255,255,0,255) in BGRA
    println!("Combining blit and compute operations in a single pass:");
    println!("  Copied 64x64 red texture via blit encoder.");
    println!("  Inverted colors via compute encoder.");
    println!(
        "  Result pixel BGRA: ({}, {}, {}, {}) — expected cyan (255, 255, 0, 255).",
        pixel[0], pixel[1], pixel[2], pixel[3]
    );
    Ok(())
}
