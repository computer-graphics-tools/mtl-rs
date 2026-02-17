/// Port of Apple's "Achieving smooth frame rates with a Metal display link" sample.
///
/// Renders a rotating 3D cube in a window with triple-buffered frame pacing
/// using shared event synchronization. Demonstrates the core display link
/// rendering pattern: update → encode → present → signal → wait.
///
/// Apple sample: https://developer.apple.com/documentation/metal/achieving-smooth-frame-rates-with-a-metal-display-link
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

const MAX_FRAMES_IN_FLIGHT: usize = 2;

/// Cube vertex: position (xyz) + color (rgb).
#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    _pad: f32,
    color: [f32; 3],
    _pad2: f32,
}

/// Per-frame uniforms: MVP matrix.
#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct Uniforms {
    mvp: [[f32; 4]; 4],
}

fn cube_vertices() -> Vec<Vertex> {
    let v = |p: [f32; 3], c: [f32; 3]| Vertex {
        position: p,
        _pad: 0.0,
        color: c,
        _pad2: 0.0,
    };

    let r = [1.0, 0.3, 0.3];
    let g = [0.3, 1.0, 0.3];
    let b = [0.3, 0.3, 1.0];
    let y = [1.0, 1.0, 0.3];
    let c = [0.3, 1.0, 1.0];
    let m = [1.0, 0.3, 1.0];

    vec![
        // Front (z=1)
        v([-1.0, -1.0, 1.0], r),
        v([1.0, -1.0, 1.0], r),
        v([1.0, 1.0, 1.0], r),
        v([-1.0, -1.0, 1.0], r),
        v([1.0, 1.0, 1.0], r),
        v([-1.0, 1.0, 1.0], r),
        // Back (z=-1)
        v([1.0, -1.0, -1.0], g),
        v([-1.0, -1.0, -1.0], g),
        v([-1.0, 1.0, -1.0], g),
        v([1.0, -1.0, -1.0], g),
        v([-1.0, 1.0, -1.0], g),
        v([1.0, 1.0, -1.0], g),
        // Top (y=1)
        v([-1.0, 1.0, 1.0], b),
        v([1.0, 1.0, 1.0], b),
        v([1.0, 1.0, -1.0], b),
        v([-1.0, 1.0, 1.0], b),
        v([1.0, 1.0, -1.0], b),
        v([-1.0, 1.0, -1.0], b),
        // Bottom (y=-1)
        v([-1.0, -1.0, -1.0], y),
        v([1.0, -1.0, -1.0], y),
        v([1.0, -1.0, 1.0], y),
        v([-1.0, -1.0, -1.0], y),
        v([1.0, -1.0, 1.0], y),
        v([-1.0, -1.0, 1.0], y),
        // Right (x=1)
        v([1.0, -1.0, 1.0], c),
        v([1.0, -1.0, -1.0], c),
        v([1.0, 1.0, -1.0], c),
        v([1.0, -1.0, 1.0], c),
        v([1.0, 1.0, -1.0], c),
        v([1.0, 1.0, 1.0], c),
        // Left (x=-1)
        v([-1.0, -1.0, -1.0], m),
        v([-1.0, -1.0, 1.0], m),
        v([-1.0, 1.0, 1.0], m),
        v([-1.0, -1.0, -1.0], m),
        v([-1.0, 1.0, 1.0], m),
        v([-1.0, 1.0, -1.0], m),
    ]
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

fn rotation_y(angle: f32) -> [[f32; 4]; 4] {
    let c = angle.cos();
    let s = angle.sin();
    [
        [c, 0.0, s, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [-s, 0.0, c, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
}

fn rotation_x(angle: f32) -> [[f32; 4]; 4] {
    let c = angle.cos();
    let s = angle.sin();
    [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, c, -s, 0.0],
        [0.0, s, c, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]
}

fn mat4_mul(a: [[f32; 4]; 4], b: [[f32; 4]; 4]) -> [[f32; 4]; 4] {
    let mut out = [[0.0f32; 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            out[i][j] =
                a[i][0] * b[0][j] + a[i][1] * b[1][j] + a[i][2] * b[2][j] + a[i][3] * b[3][j];
        }
    }
    out
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
    uniform_buffers: Vec<Retained<ProtocolObject<dyn MTLBuffer>>>,
    depth_texture: Option<Retained<ProtocolObject<dyn MTLTexture>>>,
    shared_event: Retained<ProtocolObject<dyn MTLSharedEvent>>,
    frame_number: u64,
    width: u32,
    height: u32,
}

impl Renderer {
    fn new(layer: Retained<CAMetalLayer>) -> Result<Self, String> {
        let context = ExampleContext::new()?;

        let _: () = unsafe { msg_send![&*layer, setDevice: &*context.device] };
        let _: () = unsafe { msg_send![&*layer, setPixelFormat: MTLPixelFormat::BGRA8Unorm] };

        let shader = r#"
#include <metal_stdlib>
using namespace metal;

struct Vertex { float3 position; float _pad; float3 color; float _pad2; };
struct Uniforms { float4x4 mvp; };
struct V2F { float4 position [[position]]; float3 color; };

vertex V2F vertex_main(const device Vertex* verts [[buffer(0)]],
                       constant Uniforms& uniforms [[buffer(1)]],
                       uint vid [[vertex_id]]) {
    V2F out;
    out.position = uniforms.mvp * float4(verts[vid].position, 1.0);
    out.color = verts[vid].color;
    return out;
}

fragment half4 fragment_main(V2F in [[stage_in]]) {
    return half4(half3(in.color), 1.0h);
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

        let depth_desc = MTLDepthStencilDescriptor::new();
        depth_desc.set_depth_compare_function(MTLCompareFunction::Less);
        depth_desc.set_depth_write_enabled(true);
        let depth_state = context
            .device
            .new_depth_stencil_state_with_descriptor(&depth_desc)
            .ok_or("Failed to create depth stencil state")?;

        let verts = cube_vertices();
        let vertex_buffer = context
            .device
            .new_buffer_with_data(
                bytemuck::cast_slice(&verts),
                MTLResourceOptions::STORAGE_MODE_SHARED,
            )
            .ok_or("Failed to create vertex buffer")?;

        let uniform_buffers: Vec<_> = (0..MAX_FRAMES_IN_FLIGHT)
            .map(|_| {
                context
                    .device
                    .new_buffer(
                        std::mem::size_of::<Uniforms>(),
                        MTLResourceOptions::STORAGE_MODE_SHARED,
                    )
                    .expect("uniform buffer")
            })
            .collect();

        let shared_event = context.device.new_shared_event().ok_or("No shared event")?;
        shared_event.set_signaled_value(0);

        Ok(Self {
            context,
            layer,
            pipeline,
            depth_state,
            vertex_buffer,
            uniform_buffers,
            depth_texture: None,
            shared_event,
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

        let desc =
            MTLTextureDescriptor::texture_2d_descriptor_with_pixel_format_width_height_mipmapped(
                MTLPixelFormat::Depth32Float,
                w as usize,
                h as usize,
                false,
            );
        desc.set_usage(MTLTextureUsage::RENDER_TARGET);
        desc.set_storage_mode(MTLStorageMode::Private);
        self.depth_texture = self.context.device.new_texture_with_descriptor(&desc);
    }

    fn render(&mut self) {
        self.frame_number += 1;
        let idx = (self.frame_number as usize) % MAX_FRAMES_IN_FLIGHT;

        // Wait for the frame using this slot to finish.
        if self.frame_number > MAX_FRAMES_IN_FLIGHT as u64 {
            let wait_val = self.frame_number - MAX_FRAMES_IN_FLIGHT as u64;
            if self.shared_event.signaled_value() < wait_val {
                self.shared_event
                    .wait_until_signaled_value_timeout_ms(wait_val, 1000);
            }
        }

        // Update MVP.
        let aspect = self.width as f32 / self.height.max(1) as f32;
        let angle = self.frame_number as f32 * 0.02;
        let model = mat4_mul(rotation_y(angle), rotation_x(angle * 0.7));
        let view = translation(0.0, 0.0, -6.0);
        let proj = perspective(std::f32::consts::FRAC_PI_4, aspect, 0.1, 100.0);
        let mvp = mat4_mul(mat4_mul(model, view), proj);

        let uniforms = Uniforms { mvp };
        unsafe {
            std::ptr::copy_nonoverlapping(
                &uniforms as *const Uniforms,
                self.uniform_buffers[idx].contents().as_ptr() as *mut Uniforms,
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

        if let Some(depth) = &self.depth_texture {
            let depth_att = pass.depth_attachment();
            depth_att.set_texture(Some(depth));
            depth_att.set_load_action(MTLLoadAction::Clear);
            depth_att.set_store_action(MTLStoreAction::DontCare);
            depth_att.set_clear_depth(1.0);
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
        enc.set_vertex_buffer(Some(&self.uniform_buffers[idx]), 0, 1);
        enc.draw_primitives(MTLPrimitiveType::Triangle, 0, 36);

        enc.end_encoding();

        cb.present_drawable(&drawable);
        cb.encode_signal_event_value(
            objc2::runtime::ProtocolObject::from_ref(&*self.shared_event),
            self.frame_number,
        );
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
            .with_title("Achieving Smooth Frame Rates with a Metal Display Link")
            .with_inner_size(winit::dpi::LogicalSize::new(800.0, 600.0));

        let window = event_loop
            .create_window(attrs)
            .expect("Failed to create window");
        let _mtm = MainThreadMarker::new().expect("Must be on main thread");

        let layer = CAMetalLayer::new();
        let handle = window.window_handle().expect("No window handle");
        if let raw_window_handle::RawWindowHandle::AppKit(handle) = handle.as_raw() {
            let ns_view: &NSView = unsafe { handle.ns_view.cast().as_ref() };
            let scale: f64 = unsafe { msg_send![&*ns_view.window().unwrap(), backingScaleFactor] };
            layer.setContentsScale(scale);
            unsafe {
                let _: () = msg_send![ns_view, setWantsLayer: true];
                let _: () = msg_send![ns_view, setLayer: &*layer];
            }
        }

        let size = window.inner_size();
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
    let event_loop = EventLoop::builder()
        .build()
        .expect("Failed to create event loop");
    let mut app = App {
        window: None,
        renderer: None,
    };
    event_loop.run_app(&mut app).expect("Event loop failed");
}
