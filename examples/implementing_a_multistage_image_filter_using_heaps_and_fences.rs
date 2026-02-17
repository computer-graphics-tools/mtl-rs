/// Port of Apple's "Implementing a multistage image filter using heaps and fences" sample.
///
/// Same multi-pass Gaussian blur as the events version, but uses MTLFence for
/// GPU-side synchronization between encoder passes within a single command buffer.
///
/// Apple sample: https://developer.apple.com/documentation/metal/implementing-a-multistage-image-filter-using-heaps-and-fences
mod common;

use metal::prelude::*;
use metal::*;

use std::ptr::NonNull;

use common::{ExampleContext, compile_library_from_source, retained_error_message};

const WIDTH: usize = 64;
const HEIGHT: usize = 64;

fn main() {
    if let Err(error) = run() {
        eprintln!("Example failed: {error}");
        std::process::exit(1);
    }
}

const BLUR_SHADER: &str = r#"
#include <metal_stdlib>
using namespace metal;

constant float weights[5] = { 0.06136f, 0.24477f, 0.38774f, 0.24477f, 0.06136f };

kernel void gaussian_blur_h(texture2d<float, access::read>  src [[texture(0)]],
                            texture2d<float, access::write> dst [[texture(1)]],
                            uint2 gid [[thread_position_in_grid]]) {
    if (gid.x >= dst.get_width() || gid.y >= dst.get_height()) return;
    float4 color = float4(0.0f);
    for (int j = -2; j <= 2; j++) {
        uint2 coord = uint2(clamp(int(gid.x) + j, 0, int(src.get_width()) - 1), gid.y);
        color += weights[j + 2] * src.read(coord);
    }
    dst.write(color, gid);
}

kernel void gaussian_blur_v(texture2d<float, access::read>  src [[texture(0)]],
                            texture2d<float, access::write> dst [[texture(1)]],
                            uint2 gid [[thread_position_in_grid]]) {
    if (gid.x >= dst.get_width() || gid.y >= dst.get_height()) return;
    float4 color = float4(0.0f);
    for (int j = -2; j <= 2; j++) {
        uint2 coord = uint2(gid.x, clamp(int(gid.y) + j, 0, int(src.get_height()) - 1));
        color += weights[j + 2] * src.read(coord);
    }
    dst.write(color, gid);
}
"#;

