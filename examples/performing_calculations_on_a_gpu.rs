/// Port of Apple's "Performing Calculations on a GPU" sample.
///
/// Demonstrates using Metal to add two arrays of floats on the GPU,
/// then validates the results on the CPU.
///
/// Apple sample: https://developer.apple.com/documentation/metal/performing-calculations-on-a-gpu
mod common;

use metal::prelude::*;

use common::{ExampleContext, compile_library_from_source, retained_error_message};

const ARRAY_LENGTH: usize = 1 << 24; // 16,777,216 elements — matches Apple sample

fn main() {
    if let Err(error) = run() {
        eprintln!("Example failed: {error}");
        std::process::exit(1);
    }
}

fn generate_random_floats(count: usize) -> Vec<f32> {
    // Simple deterministic pseudo-random for reproducibility.
    let mut values = Vec::with_capacity(count);
    let mut seed: u32 = 42;
    for _ in 0..count {
        seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
        values.push((seed >> 16) as f32 / 65535.0);
    }
    values
}

fn run() -> Result<(), String> {
    let context = ExampleContext::new()?;

    // Generate random input data — matches Apple sample's generateRandomFloatData.
    let input_a = generate_random_floats(ARRAY_LENGTH);
    let input_b = generate_random_floats(ARRAY_LENGTH);

    let buffer_size = ARRAY_LENGTH * std::mem::size_of::<f32>();

    let buffer_a = context
        .device
        .new_buffer_with_data(
            bytemuck::cast_slice(&input_a),
            MTLResourceOptions::STORAGE_MODE_SHARED,
        )
        .ok_or_else(|| "Failed to allocate buffer A".to_owned())?;

    let buffer_b = context
        .device
        .new_buffer_with_data(
            bytemuck::cast_slice(&input_b),
            MTLResourceOptions::STORAGE_MODE_SHARED,
        )
        .ok_or_else(|| "Failed to allocate buffer B".to_owned())?;

    let buffer_result = context
        .device
        .new_buffer(buffer_size, MTLResourceOptions::STORAGE_MODE_SHARED)
        .ok_or_else(|| "Failed to allocate result buffer".to_owned())?;

    // Compile the add_arrays kernel — equivalent to add.metal in the Apple sample.
    let library = compile_library_from_source(
        &context.device,
        r#"
        #include <metal_stdlib>
        using namespace metal;

        kernel void add_arrays(device const float* inA,
                               device const float* inB,
                               device float* result,
                               uint index [[thread_position_in_grid]]) {
            result[index] = inA[index] + inB[index];
        }
        "#,
    )?;

    let function = library
        .new_function_with_name("add_arrays")
        .ok_or_else(|| "Missing add_arrays kernel".to_owned())?;

    let pipeline = context
        .device
        .new_compute_pipeline_state_with_function(&*function)
        .map_err(|error| retained_error_message(&error))?;

    // Encode and dispatch.
    let command_buffer = context
        .queue
        .command_buffer()
        .ok_or_else(|| "Failed to create command buffer".to_owned())?;

    let encoder = command_buffer
        .compute_command_encoder()
        .ok_or_else(|| "Failed to create compute encoder".to_owned())?;

    encoder.set_compute_pipeline_state(&*pipeline);
    encoder.set_buffer(Some(&*buffer_a), 0, 0);
    encoder.set_buffer(Some(&*buffer_b), 0, 1);
    encoder.set_buffer(Some(&*buffer_result), 0, 2);

    let grid_size = MTLSize::new(ARRAY_LENGTH, 1, 1);
    let max_threads = pipeline.max_total_threads_per_threadgroup();
    let threadgroup_size = MTLSize::new(max_threads.min(ARRAY_LENGTH), 1, 1);

    encoder.dispatch_threads(grid_size, threadgroup_size);
    encoder.end_encoding();

    command_buffer.commit();
    command_buffer.wait_until_completed();

    // Verify results — matches Apple sample's verifyResults.
    let result = unsafe {
        std::slice::from_raw_parts(
            buffer_result.contents().as_ptr().cast::<f32>(),
            ARRAY_LENGTH,
        )
    };

    for i in 0..ARRAY_LENGTH {
        let expected = input_a[i] + input_b[i];
        if result[i] != expected {
            return Err(format!(
                "Compute ERROR: index={i} result={} vs {expected}=a+b",
                result[i]
            ));
        }
    }

    println!("Compute results as expected ({ARRAY_LENGTH} elements verified).");
    Ok(())
}
