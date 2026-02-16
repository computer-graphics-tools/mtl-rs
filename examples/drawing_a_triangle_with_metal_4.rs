/// Port of Apple's "Drawing a triangle with Metal 4" sample.
///
/// Renders a colorful, rotating 2D triangle in a window using the standard
/// Metal render pipeline with triple buffering and shared event synchronization.
///
/// This port uses the non-Metal4 code path from the Apple sample (the MetalRenderer
/// fallback), so it works on all Metal-capable Macs. The rendering logic, shaders,
/// and triangle data are identical to the Apple sample.
///
/// Apple sample: https://developer.apple.com/documentation/metal/drawing-a-triangle-with-metal-4
mod common;

use objc2_core_foundation::CGSize;

use metal::prelude::*;
use metal::*;
use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2::{MainThreadMarker, msg_send};
use objc2_app_kit::NSView;
use objc2_quartz_core::CAMetalLayer;
use raw_window_handle::HasWindowHandle;
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowId};

use common::{ExampleContext, compile_library_from_source, new_render_pipeline_state};

const MAX_FRAMES_IN_FLIGHT: usize = 3;

/// VertexData matches Apple's ShaderTypes.h.
#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct VertexData {
    position: [f32; 2],
    _pad: [f32; 2],
    color: [f32; 4],
}

/// Port of Apple's configureVertexDataForBuffer / triangleRedGreenBlue.
fn configure_triangle(rotation_degrees: f32, buffer_ptr: *mut VertexData) {
    let radius: f32 = 350.0;
    let angle0 = rotation_degrees * std::f32::consts::PI / 180.0;
    let angle1 = angle0 + (2.0 * std::f32::consts::PI / 3.0);
    let angle2 = angle0 + (4.0 * std::f32::consts::PI / 3.0);

    let vertices = [
        VertexData {
            position: [radius * angle0.cos(), radius * angle0.sin()],
            _pad: [0.0; 2],
            color: [1.0, 0.0, 0.0, 1.0],
        },
        VertexData {
            position: [radius * angle1.cos(), radius * angle1.sin()],
            _pad: [0.0; 2],
            color: [0.0, 1.0, 0.0, 1.0],
        },
        VertexData {
            position: [radius * angle2.cos(), radius * angle2.sin()],
            _pad: [0.0; 2],
            color: [0.0, 0.0, 1.0, 1.0],
        },
    ];

    unsafe {
        std::ptr::copy_nonoverlapping(vertices.as_ptr(), buffer_ptr, 3);
    }
}

struct Renderer {
    context: ExampleContext,
    layer: Retained<CAMetalLayer>,
    pipeline: Retained<ProtocolObject<dyn MTLRenderPipelineState>>,
    vertex_buffers: Vec<Retained<ProtocolObject<dyn MTLBuffer>>>,
    viewport_size_buffer: Retained<ProtocolObject<dyn MTLBuffer>>,
    shared_event: Retained<ProtocolObject<dyn MTLSharedEvent>>,
    frame_number: u64,
    viewport_size: [u32; 2],
}

