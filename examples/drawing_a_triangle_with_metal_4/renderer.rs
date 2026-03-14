use metal::{prelude::*, *};
use objc2::{msg_send, rc::Retained, runtime::ProtocolObject};
use objc2_core_foundation::CGSize;
use objc2_quartz_core::CAMetalLayer;

use crate::vertex_data::{VertexData, write_rotating_triangle_vertices};

const MAX_FRAMES_IN_FLIGHT: usize = 3;

const SHADER_SOURCE: &str = r#"
#include <metal_stdlib>
using namespace metal;

struct VertexData {
    float2 position;
    float2 _padding;
    float4 color;
};

struct RasterizerData {
    float4 position [[position]];
    float4 color;
};

vertex RasterizerData
vertex_shader(uint vertex_id [[vertex_id]],
              constant VertexData *vertex_data [[buffer(0)]],
              constant uint2 *viewport_size_pointer [[buffer(1)]]) {
    RasterizerData out;
    float2 pixel_space_position = vertex_data[vertex_id].position.xy;
    float2 viewport_size = float2(*viewport_size_pointer);
    out.position.xy = pixel_space_position / (viewport_size / 2.0);
    out.position.z = 0.0;
    out.position.w = 1.0;
    out.color = vertex_data[vertex_id].color;
    return out;
}

fragment float4 fragment_shader(RasterizerData in [[stage_in]]) {
    return in.color;
}
"#;

pub struct TriangleRenderer {
    _device: Retained<ProtocolObject<dyn MTLDevice>>,
    command_queue: Retained<ProtocolObject<dyn MTLCommandQueue>>,
    metal_layer: Retained<CAMetalLayer>,
    render_pipeline_state: Retained<ProtocolObject<dyn MTLRenderPipelineState>>,
    vertex_buffers: Box<[Retained<ProtocolObject<dyn MTLBuffer>>]>,
    viewport_size_buffer: Retained<ProtocolObject<dyn MTLBuffer>>,
    shared_event: Retained<ProtocolObject<dyn MTLSharedEvent>>,
    frame_number: u64,
    viewport_size: [u32; 2],
}

impl TriangleRenderer {
    pub fn new(metal_layer: Retained<CAMetalLayer>) -> Result<Self, String> {
        let device = <dyn MTLDevice>::system_default().ok_or_else(|| "Metal device not found".to_owned())?;
        let command_queue = device.new_command_queue().ok_or_else(|| "Failed to create command queue".to_owned())?;

        // CAMetalLayer uses objc2-metal types which are ABI-compatible but
        // distinct from mtl-rs types, so msg_send is needed for layer setup.
        unsafe {
            let _: () = msg_send![&*metal_layer, setDevice: &*device];
            let _: () = msg_send![&*metal_layer, setPixelFormat: MTLPixelFormat::BGRA8Unorm];
        }

        let shader_library = device
            .new_library_with_source(SHADER_SOURCE, None)
            .map_err(|error| error.localizedDescription().to_string())?;
        let vertex_function = shader_library.new_function_with_name("vertex_shader").ok_or("Missing vertex_shader")?;
        let fragment_function =
            shader_library.new_function_with_name("fragment_shader").ok_or("Missing fragment_shader")?;

        let pipeline_descriptor = MTLRenderPipelineDescriptor::new();
        pipeline_descriptor.set_vertex_function(Some(&vertex_function));
        pipeline_descriptor.set_fragment_function(Some(&fragment_function));
        pipeline_descriptor
            .color_attachments()
            .object_at_indexed_subscript(0)
            .set_pixel_format(MTLPixelFormat::BGRA8Unorm);
        let render_pipeline_state = device
            .new_render_pipeline_state_with_descriptor(&pipeline_descriptor)
            .map_err(|error| error.localizedDescription().to_string())?;

        let vertex_buffers: Box<[_]> = (0..MAX_FRAMES_IN_FLIGHT)
            .map(|_| {
                device
                    .new_buffer(std::mem::size_of::<VertexData>() * 3, MTLResourceOptions::STORAGE_MODE_SHARED)
                    .expect("Failed to create vertex buffer")
            })
            .collect();

        let viewport_size: [u32; 2] = [800, 600];
        let viewport_size_buffer = device
            .new_buffer_with_data(bytemuck::cast_slice(&viewport_size), MTLResourceOptions::STORAGE_MODE_SHARED)
            .ok_or("Failed to create viewport size buffer")?;

        let shared_event = device.new_shared_event().ok_or("Failed to create shared event")?;
        shared_event.set_signaled_value(0);

        Ok(Self {
            _device: device,
            command_queue,
            metal_layer,
            render_pipeline_state,
            vertex_buffers,
            viewport_size_buffer,
            shared_event,
            frame_number: 0,
            viewport_size,
        })
    }

