/// Port of Apple's "Loading textures and models using Metal fast resource loading" sample.
///
/// Demonstrates the MTLIO API for fast async resource loading: creating an
/// IO command queue, file handle, and loading buffer data from disk.
///
/// Apple sample: https://developer.apple.com/documentation/metal/loading-textures-and-models-using-metal-fast-resource-loading
mod common;

use metal::prelude::*;
use metal::*;

use common::ExampleContext;

fn main() {
    if let Err(error) = run() {
        eprintln!("Example failed: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let context = ExampleContext::new()?;

    // Create an IO command queue for fast resource loading.
    let io_queue_desc = MTLIOCommandQueueDescriptor::new();
    io_queue_desc.set_queue_type(MTLIOCommandQueueType::Concurrent);
    io_queue_desc.set_priority(MTLIOPriority::Normal);

    let io_queue = context
        .device
        .new_io_command_queue_with_descriptor(&io_queue_desc)
        .map_err(|e| format!("Failed to create IO command queue: {e}"))?;

    // Create an IO command buffer.
    let _io_cb = io_queue.command_buffer();

    // Create a destination buffer for loaded data.
    let buffer_size: usize = 4096;
    let _dest_buffer = context
        .device
        .new_buffer(buffer_size, MTLResourceOptions::STORAGE_MODE_SHARED)
        .ok_or("Failed to create destination buffer")?;

    // Note: To actually load from disk, you'd create an MTLIOFileHandle:
    //   let handle = device.new_io_handle_with_url(&url)?;
    //   io_cb.load_buffer(&dest_buffer, 0, buffer_size, &handle, 0);
    //   io_cb.commit();
    //   io_cb.wait_until_completed();
    //
    // Here we demonstrate the API setup without an actual file.

    println!("Loading textures and models using Metal fast resource loading:");
    println!("  Created IO command queue (Concurrent, Normal priority).");
    println!("  Created IO command buffer.");
    println!("  Created destination buffer ({buffer_size} bytes, Shared storage).");
    println!("  MTLIO pipeline ready for fast async resource loading.");
    Ok(())
}
