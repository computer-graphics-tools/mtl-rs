/// Port of Apple's "Creating and Sampling Textures" sample.
///
/// Loads a TGA image into a Metal texture and renders it onto a quad
/// using vertex/fragment shaders with texture sampling.
///
/// Apple sample: https://developer.apple.com/documentation/metal/creating-and-sampling-textures
mod common;

use std::ptr::NonNull;

use metal::prelude::*;
use metal::*;
use objc2::{msg_send, rc::Retained, runtime::ProtocolObject};

use common::{
    ExampleContext, TgaImage, compile_library_from_source, new_render_pipeline_state, resource_path,
};

/// Vertex with 2D position and 2D texture coordinate — matches AAPLVertex in Apple sample.
#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 2],
    tex_coord: [f32; 2],
}

fn main() {
    if let Err(error) = run() {
        eprintln!("Example failed: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let context = ExampleContext::new()?;

    // Load the TGA image — port of AAPLImage initWithTGAFileAtLocation.
    let image = TgaImage::load(&resource_path("image.tga"))?;
    println!(
        "Loaded TGA image: {}x{} ({} bytes BGRA)",
        image.width,
        image.height,
        image.data.len()
    );

    // Create a Metal texture from the image data.
    let texture_descriptor =
        MTLTextureDescriptor::texture_2d_descriptor_with_pixel_format_width_height_mipmapped(
            MTLPixelFormat::BGRA8Unorm,
            image.width,
            image.height,
            false,
        );
    texture_descriptor.set_usage(MTLTextureUsage::SHADER_READ);
    texture_descriptor.set_storage_mode(MTLStorageMode::Shared);

    let texture = context
        .device
        .new_texture_with_descriptor(&texture_descriptor)
        .ok_or_else(|| "Failed to create texture".to_owned())?;

    // Copy image data into the texture — matches replaceRegion call in Apple sample.
    let region = MTLRegion {
        origin: MTLOrigin { x: 0, y: 0, z: 0 },
        size: MTLSize {
            width: image.width,
            height: image.height,
            depth: 1,
        },
    };
    let bytes_per_row = 4 * image.width;
    unsafe {
        texture.replace_region(
            region,
            0,
            NonNull::new_unchecked(image.data.as_ptr() as *mut _),
            bytes_per_row,
        );
    }

    // Set up quad vertices with texture coordinates — matches Apple sample's triangleVertices.
    let half = 250.0f32;
    let vertices: [Vertex; 6] = [
        // First triangle.
        Vertex {
            position: [half, -half],
            tex_coord: [1.0, 1.0],
        },
        Vertex {
            position: [-half, -half],
            tex_coord: [0.0, 1.0],
        },
        Vertex {
            position: [-half, half],
            tex_coord: [0.0, 0.0],
        },
        // Second triangle.
        Vertex {
            position: [half, -half],
            tex_coord: [1.0, 1.0],
        },
        Vertex {
            position: [-half, half],
            tex_coord: [0.0, 0.0],
        },
        Vertex {
            position: [half, half],
            tex_coord: [1.0, 0.0],
        },
    ];

    let vertex_buffer = context
        .device
        .new_buffer_with_data(
            bytemuck::cast_slice(&vertices),
            MTLResourceOptions::STORAGE_MODE_SHARED,
        )
        .ok_or_else(|| "Failed to create vertex buffer".to_owned())?;

    // Compile shaders — port of AAPLShaders.metal.
    let shader_source = r#"
#include <metal_stdlib>
using namespace metal;

struct VertexIn {
    float2 position;
    float2 texCoord;
};

struct RasterizerData {
    float4 position [[position]];
    float2 texCoord;
};

vertex RasterizerData
vertexShader(uint vid [[vertex_id]],
             constant VertexIn *vertexArray [[buffer(0)]],
             constant uint2 *viewportSizePointer [[buffer(1)]]) {
    RasterizerData out;
    float2 pixelSpacePosition = vertexArray[vid].position.xy;
    float2 viewportSize = float2(*viewportSizePointer);
    out.position.xy = pixelSpacePosition / (viewportSize / 2.0);
    out.position.z = 0.0;
    out.position.w = 1.0;
    out.texCoord = vertexArray[vid].texCoord;
    return out;
}

fragment float4
samplingShader(RasterizerData in [[stage_in]],
               texture2d<half> colorTexture [[texture(0)]]) {
    constexpr sampler textureSampler(mag_filter::linear, min_filter::linear);
    const half4 colorSample = colorTexture.sample(textureSampler, in.texCoord);
    return float4(colorSample);
}
"#;

    let library = compile_library_from_source(&context.device, shader_source)?;
    let vertex_fn = library
        .new_function_with_name("vertexShader")
        .ok_or_else(|| "Missing vertexShader".to_owned())?;
    let fragment_fn = library
        .new_function_with_name("samplingShader")
        .ok_or_else(|| "Missing samplingShader".to_owned())?;

    let pipeline_descriptor = MTLRenderPipelineDescriptor::new();
    pipeline_descriptor.set_vertex_function(Some(&*vertex_fn));
    pipeline_descriptor.set_fragment_function(Some(&*fragment_fn));
    pipeline_descriptor
        .color_attachments()
        .object_at_indexed_subscript(0)
        .set_pixel_format(MTLPixelFormat::BGRA8Unorm);
    let pipeline = new_render_pipeline_state(&context.device, &pipeline_descriptor)?;

    // Create offscreen render target.
    let width = 512usize;
    let height = 512usize;
    let rt_descriptor =
        MTLTextureDescriptor::texture_2d_descriptor_with_pixel_format_width_height_mipmapped(
            MTLPixelFormat::BGRA8Unorm,
            width,
            height,
            false,
        );
    rt_descriptor.set_usage(MTLTextureUsage::RENDER_TARGET | MTLTextureUsage::SHADER_READ);
    rt_descriptor.set_storage_mode(MTLStorageMode::Shared);
    let render_target = context
        .device
        .new_texture_with_descriptor(&rt_descriptor)
        .ok_or_else(|| "Failed to create render target".to_owned())?;

    // Set up viewport size uniform.
    let viewport_size: [u32; 2] = [width as u32, height as u32];
    let viewport_buffer = context
        .device
        .new_buffer_with_data(
            bytemuck::cast_slice(&viewport_size),
            MTLResourceOptions::STORAGE_MODE_SHARED,
        )
        .ok_or_else(|| "Failed to create viewport buffer".to_owned())?;

    // Render the textured quad.
    let render_pass = MTLRenderPassDescriptor::render_pass_descriptor();
    let color = render_pass
        .color_attachments()
        .object_at_indexed_subscript(0);
    color.set_texture(Some(&*render_target));
    color.set_load_action(MTLLoadAction::Clear);
    color.set_store_action(MTLStoreAction::Store);

    let command_buffer = context
        .queue
        .command_buffer()
        .ok_or_else(|| "Failed to create command buffer".to_owned())?;

    let encoder: Retained<ProtocolObject<dyn MTLRenderCommandEncoder>> =
        unsafe { msg_send![&*command_buffer, renderCommandEncoderWithDescriptor: &*render_pass] };

    encoder.set_render_pipeline_state(&*pipeline);

    let viewport = MTLViewport {
        origin_x: 0.0,
        origin_y: 0.0,
        width: width as f64,
        height: height as f64,
        znear: 0.0,
        zfar: 1.0,
    };
    encoder.set_viewport(viewport);

    let _: () = unsafe {
        msg_send![
            &*encoder,
            setVertexBuffer: &*vertex_buffer,
            offset: 0usize,
            atIndex: 0usize
        ]
    };
    let _: () = unsafe {
        msg_send![
            &*encoder,
            setVertexBuffer: &*viewport_buffer,
            offset: 0usize,
            atIndex: 1usize
        ]
    };
    let _: () = unsafe {
        msg_send![
            &*encoder,
            setFragmentTexture: &*texture,
            atIndex: 0usize
        ]
    };

    let _: () = unsafe {
        msg_send![
            &*encoder,
            drawPrimitives: MTLPrimitiveType::Triangle,
            vertexStart: 0usize,
            vertexCount: 6usize
        ]
    };

    encoder.end_encoding();
    command_buffer.commit();
    command_buffer.wait_until_completed();

    println!("Rendered textured quad to {width}x{height} offscreen texture.");
    Ok(())
}