    pub fn resize(
        &mut self,
        width: u32,
        height: u32,
    ) {
        self.viewport_size = [width, height];
        let pointer = self.viewport_size_buffer.contents().as_ptr() as *mut [u32; 2];
        unsafe { *pointer = self.viewport_size };
        self.metal_layer.setDrawableSize(CGSize {
            width: width as f64,
            height: height as f64,
        });
    }

    pub fn render(&mut self) {
        self.frame_number += 1;
        let frame_index = (self.frame_number as usize) % MAX_FRAMES_IN_FLIGHT;

        if self.frame_number > MAX_FRAMES_IN_FLIGHT as u64 {
            let earlier_frame = self.frame_number - MAX_FRAMES_IN_FLIGHT as u64;
            if self.shared_event.signaled_value() < earlier_frame {
                self.shared_event.wait_until_signaled_value_timeout_ms(earlier_frame, 1000);
            }
        }

        let vertex_buffer = &self.vertex_buffers[frame_index];
        let rotation_degrees = (self.frame_number % 360) as f32;
        write_rotating_triangle_vertices(rotation_degrees, vertex_buffer.contents().as_ptr() as *mut VertexData);

        // CAMetalLayer/CAMetalDrawable return objc2-metal types; use msg_send
        // to stay in mtl-rs's type system.
        let drawable: Option<Retained<ProtocolObject<dyn MTLDrawable>>> =
            unsafe { msg_send![&*self.metal_layer, nextDrawable] };
        let drawable = match drawable {
            Some(drawable) => drawable,
            None => return,
        };

        let drawable_texture: Retained<ProtocolObject<dyn MTLTexture>> = unsafe { msg_send![&*drawable, texture] };

        let render_pass_descriptor = MTLRenderPassDescriptor::render_pass_descriptor();
        let color_attachment = render_pass_descriptor.color_attachments().object_at_indexed_subscript(0);

        color_attachment.set_texture(Some(&drawable_texture));
        color_attachment.set_load_action(MTLLoadAction::Clear);
        color_attachment.set_clear_color(MTLClearColor {
            red: 0.1,
            green: 0.1,
            blue: 0.15,
            alpha: 1.0,
        });
        color_attachment.set_store_action(MTLStoreAction::Store);

        let command_buffer = match self.command_queue.command_buffer() {
            Some(command_buffer) => command_buffer,
            None => return,
        };

        let render_command_encoder =
            match command_buffer.render_command_encoder_with_descriptor(&render_pass_descriptor) {
                Some(encoder) => encoder,
                None => return,
            };

        render_command_encoder.set_render_pipeline_state(&self.render_pipeline_state);
        render_command_encoder.set_viewport(MTLViewport {
            origin_x: 0.0,
            origin_y: 0.0,
            width: self.viewport_size[0] as f64,
            height: self.viewport_size[1] as f64,
            znear: 0.0,
            zfar: 1.0,
        });
        render_command_encoder.set_vertex_buffer(Some(vertex_buffer), 0, 0);
        render_command_encoder.set_vertex_buffer(Some(&self.viewport_size_buffer), 0, 1);
        render_command_encoder.draw_primitives(MTLPrimitiveType::Triangle, 0, 3);
        render_command_encoder.end_encoding();

        command_buffer.present_drawable(&drawable);
        command_buffer.encode_signal_event_value(ProtocolObject::from_ref(&*self.shared_event), self.frame_number);
        command_buffer.commit();
    }
}