fn run() -> Result<(), String> {
    let context = ExampleContext::new()?;

    let library = compile_library_from_source(&context.device, BLUR_SHADER)?;
    let fn_h = library
        .new_function_with_name("gaussian_blur_h")
        .ok_or("Missing gaussian_blur_h")?;
    let fn_v = library
        .new_function_with_name("gaussian_blur_v")
        .ok_or("Missing gaussian_blur_v")?;
    let pso_h = context
        .device
        .new_compute_pipeline_state_with_function(&fn_h)
        .map_err(|e| retained_error_message(&e))?;
    let pso_v = context
        .device
        .new_compute_pipeline_state_with_function(&fn_v)
        .map_err(|e| retained_error_message(&e))?;

    // Create a heap for intermediate textures.
    let heap_desc = MTLHeapDescriptor::new();
    heap_desc.set_storage_mode(MTLStorageMode::Private);
    heap_desc.set_size(WIDTH * HEIGHT * 8 * 4);
    let heap = context
        .device
        .new_heap_with_descriptor(&heap_desc)
        .ok_or("Failed to create heap")?;

    // Source texture (shared).
    let tex_desc =
        MTLTextureDescriptor::texture_2d_descriptor_with_pixel_format_width_height_mipmapped(
            MTLPixelFormat::RGBA8Unorm,
            WIDTH,
            HEIGHT,
            false,
        );
    tex_desc.set_usage(MTLTextureUsage::SHADER_READ);
    tex_desc.set_storage_mode(MTLStorageMode::Shared);
    let src_texture = context
        .device
        .new_texture_with_descriptor(&tex_desc)
        .ok_or("Failed to create source texture")?;

    // Fill source with a gradient.
    let mut pixels = vec![0u8; WIDTH * HEIGHT * 4];
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let idx = (y * WIDTH + x) * 4;
            pixels[idx] = (x * 255 / WIDTH) as u8;
            pixels[idx + 1] = (y * 255 / HEIGHT) as u8;
            pixels[idx + 2] = 128;
            pixels[idx + 3] = 255;
        }
    }
    let bytes_per_row = WIDTH * 4;
    src_texture.replace_region(
        MTLRegion {
            origin: MTLOrigin { x: 0, y: 0, z: 0 },
            size: MTLSize::new(WIDTH, HEIGHT, 1),
        },
        0,
        NonNull::new(pixels.as_ptr() as *mut _).unwrap(),
        bytes_per_row,
    );

    // Heap textures.
    let heap_tex_desc =
        MTLTextureDescriptor::texture_2d_descriptor_with_pixel_format_width_height_mipmapped(
            MTLPixelFormat::RGBA8Unorm,
            WIDTH,
            HEIGHT,
            false,
        );
    heap_tex_desc.set_usage(MTLTextureUsage::SHADER_READ | MTLTextureUsage::SHADER_WRITE);
    heap_tex_desc.set_storage_mode(MTLStorageMode::Private);
    let intermediate = heap
        .new_texture(&heap_tex_desc)
        .ok_or("Failed to allocate intermediate from heap")?;
    let output = heap
        .new_texture(&heap_tex_desc)
        .ok_or("Failed to allocate output from heap")?;

    // Create a fence for GPU-side synchronization between encoders.
    let fence = context.device.new_fence().ok_or("Failed to create fence")?;

    // All stages in a single command buffer, synchronized by fences.
    let cb = context.queue.command_buffer().ok_or("No command buffer")?;

    // Stage 1: Blit source → intermediate.
    let blit = cb.blit_command_encoder().ok_or("No blit encoder")?;
    blit.copy_from_texture_to_texture(
        &src_texture,
        0,
        0,
        MTLOrigin { x: 0, y: 0, z: 0 },
        MTLSize::new(WIDTH, HEIGHT, 1),
        &intermediate,
        0,
        0,
        MTLOrigin { x: 0, y: 0, z: 0 },
    );
    blit.update_fence(&fence);
    blit.end_encoding();

    // Stage 2: Horizontal blur (intermediate → output).
    let enc_h = cb.compute_command_encoder().ok_or("No compute encoder")?;
    enc_h.wait_for_fence(&fence);
    enc_h.set_compute_pipeline_state(&pso_h);
    enc_h.set_texture(Some(&intermediate), 0);
    enc_h.set_texture(Some(&output), 1);
    enc_h.dispatch_threads(MTLSize::new(WIDTH, HEIGHT, 1), MTLSize::new(16, 16, 1));
    enc_h.update_fence(&fence);
    enc_h.end_encoding();

    // Stage 3: Vertical blur (output → intermediate).
    let enc_v = cb.compute_command_encoder().ok_or("No compute encoder")?;
    enc_v.wait_for_fence(&fence);
    enc_v.set_compute_pipeline_state(&pso_v);
    enc_v.set_texture(Some(&output), 0);
    enc_v.set_texture(Some(&intermediate), 1);
    enc_v.dispatch_threads(MTLSize::new(WIDTH, HEIGHT, 1), MTLSize::new(16, 16, 1));
    enc_v.end_encoding();

    cb.commit();
    cb.wait_until_completed();

    println!("Implementing a multistage image filter using heaps and fences:");
    println!(
        "  Created heap ({} KB) with 2 intermediate textures.",
        heap_desc.size() / 1024
    );
    println!("  Stage 1: Blit copy → update fence");
    println!("  Stage 2: Wait fence → horizontal blur → update fence");
    println!("  Stage 3: Wait fence → vertical blur");
    println!("  3-stage pipeline completed with fence synchronization (single command buffer).");
    Ok(())
}