impl Renderer {
    fn new(layer: Retained<CAMetalLayer>) -> Result<Self, String> {
        let context = ExampleContext::new()?;

        // Set the Metal device on the layer.
        let _: () = unsafe { msg_send![&*layer, setDevice: &*context.device] };
        let _: () = unsafe { msg_send![&*layer, setPixelFormat: MTLPixelFormat::BGRA8Unorm] };

        let shader_source = r#"
#include <metal_stdlib>
using namespace metal;

struct VertexData {
    float2 position;
    float2 _pad;
    float4 color;
};

struct RasterizerData {
    float4 position [[position]];
    float4 color;
};

vertex RasterizerData
vertexShader(uint vertexID [[vertex_id]],
             constant VertexData *vertexData [[buffer(0)]],
             constant uint2 *viewportSizePointer [[buffer(1)]]) {
    RasterizerData out;
    float2 pixelSpacePosition = vertexData[vertexID].position.xy;
    float2 viewportSize = float2(*viewportSizePointer);
    out.position.xy = pixelSpacePosition / (viewportSize / 2.0);
    out.position.z = 0.0;
    out.position.w = 1.0;
    out.color = vertexData[vertexID].color;
    return out;
}

fragment float4 fragmentShader(RasterizerData in [[stage_in]]) {
    return in.color;
}
"#;

        let library = compile_library_from_source(&context.device, shader_source)?;
        let vertex_fn = library
            .new_function_with_name("vertexShader")
            .ok_or("Missing vertexShader")?;
        let fragment_fn = library
            .new_function_with_name("fragmentShader")
            .ok_or("Missing fragmentShader")?;

        let desc = MTLRenderPipelineDescriptor::new();
        desc.set_vertex_function(Some(&vertex_fn));
        desc.set_fragment_function(Some(&fragment_fn));
        desc.color_attachments()
            .object_at_indexed_subscript(0)
            .set_pixel_format(MTLPixelFormat::BGRA8Unorm);
        let pipeline = new_render_pipeline_state(&context.device, &desc)?;

        let vertex_buffers: Vec<_> = (0..MAX_FRAMES_IN_FLIGHT)
            .map(|_| {
                context
                    .device
                    .new_buffer(
                        std::mem::size_of::<VertexData>() * 3,
                        MTLResourceOptions::STORAGE_MODE_SHARED,
                    )
                    .expect("Failed to create vertex buffer")
            })
            .collect();

        let viewport_size: [u32; 2] = [800, 600];
        let viewport_size_buffer = context
            .device
            .new_buffer_with_data(
                bytemuck::cast_slice(&viewport_size),
                MTLResourceOptions::STORAGE_MODE_SHARED,
            )
            .ok_or("Failed to create viewport size buffer")?;

        let shared_event = context
            .device
            .new_shared_event()
            .ok_or("Failed to create shared event")?;
        shared_event.set_signaled_value(0);

        Ok(Self {
            context,
            layer,
            pipeline,
            vertex_buffers,
            viewport_size_buffer,
            shared_event,
            frame_number: 0,
            viewport_size,
        })
    }

    fn resize(&mut self, width: u32, height: u32) {
        self.viewport_size = [width, height];
        let ptr = self.viewport_size_buffer.contents().as_ptr() as *mut [u32; 2];
        unsafe { *ptr = self.viewport_size };
        self.layer.setDrawableSize(CGSize {
            width: width as f64,
            height: height as f64,
        });
    }

