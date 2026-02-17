/// Port of Apple's "Customizing a PyTorch operation" sample.
///
/// Demonstrates a custom Metal compute kernel implementing the soft-shrink
/// activation function: f(x) = x-lambda if x>lambda, x+lambda if x<-lambda, else 0.
///
/// Apple sample: https://developer.apple.com/documentation/metal/customizing-a-pytorch-operation
mod common;

use metal::prelude::*;

use common::{ExampleContext, compile_library_from_source, retained_error_message};

const N: usize = 1024;

fn main() {
    if let Err(error) = run() {
        eprintln!("Example failed: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let context = ExampleContext::new()?;

    let library = compile_library_from_source(
        &context.device,
        r#"
        #include <metal_stdlib>
        using namespace metal;

        kernel void softshrink(device const float* input [[buffer(0)]],
                               device float* output       [[buffer(1)]],
                               device const float& lambda [[buffer(2)]],
                               uint i [[thread_position_in_grid]]) {
            float x = input[i];
            output[i] = x > lambda ? x - lambda : (x < -lambda ? x + lambda : 0.0f);
        }
        "#,
    )?;

    let function = library
        .new_function_with_name("softshrink")
        .ok_or("Missing softshrink kernel")?;
    let pipeline = context
        .device
        .new_compute_pipeline_state_with_function(&function)
        .map_err(|e| retained_error_message(&e))?;

    // Generate input: [-2.0, -1.5, -1.0, ..., 2.0] spread across N elements.
    let input: Vec<f32> = (0..N)
        .map(|i| -2.0 + 4.0 * i as f32 / (N - 1) as f32)
        .collect();
    let lambda: f32 = 0.5;

    let buf_in = context
        .device
        .new_buffer_with_data(
            bytemuck::cast_slice(&input),
            MTLResourceOptions::STORAGE_MODE_SHARED,
        )
        .ok_or("Failed to create input buffer")?;
    let buf_out = context
        .device
        .new_buffer(
            N * std::mem::size_of::<f32>(),
            MTLResourceOptions::STORAGE_MODE_SHARED,
        )
        .ok_or("Failed to create output buffer")?;
    let buf_lambda = context
        .device
        .new_buffer_with_data(
            bytemuck::bytes_of(&lambda),
            MTLResourceOptions::STORAGE_MODE_SHARED,
        )
        .ok_or("Failed to create lambda buffer")?;

    let cb = context.queue.command_buffer().ok_or("No command buffer")?;
    let enc = cb.compute_command_encoder().ok_or("No compute encoder")?;
    enc.set_compute_pipeline_state(&pipeline);
    enc.set_buffer(Some(&buf_in), 0, 0);
    enc.set_buffer(Some(&buf_out), 0, 1);
    enc.set_buffer(Some(&buf_lambda), 0, 2);
    let max_threads = pipeline.max_total_threads_per_threadgroup();
    enc.dispatch_threads(
        MTLSize::new(N, 1, 1),
        MTLSize::new(max_threads.min(N), 1, 1),
    );
    enc.end_encoding();
    cb.commit();
    cb.wait_until_completed();

    let result =
        unsafe { std::slice::from_raw_parts(buf_out.contents().as_ptr().cast::<f32>(), N) };

    // Verify.
    let mut errors = 0;
    for i in 0..N {
        let x = input[i];
        let expected = if x > lambda {
            x - lambda
        } else if x < -lambda {
            x + lambda
        } else {
            0.0
        };
        if (result[i] - expected).abs() > 1e-6 {
            errors += 1;
        }
    }

    if errors == 0 {
        println!(
            "Customizing a PyTorch operation: soft-shrink verified ({N} elements, lambda={lambda})."
        );
    } else {
        println!("Customizing a PyTorch operation: {errors} mismatches out of {N}.");
    }
    Ok(())
}
