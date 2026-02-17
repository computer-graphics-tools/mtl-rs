/// Port of Apple's "Creating a custom Metal view" sample.
///
/// Demonstrates creating and configuring a CAMetalLayer directly rather than
/// using MTKView. This shows the low-level layer setup including pixel format,
/// drawable size, and device assignment.
///
/// Apple sample: https://developer.apple.com/documentation/metal/creating-a-custom-metal-view
mod common;

use metal::prelude::*;
use objc2::msg_send;
use objc2_core_foundation::CGSize;
use objc2_quartz_core::CAMetalLayer;

use common::ExampleContext;

fn main() {
    if let Err(error) = run() {
        eprintln!("Example failed: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let context = ExampleContext::new()?;

    // Create and configure a CAMetalLayer (the core of a custom Metal view).
    let layer = CAMetalLayer::new();
    unsafe {
        let _: () = msg_send![&*layer, setDevice: &*context.device];
        let _: () = msg_send![&*layer, setPixelFormat: MTLPixelFormat::BGRA8Unorm];
        let _: () = msg_send![&*layer, setFramebufferOnly: true];
    }
    layer.setDrawableSize(CGSize::new(800.0, 600.0));

    let pixel_format: MTLPixelFormat = unsafe { msg_send![&*layer, pixelFormat] };
    let size = layer.drawableSize();
    let fb_only: bool = unsafe { msg_send![&*layer, framebufferOnly] };

    println!("Creating a custom Metal view:");
    println!("  CAMetalLayer configured with:");
    println!("    Pixel format: {pixel_format:?}");
    println!("    Drawable size: {}x{}", size.width, size.height);
    println!("    Framebuffer only: {fb_only}");

    Ok(())
}
