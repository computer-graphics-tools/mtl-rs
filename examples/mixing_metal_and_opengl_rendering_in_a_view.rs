/// Port of Apple's "Mixing Metal and OpenGL rendering in a view" sample.
///
/// Demonstrates the Metal side of Metal/OpenGL interoperability: rendering
/// a textured quad with alpha blending of two texture layers. In the original
/// sample, one texture comes from OpenGL via CVPixelBuffer — here we simulate
/// the interop by rendering two blended procedural textures.
///
/// Apple sample: https://developer.apple.com/documentation/metal/mixing-metal-and-opengl-rendering-in-a-view
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

struct Renderer {
    context: ExampleContext,
    layer: Retained<CAMetalLayer>,
    pipeline: Retained<ProtocolObject<dyn MTLRenderPipelineState>>,
    vertex_buffer: Retained<ProtocolObject<dyn MTLBuffer>>,
    frame_number: u64,
    width: u32,
    height: u32,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct QuadVertex {
    position: [f32; 2],
    texcoord: [f32; 2],
}

impl Renderer {
    fn new(layer: Retained<CAMetalLayer>) -> Result<Self, String> {
        let context = ExampleContext::new()?;
        let _: () = unsafe { msg_send![&*layer, setDevice: &*context.device] };
        let _: () = unsafe { msg_send![&*layer, setPixelFormat: MTLPixelFormat::BGRA8Unorm] };

        let shader = r#"
#include <metal_stdlib>
using namespace metal;

struct QuadVertex { float2 position; float2 texcoord; };
struct V2F { float4 position [[position]]; float2 texcoord; };

vertex V2F vertex_main(const device QuadVertex* v [[buffer(0)]],
                       constant float& rotation [[buffer(1)]],
                       uint vid [[vertex_id]]) {
    V2F out;
    float c = cos(rotation);
    float s = sin(rotation);
    float2 p = v[vid].position;
    out.position = float4(p.x * c - p.y * s, p.x * s + p.y * c, 0.0, 1.0);
    out.texcoord = v[vid].texcoord;
    return out;
}

fragment half4 fragment_main(V2F in [[stage_in]]) {
    // Simulate two blended texture layers (Metal + "OpenGL" interop texture).
    // Layer 1: checkerboard (simulating the Metal-rendered texture).
    float2 uv1 = in.texcoord * 6.0;
    float check = fmod(floor(uv1.x) + floor(uv1.y), 2.0);
    half3 metalColor = check > 0.5 ? half3(0.2, 0.5, 0.9) : half3(0.1, 0.3, 0.7);

    // Layer 2: gradient (simulating the OpenGL-rendered interop texture).
    half3 glColor = half3(in.texcoord.x, 0.3, 1.0 - in.texcoord.y);

    // Alpha blend the two layers (as the original sample does).
    half alpha = 0.5h;
    half3 blended = mix(metalColor, glColor, alpha);
    return half4(blended, 1.0h);
}
"#;

        let library = compile_library_from_source(&context.device, shader)?;
        let vert = library
            .new_function_with_name("vertex_main")
            .ok_or("Missing vertex_main")?;
        let frag = library
            .new_function_with_name("fragment_main")
            .ok_or("Missing fragment_main")?;

        let desc = MTLRenderPipelineDescriptor::new();
        desc.set_vertex_function(Some(&vert));
        desc.set_fragment_function(Some(&frag));
        desc.color_attachments()
            .object_at_indexed_subscript(0)
            .set_pixel_format(MTLPixelFormat::BGRA8Unorm);
        let pipeline = new_render_pipeline_state(&context.device, &desc)?;

        let verts = [
            QuadVertex {
                position: [-0.8, -0.8],
                texcoord: [0.0, 1.0],
            },
            QuadVertex {
                position: [0.8, -0.8],
                texcoord: [1.0, 1.0],
            },
            QuadVertex {
                position: [0.8, 0.8],
                texcoord: [1.0, 0.0],
            },
            QuadVertex {
                position: [-0.8, -0.8],
                texcoord: [0.0, 1.0],
            },
            QuadVertex {
                position: [0.8, 0.8],
                texcoord: [1.0, 0.0],
            },
            QuadVertex {
                position: [-0.8, 0.8],
                texcoord: [0.0, 0.0],
            },
        ];
        let vertex_buffer = context
            .device
            .new_buffer_with_data(
                bytemuck::cast_slice(&verts),
                MTLResourceOptions::STORAGE_MODE_SHARED,
            )
            .ok_or("vertex buffer")?;

        Ok(Self {
            context,
            layer,
            pipeline,
            vertex_buffer,
            frame_number: 0,
            width: 800,
            height: 600,
        })
    }

