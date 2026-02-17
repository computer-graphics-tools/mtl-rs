mod common;

use metal::prelude::*;
use metal::*;
use objc2::msg_send;

use common::{ExampleContext, compile_library_from_source, new_render_pipeline_state};

fn main() {
    if let Err(error) = run() {
        eprintln!("Example failed: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let context = ExampleContext::new()?;
    let width = 128usize;
    let height = 128usize;
    let sample_count = 4usize;

    let shader_source = r#"
#include <metal_stdlib>
using namespace metal;

struct VSOut {
    float4 position [[position]];
    float4 color;
};

vertex VSOut vs_main(uint vid [[vertex_id]]) {
    float2 positions[3] = {
        float2(-0.8, -0.8),
        float2( 0.0,  0.8),
        float2( 0.8, -0.8)
    };

    float3 colors[3] = {
        float3(1.0, 0.2, 0.2),
        float3(0.2, 1.0, 0.2),
        float3(0.2, 0.2, 1.0)
    };

    VSOut out;
    out.position = float4(positions[vid], 0.0, 1.0);
    out.color = float4(colors[vid], 1.0);
    return out;
}

fragment float4 fs_main(VSOut in [[stage_in]]) {
    return in.color;
}
"#;

    let library = compile_library_from_source(&context.device, shader_source)?;
    let vertex = library
        .new_function_with_name("vs_main")
        .ok_or_else(|| "Missing vs_main".to_owned())?;
    let fragment = library
        .new_function_with_name("fs_main")
        .ok_or_else(|| "Missing fs_main".to_owned())?;

    let pipeline_descriptor = MTLRenderPipelineDescriptor::new();
    pipeline_descriptor.set_vertex_function(Some(&*vertex));
    pipeline_descriptor.set_fragment_function(Some(&*fragment));
    pipeline_descriptor.set_raster_sample_count(sample_count);
    pipeline_descriptor
        .color_attachments()
        .object_at_indexed_subscript(0)
        .set_pixel_format(MTLPixelFormat::BGRA8Unorm);
    let pipeline = new_render_pipeline_state(&context.device, &pipeline_descriptor)?;

    let resolve_descriptor =
        MTLTextureDescriptor::texture_2d_descriptor_with_pixel_format_width_height_mipmapped(
            MTLPixelFormat::BGRA8Unorm,
            width,
            height,
            false,
        );
    resolve_descriptor.set_usage(MTLTextureUsage::SHADER_READ | MTLTextureUsage::RENDER_TARGET);
    resolve_descriptor.set_storage_mode(MTLStorageMode::Shared);
    let resolve_texture = context
        .device
        .new_texture_with_descriptor(&resolve_descriptor)
        .ok_or_else(|| "Failed to allocate resolve texture".to_owned())?;

    let multisample_descriptor = MTLTextureDescriptor::new();
    multisample_descriptor.set_texture_type(MTLTextureType::Type2DMultisample);
    multisample_descriptor.set_pixel_format(MTLPixelFormat::BGRA8Unorm);
    multisample_descriptor.set_width(width);
    multisample_descriptor.set_height(height);
    multisample_descriptor.set_sample_count(sample_count);
    multisample_descriptor.set_storage_mode(MTLStorageMode::Private);
    multisample_descriptor.set_usage(MTLTextureUsage::RENDER_TARGET);
    let multisample_texture = context
        .device
        .new_texture_with_descriptor(&multisample_descriptor)
        .ok_or_else(|| "Failed to allocate multisample texture".to_owned())?;

    let render_pass = MTLRenderPassDescriptor::render_pass_descriptor();
    let color = render_pass
        .color_attachments()
        .object_at_indexed_subscript(0);
    color.set_texture(Some(&*multisample_texture));
    color.set_load_action(MTLLoadAction::Clear);
    color.set_store_action(MTLStoreAction::MultisampleResolve);
    let _: () = unsafe { msg_send![&*color, setResolveTexture: Some(&*resolve_texture)] };

    let command_buffer = context
        .queue
        .command_buffer()
        .ok_or_else(|| "Failed to create command buffer".to_owned())?;
    let encoder = command_buffer
        .render_command_encoder_with_descriptor(&render_pass)
        .expect("render encoder");
    encoder.set_render_pipeline_state(&*pipeline);
    encoder.draw_primitives(MTLPrimitiveType::Triangle, 0, 3);
    encoder.end_encoding();
    command_buffer.commit();
    command_buffer.wait_until_completed();

    println!(
        "MSAA: rendered {sample_count}x multisampled triangle and resolved to {width}x{height} texture."
    );
    Ok(())
}
