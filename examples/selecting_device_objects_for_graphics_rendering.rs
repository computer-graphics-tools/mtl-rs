/// Port of Apple's "Selecting device objects for graphics rendering" sample.
///
/// Demonstrates getting the system default Metal GPU device and inspecting
/// its capabilities for graphics rendering, including GPU family support.
///
/// Apple sample: https://developer.apple.com/documentation/metal/selecting-device-objects-for-graphics-rendering
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

    let name = device.name();
    let is_low_power = device.is_low_power();
    let has_unified_memory = device.has_unified_memory();

    // Check GPU family support for rendering features.
    let supports_apple7 = device.supports_family(MTLGPUFamily::Apple7);
    let supports_common3 = device.supports_family(MTLGPUFamily::Common3);
    let supports_mac2 = device.supports_family(MTLGPUFamily::Mac2);

    let max_threadgroup_memory = device.max_threadgroup_memory_length();

    println!("Selecting device objects for graphics rendering:");
    println!("  Device: {name}");
    println!("    Low power: {is_low_power}");
    println!("    Unified memory: {has_unified_memory}");
    println!("    Supports Apple7: {supports_apple7}");
    println!("    Supports Common3: {supports_common3}");
    println!("    Supports Mac2: {supports_mac2}");
    println!(
        "    Max threadgroup memory: {} KB",
        max_threadgroup_memory / 1024
    );
    Ok(())
}
