/// Port of Apple's "Supporting Simulator in a Metal app" sample.
///
/// Demonstrates querying platform-specific Metal capabilities that differ
/// between physical devices and the iOS Simulator, including buffer alignment,
/// depth/stencil formats, storage modes, and MSAA sample counts.
///
/// Apple sample: https://developer.apple.com/documentation/metal/supporting-simulator-in-a-metal-app
mod common;

use metal::MTLGPUFamily;
use metal::prelude::*;

use common::ExampleContext;

fn main() {
    if let Err(error) = run() {
        eprintln!("Example failed: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let context = ExampleContext::new()?;
    let device = &context.device;

    // Query linear texture alignment (platform-dependent).
    let bgra_alignment =
        device.minimum_linear_texture_alignment_for_pixel_format(MTLPixelFormat::BGRA8Unorm);
    let rgba_alignment =
        device.minimum_linear_texture_alignment_for_pixel_format(MTLPixelFormat::RGBA8Unorm);

    // Check depth format support.
    let supports_depth32_stencil8 =
        device.supports_family(MTLGPUFamily::Mac2) || device.supports_family(MTLGPUFamily::Apple1);

    // Query MSAA sample count support.
    let supports_msaa_2 = device.supports_texture_sample_count(2);
    let supports_msaa_4 = device.supports_texture_sample_count(4);
    let supports_msaa_8 = device.supports_texture_sample_count(8);

    // Query read-write texture support (tier).
    let supports_rw_texture_tier2 = device.supports_family(MTLGPUFamily::Apple4);

    println!("Supporting Simulator in a Metal app:");
    println!("  Device: {}", device.name());
    println!();
    println!("  Linear texture alignment:");
    println!("    BGRA8Unorm: {} bytes", bgra_alignment);
    println!("    RGBA8Unorm: {} bytes", rgba_alignment);
    println!();
    println!("  Depth/stencil:");
    println!(
        "    Depth32Float_Stencil8: {}",
        if supports_depth32_stencil8 {
            "supported"
        } else {
            "not supported"
        }
    );
    println!();
    println!("  MSAA sample counts:");
    println!("    2x: {supports_msaa_2}");
    println!("    4x: {supports_msaa_4}");
    println!("    8x: {supports_msaa_8}");
    println!();
    println!(
        "  Read-write textures (tier 2): {}",
        if supports_rw_texture_tier2 {
            "supported"
        } else {
            "not supported"
        }
    );
    Ok(())
}
