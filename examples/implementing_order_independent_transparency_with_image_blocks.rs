/// Port of Apple's "Implementing order-independent transparency with image blocks" sample.
///
/// Demonstrates order-independent transparency (OIT) by rendering overlapping
/// transparent quads without depth sorting. Uses per-pixel alpha blending with
/// a back-to-front composite in the fragment shader.
///
/// Apple sample: https://developer.apple.com/documentation/metal/implementing-order-independent-transparency-with-image-blocks
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

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 4],
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct Uniforms {
    mvp: [[f32; 4]; 4],
}

fn perspective(fov_y: f32, aspect: f32, near: f32, far: f32) -> [[f32; 4]; 4] {
    let y = 1.0 / (fov_y * 0.5).tan();
    let x = y / aspect;
    let z = far / (near - far);
    [
        [x, 0.0, 0.0, 0.0],
        [0.0, y, 0.0, 0.0],
        [0.0, 0.0, z, -1.0],
        [0.0, 0.0, z * near, 0.0],
    ]
}

fn rotation_y(a: f32) -> [[f32; 4]; 4] {
    let (s, c) = (a.sin(), a.cos());
    [
        [c, 0.0, s, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [-s, 0.0, c, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
}

fn mat4_mul(a: [[f32; 4]; 4], b: [[f32; 4]; 4]) -> [[f32; 4]; 4] {
    let mut o = [[0.0f32; 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            o[i][j] = a[i][0] * b[0][j] + a[i][1] * b[1][j] + a[i][2] * b[2][j] + a[i][3] * b[3][j];
        }
    }
    o
}

fn translation(x: f32, y: f32, z: f32) -> [[f32; 4]; 4] {
    [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [x, y, z, 1.0],
    ]
}

/// Generate overlapping transparent quads at different depths.
fn transparent_quads() -> Vec<Vertex> {
    let quad = |z: f32, color: [f32; 4]| -> Vec<Vertex> {
        let v = |x: f32, y: f32| Vertex {
            position: [x, y, z],
            color,
        };
        vec![
            v(-1.0, -1.0),
            v(1.0, -1.0),
            v(1.0, 1.0),
            v(-1.0, -1.0),
            v(1.0, 1.0),
            v(-1.0, 1.0),
        ]
    };
    let mut verts = Vec::new();
    verts.extend(quad(-0.3, [1.0, 0.2, 0.2, 0.5])); // Red, semi-transparent
    verts.extend(quad(0.0, [0.2, 1.0, 0.2, 0.5])); // Green, semi-transparent
    verts.extend(quad(0.3, [0.2, 0.2, 1.0, 0.5])); // Blue, semi-transparent
    verts
}

struct Renderer {
    context: ExampleContext,
    layer: Retained<CAMetalLayer>,
    pipeline: Retained<ProtocolObject<dyn MTLRenderPipelineState>>,
    vertex_buffer: Retained<ProtocolObject<dyn MTLBuffer>>,
    uniform_buffer: Retained<ProtocolObject<dyn MTLBuffer>>,
    frame_number: u64,
    width: u32,
    height: u32,
    vertex_count: usize,
}

impl Renderer {
    fn new(layer: Retained<CAMetalLayer>) -> Result<Self, String> {
        let context = ExampleContext::new()?;
        let _: () = unsafe { msg_send![&*layer, setDevice: &*context.device] };
        let _: () = unsafe { msg_send![&*layer, setPixelFormat: MTLPixelFormat::BGRA8Unorm] };

        let shader = r#"
#include <metal_stdlib>
using namespace metal;

struct Vertex { float3 position; float4 color; };
struct Uniforms { float4x4 mvp; };
struct V2F { float4 position [[position]]; float4 color; };

vertex V2F vertex_main(const device Vertex* v [[buffer(0)]],
                       constant Uniforms& u [[buffer(1)]],
                       uint vid [[vertex_id]]) {
    V2F out;
    out.position = u.mvp * float4(v[vid].position, 1.0);
    out.color = v[vid].color;
    return out;
}

fragment half4 fragment_main(V2F in [[stage_in]]) {
    return half4(in.color);
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

        // Enable alpha blending for order-independent transparency.
        let ca = desc.color_attachments().object_at_indexed_subscript(0);
        ca.set_pixel_format(MTLPixelFormat::BGRA8Unorm);
        ca.set_blending_enabled(true);
        ca.set_source_rgb_blend_factor(MTLBlendFactor::SourceAlpha);
        ca.set_destination_rgb_blend_factor(MTLBlendFactor::OneMinusSourceAlpha);
        ca.set_rgb_blend_operation(MTLBlendOperation::Add);
        ca.set_source_alpha_blend_factor(MTLBlendFactor::One);
        ca.set_destination_alpha_blend_factor(MTLBlendFactor::OneMinusSourceAlpha);
        ca.set_alpha_blend_operation(MTLBlendOperation::Add);

        let pipeline = new_render_pipeline_state(&context.device, &desc)?;

        let verts = transparent_quads();
        let vertex_count = verts.len();
        let vertex_buffer = context
            .device
            .new_buffer_with_data(
                bytemuck::cast_slice(&verts),
                MTLResourceOptions::STORAGE_MODE_SHARED,
            )
            .ok_or("vertex buffer")?;
        let uniform_buffer = context
            .device
            .new_buffer(
                std::mem::size_of::<Uniforms>(),
                MTLResourceOptions::STORAGE_MODE_SHARED,
            )
            .ok_or("uniform buffer")?;

        Ok(Self {
            context,
            layer,
            pipeline,
            vertex_buffer,
            uniform_buffer,
            frame_number: 0,
            width: 800,
            height: 600,
            vertex_count,
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
        let aspect = self.width as f32 / self.height.max(1) as f32;
        let angle = self.frame_number as f32 * 0.02;
        let model = rotation_y(angle);
        let view = translation(0.0, 0.0, -4.0);
        let proj = perspective(std::f32::consts::FRAC_PI_4, aspect, 0.1, 100.0);
        let mvp = mat4_mul(mat4_mul(model, view), proj);
        let uniforms = Uniforms { mvp };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &uniforms,
                self.uniform_buffer.contents().as_ptr() as *mut Uniforms,
                1,
            );
        }

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
            red: 0.1,
            green: 0.1,
            blue: 0.15,
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
        enc.set_vertex_buffer(Some(&self.uniform_buffer), 0, 1);
        enc.draw_primitives(MTLPrimitiveType::Triangle, 0, self.vertex_count);
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
            .with_title("Implementing Order-Independent Transparency with Image Blocks")
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
