/// Port of Apple's "Drawing a triangle with Metal 4" sample.
///
/// Apple sample: https://developer.apple.com/documentation/metal/drawing-a-triangle-with-metal-4
mod renderer;
mod vertex_data;

use objc2::{MainThreadMarker, msg_send};
use objc2_app_kit::NSView;
use objc2_core_foundation::CGSize;
use objc2_quartz_core::CAMetalLayer;
use raw_window_handle::HasWindowHandle;
use renderer::TriangleRenderer;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowId},
};

struct TriangleApplication {
    window: Option<Window>,
    renderer: Option<TriangleRenderer>,
}

impl ApplicationHandler for TriangleApplication {
    fn resumed(
        &mut self,
        event_loop: &ActiveEventLoop,
    ) {
        if self.window.is_some() {
            return;
        }

        let window_attributes = Window::default_attributes()
            .with_title("Drawing a Triangle with Metal")
            .with_inner_size(winit::dpi::LogicalSize::new(800.0, 600.0));

        let window = event_loop.create_window(window_attributes).expect("Failed to create window");

        let _main_thread_marker = MainThreadMarker::new().expect("Must be on the main thread");

        let metal_layer = CAMetalLayer::new();

        let window_handle = window.window_handle().expect("No window handle");
        if let raw_window_handle::RawWindowHandle::AppKit(appkit_handle) = window_handle.as_raw() {
            let ns_view: &NSView = unsafe { appkit_handle.ns_view.cast().as_ref() };
            let backing_scale_factor: f64 =
                unsafe { msg_send![&*ns_view.window().expect("NSView must have a window"), backingScaleFactor] };
            metal_layer.setContentsScale(backing_scale_factor);
            unsafe {
                let _: () = msg_send![ns_view, setWantsLayer: true];
                let _: () = msg_send![ns_view, setLayer: &*metal_layer];
            }
        }

        let window_size = window.inner_size();
        metal_layer.setDrawableSize(CGSize {
            width: window_size.width as f64,
            height: window_size.height as f64,
        });

        let mut renderer = TriangleRenderer::new(metal_layer).expect("Failed to create renderer");
        renderer.resize(window_size.width, window_size.height);
        self.renderer = Some(renderer);
        window.request_redraw();
        self.window = Some(window);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(new_size) => {
                if let Some(renderer) = &mut self.renderer {
                    renderer.resize(new_size.width, new_size.height);
                }
            },
            WindowEvent::RedrawRequested => {
                if let Some(renderer) = &mut self.renderer {
                    renderer.render();
                }
                if let Some(window) = &self.window {
                    window.request_redraw();
                }
            },
            _ => {},
        }
    }
}

fn main() {
    let event_loop = EventLoop::builder().build().expect("Failed to create event loop");

    let mut application = TriangleApplication {
        window: None,
        renderer: None,
    };

    event_loop.run_app(&mut application).expect("Event loop failed");
}
