mod common;

use metal::prelude::*;
use metal::*;

use common::{ExampleContext, compile_library_from_source, retained_error_message};

fn main() {
    if let Err(error) = run() {
        eprintln!("Example failed: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let context = ExampleContext::new()?;

    // Run a compute pass that could populate an argument buffer.
    let values: Vec<u32> = (0..64).collect();
    let bytes = bytemuck::cast_slice(&values);
    let buffer = context
        .device
        .new_buffer_with_data(bytes, MTLResourceOptions::STORAGE_MODE_SHARED)
        .ok_or_else(|| "Failed to allocate buffer".to_owned())?;

    let library = compile_library_from_source(
        &context.device,
        r#"
        #include <metal_stdlib>
        using namespace metal;
        kernel void fill(device uint* values [[buffer(0)]], uint tid [[thread_position_in_grid]]) {
            values[tid] = values[tid] * 2;
        }
        "#,
    )?;
    let function = library
        .new_function_with_name("fill")
        .ok_or_else(|| "Missing fill kernel".to_owned())?;
    let pipeline = context
        .device
        .new_compute_pipeline_state_with_function(&*function)
        .map_err(|error| retained_error_message(&error))?;

    let command_buffer = context
        .queue
        .command_buffer()
        .ok_or_else(|| "Failed to create command buffer".to_owned())?;
    let encoder = command_buffer
        .new_compute_command_encoder()
        .ok_or_else(|| "Failed to create compute encoder".to_owned())?;
    encoder.set_compute_pipeline_state(&*pipeline);
    encoder.set_buffer(Some(&*buffer), 0, 0);
    encoder.dispatch_threads(MTLSize::new(values.len(), 1, 1), MTLSize::new(64, 1, 1));
    encoder.end_encoding();
    command_buffer.commit();
    command_buffer.wait_until_completed();

    // Set up argument buffer.
    let arg_desc = MTLArgumentDescriptor::new();
    arg_desc.set_data_type(MTLDataType::Pointer);
    arg_desc.set_access(MTLBindingAccess::ReadOnly);
    arg_desc.set_index(0);

    let arg_encoder = context
        .device
        .new_argument_encoder_with_arguments(&[&arg_desc])
        .ok_or_else(|| "Failed to create argument encoder".to_owned())?;

    let argument_buffer = context
        .device
        .new_buffer(
            arg_encoder.encoded_length(),
            MTLResourceOptions::STORAGE_MODE_SHARED,
        )
        .ok_or_else(|| "Failed to allocate argument buffer".to_owned())?;
    arg_encoder.set_argument_buffer(Some(&*argument_buffer), 0);

    println!("Argument buffers GPU encoding: compute pass + argument buffer setup complete.");
    Ok(())
}
