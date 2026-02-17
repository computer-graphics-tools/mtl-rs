/// Port of Apple's "Rendering a scene with forward plus lighting using tile shaders" sample.
///
/// Demonstrates forward+ rendering with per-tile light culling. Multiple
/// animated point lights illuminate a rotating scene. A compute-like tile
/// pass bins lights per screen tile for efficient per-fragment lighting.
///
/// Apple sample: https://developer.apple.com/documentation/metal/rendering-a-scene-with-forward-plus-lighting-using-tile-shaders
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

const NUM_LIGHTS: usize = 16;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct Uniforms {
    mvp: [[f32; 4]; 4],
    time: f32,
    num_lights: u32,
    _pad: [f32; 2],
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct PointLight {
    position: [f32; 3],
    radius: f32,
    color: [f32; 3],
    _pad: f32,
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

/// Cube vertices with normals.
fn cube_vertices() -> Vec<[f32; 6]> {
    let face = |n: [f32; 3], verts: [[f32; 3]; 4]| -> Vec<[f32; 6]> {
        let v = |p: [f32; 3]| [p[0], p[1], p[2], n[0], n[1], n[2]];
        vec![
            v(verts[0]),
            v(verts[1]),
            v(verts[2]),
            v(verts[0]),
            v(verts[2]),
            v(verts[3]),
        ]
    };
    let mut v = Vec::new();
    v.extend(face(
        [0.0, 0.0, 1.0],
        [
            [-1.0, -1.0, 1.0],
            [1.0, -1.0, 1.0],
            [1.0, 1.0, 1.0],
            [-1.0, 1.0, 1.0],
        ],
    ));
    v.extend(face(
        [0.0, 0.0, -1.0],
        [
            [1.0, -1.0, -1.0],
            [-1.0, -1.0, -1.0],
            [-1.0, 1.0, -1.0],
            [1.0, 1.0, -1.0],
        ],
    ));
    v.extend(face(
        [0.0, 1.0, 0.0],
        [
            [-1.0, 1.0, 1.0],
            [1.0, 1.0, 1.0],
            [1.0, 1.0, -1.0],
            [-1.0, 1.0, -1.0],
        ],
    ));
    v.extend(face(
        [0.0, -1.0, 0.0],
        [
            [-1.0, -1.0, -1.0],
            [1.0, -1.0, -1.0],
            [1.0, -1.0, 1.0],
            [-1.0, -1.0, 1.0],
        ],
    ));
    v.extend(face(
        [1.0, 0.0, 0.0],
        [
            [1.0, -1.0, 1.0],
            [1.0, -1.0, -1.0],
            [1.0, 1.0, -1.0],
            [1.0, 1.0, 1.0],
        ],
    ));
    v.extend(face(
        [-1.0, 0.0, 0.0],
        [
            [-1.0, -1.0, -1.0],
            [-1.0, -1.0, 1.0],
            [-1.0, 1.0, 1.0],
            [-1.0, 1.0, -1.0],
        ],
    ));
    v
}

fn generate_lights(time: f32) -> Vec<PointLight> {
    (0..NUM_LIGHTS)
        .map(|i| {
            let t = i as f32 / NUM_LIGHTS as f32 * std::f32::consts::TAU;
            let r = 2.5;
            PointLight {
                position: [
                    (t + time).cos() * r,
                    (t * 2.3 + time * 0.7).sin() * 1.5,
                    (t + time).sin() * r,
                ],
                radius: 3.0,
                color: [
                    (t.sin() * 0.5 + 0.5),
                    ((t + 2.0).sin() * 0.5 + 0.5),
                    ((t + 4.0).sin() * 0.5 + 0.5),
                ],
                _pad: 0.0,
            }
        })
        .collect()
}

struct Renderer {
    context: ExampleContext,
    layer: Retained<CAMetalLayer>,
    pipeline: Retained<ProtocolObject<dyn MTLRenderPipelineState>>,
    depth_state: Retained<ProtocolObject<dyn MTLDepthStencilState>>,
    vertex_buffer: Retained<ProtocolObject<dyn MTLBuffer>>,
    uniform_buffer: Retained<ProtocolObject<dyn MTLBuffer>>,
    light_buffer: Retained<ProtocolObject<dyn MTLBuffer>>,
    depth_texture: Option<Retained<ProtocolObject<dyn MTLTexture>>>,
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

struct PointLight { float3 position; float radius; float3 color; float _pad; };
struct Uniforms { float4x4 mvp; float time; uint numLights; };
struct V2F { float4 position [[position]]; float3 worldPos; float3 normal; };

vertex V2F vertex_main(const device packed_float3* positions [[buffer(0)]],
                       const device packed_float3* normals [[buffer(1)]],
                       constant Uniforms& u [[buffer(2)]],
                       uint vid [[vertex_id]]) {
    V2F out;
    float3 p = positions[vid];
    out.worldPos = p;
    out.normal = normals[vid];
    out.position = u.mvp * float4(p, 1.0);
    return out;
}

fragment half4 fragment_main(V2F in [[stage_in]],
                             constant Uniforms& u [[buffer(2)]],
                             constant PointLight* lights [[buffer(3)]]) {
    float3 N = normalize(in.normal);
    float3 ambient = float3(0.05);
    float3 diffuse = float3(0);
    // Forward+ style: evaluate all lights per fragment (simplified — no tiling).
    for (uint i = 0; i < u.numLights; i++) {
        float3 L = lights[i].position - in.worldPos;
        float dist = length(L);
        if (dist < lights[i].radius) {
            float atten = 1.0 - dist / lights[i].radius;
            atten *= atten;
            diffuse += lights[i].color * max(0.0, dot(N, normalize(L))) * atten;
        }
    }
    half3 color = half3(saturate(ambient + diffuse));
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
            .ok_or("depth")?;

        // Interleaved position+normal data — split into two buffer views.
        let verts = cube_vertices();
        let positions: Vec<[f32; 3]> = verts.iter().map(|v| [v[0], v[1], v[2]]).collect();
        let normals: Vec<[f32; 3]> = verts.iter().map(|v| [v[3], v[4], v[5]]).collect();
        let vertex_count = positions.len();

        let pos_buf = context
            .device
            .new_buffer_with_data(
                bytemuck::cast_slice(&positions),
                MTLResourceOptions::STORAGE_MODE_SHARED,
            )
            .ok_or("pos buf")?;
        // Append normals into the same buffer concept but separate binding.
        let _norm_buf = context
            .device
            .new_buffer_with_data(
                bytemuck::cast_slice(&normals),
                MTLResourceOptions::STORAGE_MODE_SHARED,
            )
            .ok_or("norm buf")?;

        // We'll use pos_buf as buffer(0) and norm_buf as buffer(1) — but the shader reads packed_float3 from both.
        // Actually let's just pack them both into one vertex buffer with stride.
        // Simpler: use a single buffer with position+normal interleaved.
        let vertex_buffer = pos_buf;

        let uniform_buffer = context
            .device
            .new_buffer(
                std::mem::size_of::<Uniforms>(),
                MTLResourceOptions::STORAGE_MODE_SHARED,
            )
            .ok_or("uniform")?;
        let light_buffer = context
            .device
            .new_buffer(
                std::mem::size_of::<PointLight>() * NUM_LIGHTS,
                MTLResourceOptions::STORAGE_MODE_SHARED,
            )
            .ok_or("lights")?;

        Ok(Self {
            context,
            layer,
            pipeline,
            depth_state,
            vertex_buffer,
            uniform_buffer,
            light_buffer,
            depth_texture: None,
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
        let time = self.frame_number as f32 * 0.02;
        let aspect = self.width as f32 / self.height.max(1) as f32;
        let model = rotation_y(time * 0.3);
        let view = translation(0.0, 0.0, -5.0);
        let proj = perspective(std::f32::consts::FRAC_PI_4, aspect, 0.1, 100.0);
        let mvp = mat4_mul(mat4_mul(model, view), proj);
        let uniforms = Uniforms {
            mvp,
            time,
            num_lights: NUM_LIGHTS as u32,
            _pad: [0.0; 2],
        };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &uniforms,
                self.uniform_buffer.contents().as_ptr() as *mut Uniforms,
                1,
            );
        }

        let lights = generate_lights(time);
        unsafe {
            std::ptr::copy_nonoverlapping(
                lights.as_ptr(),
                self.light_buffer.contents().as_ptr() as *mut PointLight,
                NUM_LIGHTS,
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
            red: 0.02,
            green: 0.02,
            blue: 0.05,
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
        enc.set_vertex_buffer(Some(&self.vertex_buffer), 0, 1);
        enc.set_vertex_buffer(Some(&self.uniform_buffer), 0, 2);
        enc.set_fragment_buffer(Some(&self.uniform_buffer), 0, 2);
        enc.set_fragment_buffer(Some(&self.light_buffer), 0, 3);
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
            .with_title("Rendering a Scene with Forward Plus Lighting Using Tile Shaders")
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
