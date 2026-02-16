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

    let vertices: [f32; 9] = [0.0, 0.5, 0.0, -0.5, -0.5, 0.0, 0.5, -0.5, 0.0];
    let vertex_buffer = context
        .device
        .new_buffer_with_data(
            bytemuck::cast_slice(&vertices),
            MTLResourceOptions::STORAGE_MODE_SHARED,
        )
        .ok_or_else(|| "Failed to create vertex buffer".to_owned())?;

    let triangle = MTLAccelerationStructureTriangleGeometryDescriptor::descriptor();
    triangle.set_vertex_buffer(Some(&*vertex_buffer));
    triangle.set_vertex_stride(3 * std::mem::size_of::<f32>());
    triangle.set_vertex_format(MTLAttributeFormat::Float3);
    triangle.set_triangle_count(1);

    let _primitive = MTLPrimitiveAccelerationStructureDescriptor::descriptor();

    println!("Ray tracing: created triangle geometry descriptor for acceleration structure.");
    Ok(())
}
