/// Port of Apple's "Selecting device objects for compute processing" sample.
///
/// Demonstrates getting the system default Metal GPU device and inspecting
/// its capabilities for compute workloads.
///
/// Apple sample: https://developer.apple.com/documentation/metal/selecting-device-objects-for-compute-processing
mod common;

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
    let max_threads = device.max_threads_per_threadgroup();
    let max_buffer_length = device.max_buffer_length();
    let recommended_max_working_set_size = device.recommended_max_working_set_size();

    println!("Selecting device objects for compute processing:");
    println!("  Device: {name}");
    println!("    Low power: {is_low_power}");
    println!("    Unified memory: {has_unified_memory}");
    println!(
        "    Max threads per threadgroup: {}x{}x{}",
        max_threads.width, max_threads.height, max_threads.depth
    );
    println!(
        "    Max buffer length: {} MB",
        max_buffer_length / (1024 * 1024)
    );
    println!(
        "    Recommended max working set: {} MB",
        recommended_max_working_set_size / (1024 * 1024)
    );
    Ok(())
}
