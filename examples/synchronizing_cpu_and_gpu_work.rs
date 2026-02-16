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

    for frame_index in 0..3usize {
        let values = vec![frame_index as u32; 64];
        let bytes = bytemuck::cast_slice(&values);
        let buffer = context
            .device
            .new_buffer_with_data(bytes, MTLResourceOptions::STORAGE_MODE_SHARED)
            .ok_or_else(|| "Failed to allocate per-frame buffer".to_owned())?;

        let ptr = buffer.contents().as_ptr().cast::<u32>();
        unsafe {
            *ptr = frame_index as u32 + 10;
        }

        println!(
            "Frame {frame_index}: buffer[0] updated to {}",
            frame_index + 10
        );
    }

    println!("CPU/GPU sync: triple-buffered resource cycling complete.");
    Ok(())
}
