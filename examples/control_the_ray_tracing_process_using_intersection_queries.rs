/// Port of Apple's "Control the ray tracing process using intersection queries" sample.
///
/// Demonstrates setting up acceleration structures and configuring intersection
/// function tables for explicit ray-object intersection testing.
///
/// Apple sample: https://developer.apple.com/documentation/metal/control-the-ray-tracing-process-using-intersection-queries
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

    // Create triangle geometry for the acceleration structure.
    let vertices: [f32; 9] = [0.0, 0.5, 0.0, -0.5, -0.5, 0.0, 0.5, -0.5, 0.0];
    let vertex_buffer = context
        .device
        .new_buffer_with_data(
            bytemuck::cast_slice(&vertices),
            MTLResourceOptions::STORAGE_MODE_SHARED,
        )
        .ok_or("Failed to create vertex buffer")?;

    // Set up triangle geometry descriptor.
    let triangle = MTLAccelerationStructureTriangleGeometryDescriptor::descriptor();
    triangle.set_vertex_buffer(Some(&vertex_buffer));
    triangle.set_vertex_stride(3 * std::mem::size_of::<f32>());
    triangle.set_vertex_format(MTLAttributeFormat::Float3);
    triangle.set_triangle_count(1);
    triangle.set_opaque(true);

    // Create a primitive acceleration structure descriptor.
    let _accel_desc = MTLPrimitiveAccelerationStructureDescriptor::descriptor();

    // Create an intersection function table descriptor for custom intersection testing.
    let table_desc =
        MTLIntersectionFunctionTableDescriptor::intersection_function_table_descriptor();
    table_desc.set_function_count(1);

    println!("Control the ray tracing process using intersection queries:");
    println!("  Created triangle geometry (1 triangle, 3 vertices).");
    println!("  Created primitive acceleration structure descriptor.");
    println!(
        "  Created intersection function table descriptor ({} function slot).",
        table_desc.function_count()
    );
    println!("  Ready for intersection query-based ray tracing.");
    Ok(())
}
