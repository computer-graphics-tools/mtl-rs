mod common;

use metal::prelude::*;

use common::{ExampleContext, compile_library_from_source, retained_error_message};

fn main() {
    if let Err(error) = run() {
        eprintln!("Example failed: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let context = ExampleContext::new()?;
    let width = 16usize;
    let height = 16usize;

    let descriptor =
        MTLTextureDescriptor::texture_2d_descriptor_with_pixel_format_width_height_mipmapped(
            MTLPixelFormat::RGBA8Unorm,
            width,
            height,
            false,
        );
    descriptor.set_storage_mode(MTLStorageMode::Shared);
    descriptor.set_usage(MTLTextureUsage::SHADER_READ | MTLTextureUsage::SHADER_WRITE);
    let input = context
        .device
        .new_texture_with_descriptor(&descriptor)
        .ok_or_else(|| "Failed to create input texture".to_owned())?;
    let output = context
        .device
        .new_texture_with_descriptor(&descriptor)
        .ok_or_else(|| "Failed to create output texture".to_owned())?;

    let library = compile_library_from_source(
        &context.device,
        r#"
        #include <metal_stdlib>
        using namespace metal;
        kernel void grayscale(texture2d<float, access::read> in_tex [[texture(0)]],
                              texture2d<float, access::write> out_tex [[texture(1)]],
                              uint2 gid [[thread_position_in_grid]]) {
            float4 color = in_tex.read(gid);
            float gray = dot(color.rgb, float3(0.299, 0.587, 0.114));
            out_tex.write(float4(gray, gray, gray, color.a), gid);
        }
        "#,
    )?;
    let function = library
        .new_function_with_name("grayscale")
        .ok_or_else(|| "Missing grayscale kernel".to_owned())?;
    let pipeline = context
        .device
        .new_compute_pipeline_state_with_function(&*function)
        .map_err(|error| retained_error_message(&error))?;

    let command_buffer = context
        .queue
        .command_buffer()
        .ok_or_else(|| "Failed to create command buffer".to_owned())?;
    let encoder = command_buffer
        .new_compute_command_encoder()
        .ok_or_else(|| "Failed to create compute encoder".to_owned())?;
    encoder.set_compute_pipeline_state(&*pipeline);
    encoder.set_texture(Some(&*input), 0);
    encoder.set_texture(Some(&*output), 1);
    encoder.dispatch_threads(MTLSize::new(width, height, 1), MTLSize::new(8, 8, 1));
    encoder.end_encoding();
    command_buffer.commit();
    command_buffer.wait_until_completed();

    println!("Texture sampling: grayscale compute filter applied to {width}x{height} texture.");
    Ok(())
}
