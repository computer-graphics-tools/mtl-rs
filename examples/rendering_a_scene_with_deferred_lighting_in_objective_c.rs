/// Port of Apple's "Rendering a scene with deferred lighting in Objective-C" sample.
///
/// Demonstrates deferred lighting with a G-buffer pass and a lighting pass.
/// The G-buffer stores per-pixel normals and albedo; the lighting pass
/// evaluates multiple animated point lights using the G-buffer data.
///
/// Apple sample: https://developer.apple.com/documentation/metal/rendering-a-scene-with-deferred-lighting-in-objective-c
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

const NUM_LIGHTS: usize = 32;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct Uniforms {
    mvp: [[f32; 4]; 4],
    model: [[f32; 4]; 4],
    time: f32,
    num_lights: u32,
    _pad: [f32; 2],
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct Light {
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

/// Cube with per-face normals.
fn cube_verts() -> Vec<[f32; 6]> {
    let face = |n: [f32; 3], vs: [[f32; 3]; 4]| -> Vec<[f32; 6]> {
        let v = |p: [f32; 3]| [p[0], p[1], p[2], n[0], n[1], n[2]];
        vec![v(vs[0]), v(vs[1]), v(vs[2]), v(vs[0]), v(vs[2]), v(vs[3])]
    };
    let mut r = Vec::new();
    r.extend(face(
        [0.0, 0.0, 1.0],
        [
            [-1.0, -1.0, 1.0],
            [1.0, -1.0, 1.0],
            [1.0, 1.0, 1.0],
            [-1.0, 1.0, 1.0],
        ],
    ));
    r.extend(face(
        [0.0, 0.0, -1.0],
        [
            [1.0, -1.0, -1.0],
            [-1.0, -1.0, -1.0],
            [-1.0, 1.0, -1.0],
            [1.0, 1.0, -1.0],
        ],
    ));
    r.extend(face(
        [0.0, 1.0, 0.0],
        [
            [-1.0, 1.0, 1.0],
            [1.0, 1.0, 1.0],
            [1.0, 1.0, -1.0],
            [-1.0, 1.0, -1.0],
        ],
    ));
    r.extend(face(
        [0.0, -1.0, 0.0],
        [
            [-1.0, -1.0, -1.0],
            [1.0, -1.0, -1.0],
            [1.0, -1.0, 1.0],
            [-1.0, -1.0, 1.0],
        ],
    ));
    r.extend(face(
        [1.0, 0.0, 0.0],
        [
            [1.0, -1.0, 1.0],
            [1.0, -1.0, -1.0],
            [1.0, 1.0, -1.0],
            [1.0, 1.0, 1.0],
        ],
    ));
    r.extend(face(
        [-1.0, 0.0, 0.0],
        [
            [-1.0, -1.0, -1.0],
            [-1.0, -1.0, 1.0],
            [-1.0, 1.0, 1.0],
            [-1.0, 1.0, -1.0],
        ],
    ));
    r
}

fn gen_lights(time: f32) -> Vec<Light> {
    (0..NUM_LIGHTS)
        .map(|i| {
            let t = i as f32 / NUM_LIGHTS as f32 * std::f32::consts::TAU;
            let r = 3.0;
            Light {
                position: [
                    (t + time).cos() * r,
                    (t * 1.7 + time * 0.5).sin() * 2.0,
                    (t + time).sin() * r,
                ],
                radius: 4.0,
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
    gbuffer_pipeline: Retained<ProtocolObject<dyn MTLRenderPipelineState>>,
    lighting_pipeline: Retained<ProtocolObject<dyn MTLRenderPipelineState>>,
    depth_state: Retained<ProtocolObject<dyn MTLDepthStencilState>>,
    vertex_buffer: Retained<ProtocolObject<dyn MTLBuffer>>,
    uniform_buffer: Retained<ProtocolObject<dyn MTLBuffer>>,
    light_buffer: Retained<ProtocolObject<dyn MTLBuffer>>,
    albedo_texture: Option<Retained<ProtocolObject<dyn MTLTexture>>>,
    normal_texture: Option<Retained<ProtocolObject<dyn MTLTexture>>>,
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

        // G-buffer pass: writes albedo + normals to textures.
        let gbuf_shader = r#"
#include <metal_stdlib>
using namespace metal;
struct Uniforms { float4x4 mvp; float4x4 model; float time; uint numLights; };
struct V2F { float4 position [[position]]; float3 normal; float3 worldPos; };
struct GBufOut { half4 albedo [[color(0)]]; half4 normal [[color(1)]]; };

vertex V2F gbuf_vert(const device packed_float3* pos [[buffer(0)]],
                     const device packed_float3* norm [[buffer(1)]],
                     constant Uniforms& u [[buffer(2)]],
                     uint vid [[vertex_id]]) {
    V2F out;
    float3 p = pos[vid];
    out.position = u.mvp * float4(p, 1.0);
    out.normal = (u.model * float4(norm[vid], 0.0)).xyz;
    out.worldPos = (u.model * float4(p, 1.0)).xyz;
    return out;
}

fragment GBufOut gbuf_frag(V2F in [[stage_in]]) {
    GBufOut out;
    out.albedo = half4(0.7, 0.7, 0.7, 1.0);
    out.normal = half4(half3(normalize(in.normal) * 0.5 + 0.5), 1.0);
    return out;
}
"#;

        // Lighting pass: full-screen quad reads G-buffer, evaluates lights.
        let light_shader = r#"
#include <metal_stdlib>
using namespace metal;
struct Light { float3 position; float radius; float3 color; float _pad; };
struct Uniforms { float4x4 mvp; float4x4 model; float time; uint numLights; };
struct V2F { float4 position [[position]]; float2 uv; };

vertex V2F light_vert(uint vid [[vertex_id]]) {
    float2 positions[] = { {-1,-1}, {1,-1}, {1,1}, {-1,-1}, {1,1}, {-1,1} };
    float2 uvs[] = { {0,1}, {1,1}, {1,0}, {0,1}, {1,0}, {0,0} };
    V2F out;
    out.position = float4(positions[vid], 0, 1);
    out.uv = uvs[vid];
    return out;
}

fragment half4 light_frag(V2F in [[stage_in]],
                          texture2d<half> albedoTex [[texture(0)]],
                          texture2d<half> normalTex [[texture(1)]],
                          constant Uniforms& u [[buffer(0)]],
                          constant Light* lights [[buffer(1)]]) {
    constexpr sampler s(coord::normalized, filter::nearest);
    half4 albedo = albedoTex.sample(s, in.uv);
    half3 N = normalize(half3(normalTex.sample(s, in.uv).xyz) * 2.0h - 1.0h);
    float3 fragPos = float3(in.uv * 2.0 - 1.0, 0.0); // simplified world pos

    float3 lighting = float3(0.08); // ambient
    for (uint i = 0; i < u.numLights; i++) {
        float3 L = lights[i].position - fragPos;
        float dist = length(L);
        if (dist < lights[i].radius) {
            float atten = 1.0 - dist / lights[i].radius;
            atten *= atten;
            float NdotL = max(0.0, dot(float3(N), normalize(L)));
            lighting += lights[i].color * NdotL * atten;
        }
    }
    return half4(half3(float3(albedo.rgb) * saturate(lighting)), 1.0h);
}
"#;

        let gbuf_lib = compile_library_from_source(&context.device, gbuf_shader)?;
        let light_lib = compile_library_from_source(&context.device, light_shader)?;

        // G-buffer pipeline: two color attachments (albedo + normal).
        let gdesc = MTLRenderPipelineDescriptor::new();
        let gv = gbuf_lib
            .new_function_with_name("gbuf_vert")
            .ok_or("gbuf_vert")?;
        let gf = gbuf_lib
            .new_function_with_name("gbuf_frag")
            .ok_or("gbuf_frag")?;
        gdesc.set_vertex_function(Some(&gv));
        gdesc.set_fragment_function(Some(&gf));
        gdesc
            .color_attachments()
            .object_at_indexed_subscript(0)
            .set_pixel_format(MTLPixelFormat::BGRA8Unorm);
        gdesc
            .color_attachments()
            .object_at_indexed_subscript(1)
            .set_pixel_format(MTLPixelFormat::RGBA8Unorm);
        gdesc.set_depth_attachment_pixel_format(MTLPixelFormat::Depth32Float);
        let gbuffer_pipeline = new_render_pipeline_state(&context.device, &gdesc)?;

        // Lighting pipeline: reads G-buffer textures, outputs to screen.
        let ldesc = MTLRenderPipelineDescriptor::new();
        let lv = light_lib
            .new_function_with_name("light_vert")
            .ok_or("light_vert")?;
        let lf = light_lib
            .new_function_with_name("light_frag")
            .ok_or("light_frag")?;
        ldesc.set_vertex_function(Some(&lv));
        ldesc.set_fragment_function(Some(&lf));
        ldesc
            .color_attachments()
            .object_at_indexed_subscript(0)
            .set_pixel_format(MTLPixelFormat::BGRA8Unorm);
        let lighting_pipeline = new_render_pipeline_state(&context.device, &ldesc)?;

        let dd = MTLDepthStencilDescriptor::new();
        dd.set_depth_compare_function(MTLCompareFunction::Less);
        dd.set_depth_write_enabled(true);
        let depth_state = context
            .device
            .new_depth_stencil_state_with_descriptor(&dd)
            .ok_or("depth")?;

        let verts = cube_verts();
        let vertex_count = verts.len();
        let positions: Vec<[f32; 3]> = verts.iter().map(|v| [v[0], v[1], v[2]]).collect();
        let normals: Vec<[f32; 3]> = verts.iter().map(|v| [v[3], v[4], v[5]]).collect();
        let pos_data: Vec<u8> = bytemuck::cast_slice(&positions).to_vec();
        let norm_data: Vec<u8> = bytemuck::cast_slice(&normals).to_vec();
        let mut combined = pos_data;
        combined.extend(norm_data);
        let vertex_buffer = context
            .device
            .new_buffer_with_data(&combined, MTLResourceOptions::STORAGE_MODE_SHARED)
            .ok_or("vb")?;

        let uniform_buffer = context
            .device
            .new_buffer(
                std::mem::size_of::<Uniforms>(),
                MTLResourceOptions::STORAGE_MODE_SHARED,
            )
            .ok_or("ub")?;
        let light_buffer = context
            .device
            .new_buffer(
                std::mem::size_of::<Light>() * NUM_LIGHTS,
                MTLResourceOptions::STORAGE_MODE_SHARED,
            )
            .ok_or("lb")?;

        Ok(Self {
            context,
            layer,
            gbuffer_pipeline,
            lighting_pipeline,
            depth_state,
            vertex_buffer,
            uniform_buffer,
            light_buffer,
            albedo_texture: None,
            normal_texture: None,
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

        let make_tex = |fmt, usage| {
            let d = MTLTextureDescriptor::texture_2d_descriptor_with_pixel_format_width_height_mipmapped(fmt, w as usize, h as usize, false);
            d.set_usage(usage);
            d.set_storage_mode(MTLStorageMode::Private);
            self.context.device.new_texture_with_descriptor(&d)
        };
        self.albedo_texture = make_tex(
            MTLPixelFormat::BGRA8Unorm,
            MTLTextureUsage::RENDER_TARGET | MTLTextureUsage::SHADER_READ,
        );
        self.normal_texture = make_tex(
            MTLPixelFormat::RGBA8Unorm,
            MTLTextureUsage::RENDER_TARGET | MTLTextureUsage::SHADER_READ,
        );
        self.depth_texture = make_tex(MTLPixelFormat::Depth32Float, MTLTextureUsage::RENDER_TARGET);
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
            model,
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
        let lights = gen_lights(time);
        unsafe {
            std::ptr::copy_nonoverlapping(
                lights.as_ptr(),
                self.light_buffer.contents().as_ptr() as *mut Light,
                NUM_LIGHTS,
            );
        }

        let (Some(albedo), Some(normal), Some(depth)) = (
            &self.albedo_texture,
            &self.normal_texture,
            &self.depth_texture,
        ) else {
            return;
        };

        let drawable: Option<Retained<ProtocolObject<dyn MTLDrawable>>> =
            unsafe { msg_send![&*self.layer, nextDrawable] };
        let drawable = match drawable {
            Some(d) => d,
            None => return,
        };
        let draw_tex: Option<Retained<ProtocolObject<dyn MTLTexture>>> =
            unsafe { msg_send![&*drawable, texture] };
        let draw_tex = match draw_tex {
            Some(t) => t,
            None => return,
        };

        let cb = match self.context.queue.command_buffer() {
            Some(cb) => cb,
            None => return,
        };

        // Pass 1: G-buffer.
        {
            let pass = MTLRenderPassDescriptor::render_pass_descriptor();
            let c0 = pass.color_attachments().object_at_indexed_subscript(0);
            c0.set_texture(Some(albedo));
            c0.set_load_action(MTLLoadAction::Clear);
            c0.set_clear_color(MTLClearColor {
                red: 0.0,
                green: 0.0,
                blue: 0.0,
                alpha: 1.0,
            });
            c0.set_store_action(MTLStoreAction::Store);
            let c1 = pass.color_attachments().object_at_indexed_subscript(1);
            c1.set_texture(Some(normal));
            c1.set_load_action(MTLLoadAction::Clear);
            c1.set_clear_color(MTLClearColor {
                red: 0.5,
                green: 0.5,
                blue: 0.5,
                alpha: 1.0,
            });
            c1.set_store_action(MTLStoreAction::Store);
            let da = pass.depth_attachment();
            da.set_texture(Some(depth));
            da.set_load_action(MTLLoadAction::Clear);
            da.set_store_action(MTLStoreAction::DontCare);
            da.set_clear_depth(1.0);

            let enc = cb.render_command_encoder_with_descriptor(&pass);
            let enc = match enc {
                Some(e) => e,
                None => return,
            };
            enc.set_render_pipeline_state(&self.gbuffer_pipeline);
            enc.set_depth_stencil_state(&self.depth_state);
            enc.set_cull_mode(MTLCullMode::Back);
            let norm_offset = self.vertex_count * 3 * 4; // positions size in bytes
            enc.set_vertex_buffer(Some(&self.vertex_buffer), 0, 0);
            enc.set_vertex_buffer(Some(&self.vertex_buffer), norm_offset, 1);
            enc.set_vertex_buffer(Some(&self.uniform_buffer), 0, 2);
            enc.draw_primitives(MTLPrimitiveType::Triangle, 0, self.vertex_count);
            enc.end_encoding();
        }

        // Pass 2: Lighting (full-screen quad).
        {
            let pass = MTLRenderPassDescriptor::render_pass_descriptor();
            let c0 = pass.color_attachments().object_at_indexed_subscript(0);
            c0.set_texture(Some(&draw_tex));
            c0.set_load_action(MTLLoadAction::Clear);
            c0.set_clear_color(MTLClearColor {
                red: 0.0,
                green: 0.0,
                blue: 0.0,
                alpha: 1.0,
            });
            c0.set_store_action(MTLStoreAction::Store);

            let enc = cb.render_command_encoder_with_descriptor(&pass);
            let enc = match enc {
                Some(e) => e,
                None => return,
            };
            enc.set_render_pipeline_state(&self.lighting_pipeline);
            enc.set_fragment_texture(Some(albedo), 0);
            enc.set_fragment_texture(Some(normal), 1);
            enc.set_fragment_buffer(Some(&self.uniform_buffer), 0, 0);
            enc.set_fragment_buffer(Some(&self.light_buffer), 0, 1);
            enc.draw_primitives(MTLPrimitiveType::Triangle, 0, 6);
            enc.end_encoding();
        }

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
            .with_title("Rendering a Scene with Deferred Lighting")
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
