/// Port of Apple's "Customizing a TensorFlow operation" sample.
///
/// Demonstrates a custom Metal compute kernel for hash grid encoding,
/// a technique used in neural radiance fields. The kernel maps input
/// coordinates to feature vectors via a learned hash table.
///
/// Apple sample: https://developer.apple.com/documentation/metal/customizing-a-tensorflow-operation
mod common;

use metal::prelude::*;

use common::{ExampleContext, compile_library_from_source, retained_error_message};

const N_POINTS: usize = 64;
const TABLE_SIZE: usize = 256;
const N_FEATURES: usize = 2;

fn main() {
    if let Err(error) = run() {
        eprintln!("Example failed: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let context = ExampleContext::new()?;

    // Simplified hash encoding kernel (2D, single level).
    let library = compile_library_from_source(
        &context.device,
        r#"
        #include <metal_stdlib>
        using namespace metal;

        kernel void hash_encode(device const float2* positions [[buffer(0)]],
                                device const float*  table     [[buffer(1)]],
                                device float2*       output    [[buffer(2)]],
                                constant uint&       table_sz  [[buffer(3)]],
                                constant float&      scale     [[buffer(4)]],
                                uint i [[thread_position_in_grid]]) {
            float2 p = positions[i] * scale;
            int2 c = int2(floor(p));
            float2 f = p - float2(c);

            // Hash corners of the grid cell.
            auto hash = [&](int2 v) -> uint {
                return (uint(v.x) * 2654435761u ^ uint(v.y) * 2246822519u) % table_sz;
            };

            uint h00 = hash(c);
            uint h10 = hash(c + int2(1, 0));
            uint h01 = hash(c + int2(0, 1));
            uint h11 = hash(c + int2(1, 1));

            // Bilinear interpolation of 2 features per corner.
            for (uint feat = 0; feat < 2; feat++) {
                float v00 = table[h00 * 2 + feat];
                float v10 = table[h10 * 2 + feat];
                float v01 = table[h01 * 2 + feat];
                float v11 = table[h11 * 2 + feat];
                float top    = mix(v00, v10, f.x);
                float bottom = mix(v01, v11, f.x);
                output[i][feat] = mix(top, bottom, f.y);
            }
        }
        "#,
    )?;

    let function = library
        .new_function_with_name("hash_encode")
        .ok_or("Missing hash_encode kernel")?;
    let pipeline = context
        .device
        .new_compute_pipeline_state_with_function(&function)
        .map_err(|e| retained_error_message(&e))?;

    // Input: random 2D positions in [0, 1].
    let positions: Vec<[f32; 2]> = (0..N_POINTS)
        .map(|i| {
            let t = i as f32 / N_POINTS as f32;
            [t, t * 0.7 + 0.1]
        })
        .collect();

    // Hash table: random features.
    let mut table = vec![0.0f32; TABLE_SIZE * N_FEATURES];
    let mut seed: u32 = 123;
    for v in &mut table {
        seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
        *v = (seed >> 16) as f32 / 65535.0 - 0.5;
    }

    let scale: f32 = 16.0;
    let table_sz: u32 = TABLE_SIZE as u32;

    let buf_pos = context
        .device
        .new_buffer_with_data(
            bytemuck::cast_slice(&positions),
            MTLResourceOptions::STORAGE_MODE_SHARED,
        )
        .ok_or("Failed to create positions buffer")?;
    let buf_table = context
        .device
        .new_buffer_with_data(
            bytemuck::cast_slice(&table),
            MTLResourceOptions::STORAGE_MODE_SHARED,
        )
        .ok_or("Failed to create table buffer")?;
    let buf_out = context
        .device
        .new_buffer(
            N_POINTS * N_FEATURES * std::mem::size_of::<f32>(),
            MTLResourceOptions::STORAGE_MODE_SHARED,
        )
        .ok_or("Failed to create output buffer")?;
    let buf_sz = context
        .device
        .new_buffer_with_data(
            bytemuck::bytes_of(&table_sz),
            MTLResourceOptions::STORAGE_MODE_SHARED,
        )
        .ok_or("Failed to create table_sz buffer")?;
    let buf_scale = context
        .device
        .new_buffer_with_data(
            bytemuck::bytes_of(&scale),
            MTLResourceOptions::STORAGE_MODE_SHARED,
        )
        .ok_or("Failed to create scale buffer")?;

    let cb = context.queue.command_buffer().ok_or("No command buffer")?;
    let enc = cb.compute_command_encoder().ok_or("No compute encoder")?;
    enc.set_compute_pipeline_state(&pipeline);
    enc.set_buffer(Some(&buf_pos), 0, 0);
    enc.set_buffer(Some(&buf_table), 0, 1);
    enc.set_buffer(Some(&buf_out), 0, 2);
    enc.set_buffer(Some(&buf_sz), 0, 3);
    enc.set_buffer(Some(&buf_scale), 0, 4);
    let max_threads = pipeline.max_total_threads_per_threadgroup();
    enc.dispatch_threads(
        MTLSize::new(N_POINTS, 1, 1),
        MTLSize::new(max_threads.min(N_POINTS), 1, 1),
    );
    enc.end_encoding();
    cb.commit();
    cb.wait_until_completed();

    let result = unsafe {
        std::slice::from_raw_parts(buf_out.contents().as_ptr().cast::<[f32; 2]>(), N_POINTS)
    };

    println!("Customizing a TensorFlow operation: hash encoding computed for {N_POINTS} points.");
    println!(
        "  First 3 outputs: [{:.4}, {:.4}], [{:.4}, {:.4}], [{:.4}, {:.4}]",
        result[0][0], result[0][1], result[1][0], result[1][1], result[2][0], result[2][1]
    );
    Ok(())
}
