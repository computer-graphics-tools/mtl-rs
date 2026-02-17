/// Port of Apple's "Rendering a curve primitive in a ray tracing scene" sample.
///
/// Demonstrates creating curve geometry descriptors for use in ray tracing
/// acceleration structures. Curve primitives allow efficient rendering of
/// hair, fur, and other thin geometry in ray-traced scenes.
///
/// Apple sample: https://developer.apple.com/documentation/metal/rendering-a-curve-primitive-in-a-ray-tracing-scene
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

    // Define control points for a cubic Bezier curve.
    let control_points: [f32; 12] = [
        0.0, 0.0, 0.0, // P0
        0.0, 0.5, 0.0, // P1
        0.5, 0.5, 0.0, // P2
        0.5, 0.0, 0.0, // P3
    ];
    let control_point_buffer = context
        .device
        .new_buffer_with_data(
            bytemuck::cast_slice(&control_points),
            MTLResourceOptions::STORAGE_MODE_SHARED,
        )
        .ok_or("Failed to create control point buffer")?;

    // Define per-control-point radii (varying thickness along the curve).
    let radii: [f32; 4] = [0.02, 0.015, 0.015, 0.02];
    let radius_buffer = context
        .device
        .new_buffer_with_data(
            bytemuck::cast_slice(&radii),
            MTLResourceOptions::STORAGE_MODE_SHARED,
        )
        .ok_or("Failed to create radius buffer")?;

    // Create a curve geometry descriptor.
    let curve = MTLAccelerationStructureCurveGeometryDescriptor::descriptor();
    curve.set_control_point_buffer(Some(&control_point_buffer));
    curve.set_control_point_count(4);
    curve.set_control_point_stride(3 * std::mem::size_of::<f32>());
    curve.set_control_point_format(MTLAttributeFormat::Float3);
    curve.set_radius_buffer(Some(&radius_buffer));
    curve.set_radius_stride(std::mem::size_of::<f32>());
    curve.set_radius_format(MTLAttributeFormat::Float);
    curve.set_segment_count(1);
    curve.set_segment_control_point_count(4);
    curve.set_curve_type(MTLCurveType::Round);
    curve.set_curve_basis(MTLCurveBasis::BSpline);

    // Create a primitive acceleration structure descriptor with the curve.
    let _accel_desc = MTLPrimitiveAccelerationStructureDescriptor::descriptor();

    println!("Rendering a curve primitive in a ray tracing scene:");
    println!("  Created curve geometry with 4 control points.");
    println!("  Curve type: Round, Basis: BSpline");
    println!("  Radii range: {:.3} to {:.3}", radii[0], radii[3]);
    println!("  Ready for acceleration structure build.");
    Ok(())
}