    fn resize(&mut self, w: u32, h: u32) {
        self.width = w;
        self.height = h;
        self.layer.setDrawableSize(CGSize {
            width: w as f64,
            height: h as f64,
        });
    }

    fn render(&mut self) {
        self.frame_number += 1;
        let rotation: f32 = self.frame_number as f32 * 0.01;

        let drawable: Option<Retained<ProtocolObject<dyn MTLDrawable>>> =
            unsafe { msg_send![&*self.layer, nextDrawable] };
        let drawable = match drawable {
            Some(d) => d,
            None => return,
        };

        let pass = MTLRenderPassDescriptor::render_pass_descriptor();
        let color = pass.color_attachments().object_at_indexed_subscript(0);
        let texture: Option<Retained<ProtocolObject<dyn MTLTexture>>> =
            unsafe { msg_send![&*drawable, texture] };
        let texture = match texture {
            Some(t) => t,
            None => return,
        };
        color.set_texture(Some(&texture));
        color.set_load_action(MTLLoadAction::Clear);
        color.set_clear_color(MTLClearColor {
            red: 0.15,
            green: 0.15,
            blue: 0.2,
            alpha: 1.0,
        });
        color.set_store_action(MTLStoreAction::Store);

        let cb = match self.context.queue.command_buffer() {
            Some(cb) => cb,
            None => return,
        };
        let enc = cb.render_command_encoder_with_descriptor(&pass);
        let enc = match enc {
            Some(e) => e,
            None => return,
        };

        enc.set_render_pipeline_state(&self.pipeline);
        enc.set_vertex_buffer(Some(&self.vertex_buffer), 0, 0);
        enc.set_vertex_bytes(unsafe { std::ptr::NonNull::new_unchecked(&rotation as *const f32 as *mut std::ffi::c_void) }, 4, 1);
        enc.draw_primitives(MTLPrimitiveType::Triangle, 0, 6);
        enc.end_encoding();

        cb.present_drawable(&drawable);
        cb.commit();
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
            .with_title("Mixing Metal and OpenGL Rendering in a View")
            .with_inner_size(winit::dpi::LogicalSize::new(800.0, 600.0));
        let window = event_loop.create_window(attrs).expect("window");
        let _mtm = MainThreadMarker::new().expect("main thread");
        let layer = CAMetalLayer::new();
        let handle = window.window_handle().expect("handle");
        if let raw_window_handle::RawWindowHandle::AppKit(h) = handle.as_raw() {
            let ns_view: &NSView = unsafe { h.ns_view.cast().as_ref() };
            let scale: f64 = unsafe { msg_send![&*ns_view.window().unwrap(), backingScaleFactor] };
            layer.setContentsScale(scale);
            unsafe {
                let _: () = msg_send![ns_view, setWantsLayer: true];
                let _: () = msg_send![ns_view, setLayer: &*layer];
            }
        }
        let size = window.inner_size();
        let mut renderer = Renderer::new(layer).expect("renderer");
        renderer.resize(size.width, size.height);
        self.renderer = Some(renderer);
        window.request_redraw();
        self.window = Some(window);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => {
                if let Some(r) = &mut self.renderer {
                    r.resize(size.width, size.height);
                }
            }
            WindowEvent::RedrawRequested => {
                if let Some(r) = &mut self.renderer {
                    r.render();
                }
                if let Some(w) = &self.window {
                    w.request_redraw();
                }
            }
            _ => {}
        }
    }
}

fn main() {
    let event_loop = EventLoop::builder().build().expect("event loop");
    let mut app = App {
        window: None,
        renderer: None,
    };
    event_loop.run_app(&mut app).expect("run");
}
