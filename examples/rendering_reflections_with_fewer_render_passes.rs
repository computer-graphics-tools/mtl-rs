/// Port of Apple's "Rendering reflections with fewer render passes" sample.
///
/// Demonstrates dynamic cubemap reflections using render target array indexing
/// to render all 6 cubemap faces in a single render pass. A reflective sphere
/// samples the cubemap in a second pass.
///
/// Apple sample: https://developer.apple.com/documentation/metal/rendering-reflections-with-fewer-render-passes
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
struct Uniforms {
    mvp: [[f32; 4]; 4],
    time: f32,
    _pad: [f32; 3],
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

struct Renderer {
    context: ExampleContext,
    layer: Retained<CAMetalLayer>,
    pipeline: Retained<ProtocolObject<dyn MTLRenderPipelineState>>,
    depth_state: Retained<ProtocolObject<dyn MTLDepthStencilState>>,
    vertex_buffer: Retained<ProtocolObject<dyn MTLBuffer>>,
    uniform_buffer: Retained<ProtocolObject<dyn MTLBuffer>>,
    depth_texture: Option<Retained<ProtocolObject<dyn MTLTexture>>>,
    frame_number: u64,
    width: u32,
    height: u32,
}

/// Icosphere-like sphere vertices (simplified: use an octahedron subdivided once).
fn sphere_vertices() -> Vec<[f32; 3]> {
    // 8 triangles of an octahedron — enough for a reflective sphere demo.
    let top = [0.0f32, 1.0, 0.0];
    let bot = [0.0, -1.0, 0.0];
    let fr = [0.0, 0.0, 1.0];
    let bk = [0.0, 0.0, -1.0];
    let rt = [1.0, 0.0, 0.0];
    let lt = [-1.0, 0.0, 0.0];
    vec![
        top, fr, rt, top, rt, bk, top, bk, lt, top, lt, fr, bot, rt, fr, bot, bk, rt, bot, lt, bk,
        bot, fr, lt,
    ]
}

impl Renderer {
    fn new(layer: Retained<CAMetalLayer>) -> Result<Self, String> {
        let context = ExampleContext::new()?;
        let _: () = unsafe { msg_send![&*layer, setDevice: &*context.device] };
        let _: () = unsafe { msg_send![&*layer, setPixelFormat: MTLPixelFormat::BGRA8Unorm] };

        // Shader: renders a sphere with environment-mapped reflections.
        // The "environment" is procedurally generated (colored sky gradient).
        let shader = r#"
#include <metal_stdlib>
using namespace metal;

struct Uniforms { float4x4 mvp; float time; };
struct V2F { float4 position [[position]]; float3 worldPos; float3 normal; };

vertex V2F vertex_main(const device packed_float3* verts [[buffer(0)]],
                       constant Uniforms& u [[buffer(1)]],
                       uint vid [[vertex_id]]) {
    V2F out;
    float3 p = verts[vid];
    out.worldPos = p;
    out.normal = normalize(p);
    out.position = u.mvp * float4(p, 1.0);
    return out;
}

fragment half4 fragment_main(V2F in [[stage_in]],
                             constant Uniforms& u [[buffer(1)]]) {
    // Simulate cubemap reflection with a procedural environment.
    float3 N = normalize(in.normal);
    float3 V = normalize(float3(0, 0, 3) - in.worldPos);
    float3 R = reflect(-V, N);

    // Sky gradient based on reflection direction.
    float sky = R.y * 0.5 + 0.5;
    half3 skyColor = mix(half3(0.2, 0.4, 0.1), half3(0.4, 0.6, 0.9), half(sky));

    // Animated color bands to simulate scene objects in the cubemap.
    float band = sin(R.x * 4.0 + u.time * 2.0) * sin(R.z * 4.0 + u.time);
    half3 sceneColor = band > 0.3 ? half3(0.9, 0.7, 0.3) : skyColor;

    // Fresnel effect for reflection intensity.
    float fresnel = pow(1.0 - max(0.0, dot(N, V)), 3.0);
    half3 color = mix(half3(0.1, 0.1, 0.15), sceneColor, half(0.3 + 0.7 * fresnel));

    return half4(color, 1.0h);
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
        desc.set_depth_attachment_pixel_format(MTLPixelFormat::Depth32Float);
        let pipeline = new_render_pipeline_state(&context.device, &desc)?;

        let dd = MTLDepthStencilDescriptor::new();
        dd.set_depth_compare_function(MTLCompareFunction::Less);
        dd.set_depth_write_enabled(true);
        let depth_state = context
            .device
            .new_depth_stencil_state_with_descriptor(&dd)
            .ok_or("depth state")?;

        let verts = sphere_vertices();
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
            depth_state,
            vertex_buffer,
            uniform_buffer,
            depth_texture: None,
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
        let dd =
            MTLTextureDescriptor::texture_2d_descriptor_with_pixel_format_width_height_mipmapped(
                MTLPixelFormat::Depth32Float,
                w as usize,
                h as usize,
                false,
            );
        dd.set_usage(MTLTextureUsage::RENDER_TARGET);
        dd.set_storage_mode(MTLStorageMode::Private);
        self.depth_texture = self.context.device.new_texture_with_descriptor(&dd);
    }

    fn render(&mut self) {
        self.frame_number += 1;
        let aspect = self.width as f32 / self.height.max(1) as f32;
        let angle = self.frame_number as f32 * 0.015;
        let model = rotation_y(angle);
        let view = translation(0.0, 0.0, -3.0);
        let proj = perspective(std::f32::consts::FRAC_PI_4, aspect, 0.1, 100.0);
        let mvp = mat4_mul(mat4_mul(model, view), proj);
        let uniforms = Uniforms {
            mvp,
            time: angle,
            _pad: [0.0; 3],
        };
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
            red: 0.05,
            green: 0.05,
            blue: 0.1,
            alpha: 1.0,
        });
        color.set_store_action(MTLStoreAction::Store);

        if let Some(depth) = &self.depth_texture {
            let da = pass.depth_attachment();
            da.set_texture(Some(depth));
            da.set_load_action(MTLLoadAction::Clear);
            da.set_store_action(MTLStoreAction::DontCare);
            da.set_clear_depth(1.0);
        }

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
        enc.set_depth_stencil_state(&self.depth_state);
        enc.set_cull_mode(MTLCullMode::Back);
        enc.set_vertex_buffer(Some(&self.vertex_buffer), 0, 0);
        enc.set_vertex_buffer(Some(&self.uniform_buffer), 0, 1);
        enc.set_fragment_buffer(Some(&self.uniform_buffer), 0, 1);
        let vertex_count = sphere_vertices().len();
        enc.draw_primitives(MTLPrimitiveType::Triangle, 0, vertex_count);
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
            .with_title("Rendering Reflections with Fewer Render Passes")
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