    fn render(&mut self) {
        self.frame_number += 1;
        let frame_index = (self.frame_number as usize) % MAX_FRAMES_IN_FLIGHT;

        // Wait for the frame that was using this buffer slot to finish.
        if self.frame_number > MAX_FRAMES_IN_FLIGHT as u64 {
            // Simple synchronization: wait for the earlier frame's event signal.
            let earlier = self.frame_number - MAX_FRAMES_IN_FLIGHT as u64;
            if self.shared_event.signaled_value() < earlier {
                self.shared_event
                    .wait_until_signaled_value_timeout_ms(earlier, 1000);
            }
        }

        // Update triangle vertex data for this frame's rotation.
        let buffer = &self.vertex_buffers[frame_index];
        let rotation = (self.frame_number % 360) as f32;
        configure_triangle(rotation, buffer.contents().as_ptr() as *mut VertexData);

        // Get next drawable from the layer.
        let drawable: Option<Retained<ProtocolObject<dyn MTLDrawable>>> =
            unsafe { msg_send![&*self.layer, nextDrawable] };
        let drawable = match drawable {
            Some(d) => d,
            None => return,
        };

        let render_pass = MTLRenderPassDescriptor::render_pass_descriptor();
        let color = render_pass
            .color_attachments()
            .object_at_indexed_subscript(0);

        let texture: Option<Retained<ProtocolObject<dyn MTLTexture>>> =
            unsafe { msg_send![&*drawable, texture] };
        let texture = match texture {
            Some(t) => t,
            None => return,
        };
        color.set_texture(Some(&texture));
        color.set_load_action(MTLLoadAction::Clear);
        color.set_clear_color(MTLClearColor {
            red: 0.1,
            green: 0.1,
            blue: 0.15,
            alpha: 1.0,
        });
        color.set_store_action(MTLStoreAction::Store);

        let command_buffer = match self.context.queue.command_buffer() {
            Some(cb) => cb,
            None => return,
        };

        let encoder: Option<Retained<ProtocolObject<dyn MTLRenderCommandEncoder>>> = unsafe {
            msg_send![&*command_buffer, renderCommandEncoderWithDescriptor: &*render_pass]
        };
        let encoder = match encoder {
            Some(e) => e,
            None => return,
        };

        encoder.set_render_pipeline_state(&self.pipeline);

        let viewport = MTLViewport {
            origin_x: 0.0,
            origin_y: 0.0,
            width: self.viewport_size[0] as f64,
            height: self.viewport_size[1] as f64,
            znear: 0.0,
            zfar: 1.0,
        };
        encoder.set_viewport(viewport);

        // Bind vertex data and viewport size buffers.
        let _: () = unsafe {
            msg_send![
                &*encoder,
                setVertexBuffer: &**buffer,
                offset: 0usize,
                atIndex: 0usize
            ]
        };
        let _: () = unsafe {
            msg_send![
                &*encoder,
                setVertexBuffer: &*self.viewport_size_buffer,
                offset: 0usize,
                atIndex: 1usize
            ]
        };

        // Draw the triangle.
        let _: () = unsafe {
            msg_send![
                &*encoder,
                drawPrimitives: MTLPrimitiveType::Triangle,
                vertexStart: 0usize,
                vertexCount: 3usize
            ]
        };

        encoder.end_encoding();

        // Present drawable and signal event for frame pacing.
        let _: () = unsafe { msg_send![&*command_buffer, presentDrawable: &*drawable] };
        let _: () = unsafe {
            msg_send![
                &*command_buffer,
                encodeSignalEvent: &*self.shared_event,
                value: self.frame_number
            ]
        };

        command_buffer.commit();
    }
}

struct App {
    window: Option<Window>,
    renderer: Option<Renderer>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }

        let attrs = Window::default_attributes()
            .with_title("Drawing a Triangle with Metal 4")
            .with_inner_size(winit::dpi::LogicalSize::new(800.0, 600.0));

        let window = event_loop
            .create_window(attrs)
            .expect("Failed to create window");

        let _mtm = MainThreadMarker::new().expect("Must be on the main thread");

        // Set up CAMetalLayer on the window's NSView.
        let layer = CAMetalLayer::new();

        let handle = window.window_handle().expect("No window handle");
        if let raw_window_handle::RawWindowHandle::AppKit(handle) = handle.as_raw() {
            let ns_view: &NSView = unsafe { handle.ns_view.cast().as_ref() };
            // Get the backing scale factor for Retina displays.
            let scale_factor: f64 =
                unsafe { msg_send![&*ns_view.window().unwrap(), backingScaleFactor] };
            layer.setContentsScale(scale_factor);
            unsafe {
                let _: () = msg_send![ns_view, setWantsLayer: true];
                let _: () = msg_send![ns_view, setLayer: &*layer];
            }
        }

        let size = window.inner_size();
        layer.setDrawableSize(CGSize {
            width: size.width as f64,
            height: size.height as f64,
        });

        let mut renderer = Renderer::new(layer).expect("Failed to create renderer");
        renderer.resize(size.width, size.height);
        self.renderer = Some(renderer);
        window.request_redraw();
        self.window = Some(window);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => {
                if let Some(renderer) = &mut self.renderer {
                    renderer.resize(size.width, size.height);
                }
            }
            WindowEvent::RedrawRequested => {
                if let Some(renderer) = &mut self.renderer {
                    renderer.render();
                }
                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            }
            _ => {}
        }
    }
}

fn main() {
    let event_loop = EventLoop::builder()
        .build()
        .expect("Failed to create event loop");

    let mut app = App {
        window: None,
        renderer: None,
    };

    event_loop.run_app(&mut app).expect("Event loop failed");
}
