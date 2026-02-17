/// Port of Apple's "Accelerating ray tracing and motion blur using Metal" sample.
///
/// Demonstrates creating motion acceleration structures for ray tracing with
/// temporal motion blur. Uses motion triangle geometry descriptors with
/// multiple keyframe vertex buffers wrapped in MTLMotionKeyframeData.
///
/// Apple sample: https://developer.apple.com/documentation/metal/accelerating-ray-tracing-and-motion-blur-using-metal
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

    // Create two keyframes of a triangle: start position and end position.
    let keyframe_0: [f32; 9] = [0.0, 0.5, 0.0, -0.5, -0.5, 0.0, 0.5, -0.5, 0.0];
    let keyframe_1: [f32; 9] = [0.0, 0.7, 0.5, -0.7, -0.3, 0.5, 0.7, -0.3, 0.5];

    let vb0 = context
        .device
        .new_buffer_with_data(
            bytemuck::cast_slice(&keyframe_0),
            MTLResourceOptions::STORAGE_MODE_SHARED,
        )
        .ok_or("Failed to create keyframe 0 buffer")?;
    let vb1 = context
        .device
        .new_buffer_with_data(
            bytemuck::cast_slice(&keyframe_1),
            MTLResourceOptions::STORAGE_MODE_SHARED,
        )
        .ok_or("Failed to create keyframe 1 buffer")?;

    // Wrap buffers in MTLMotionKeyframeData.
    let kf0 = MTLMotionKeyframeData::data();
    kf0.set_buffer(Some(&vb0));
    let kf1 = MTLMotionKeyframeData::data();
    kf1.set_buffer(Some(&vb1));

    // Create a motion triangle geometry descriptor with 2 keyframes.
    let motion_tri = MTLAccelerationStructureMotionTriangleGeometryDescriptor::descriptor();
    motion_tri.set_vertex_buffers(&[&kf0, &kf1]);
    motion_tri.set_vertex_stride(3 * std::mem::size_of::<f32>());
    motion_tri.set_vertex_format(MTLAttributeFormat::Float3);
    motion_tri.set_triangle_count(1);

    // Create the primitive acceleration structure descriptor with motion.
    let accel_desc = MTLPrimitiveAccelerationStructureDescriptor::descriptor();
    accel_desc.set_motion_keyframe_count(2);
    accel_desc.set_motion_start_border_mode(MTLMotionBorderMode::Clamp);
    accel_desc.set_motion_end_border_mode(MTLMotionBorderMode::Clamp);
    accel_desc.set_motion_start_time(0.0);
    accel_desc.set_motion_end_time(1.0);

    let motion_start = accel_desc.motion_start_time();
    let motion_end = accel_desc.motion_end_time();
    let keyframes = accel_desc.motion_keyframe_count();

    println!("Accelerating ray tracing and motion blur using Metal:");
    println!("  Created motion triangle geometry with 2 keyframes.");
    println!("  Primitive acceleration structure descriptor:");
    println!("    Motion time range: {motion_start} .. {motion_end}");
    println!("    Keyframe count: {keyframes}");
    println!("    Border mode: Clamp (both start and end)");
    println!("  Ready for motion blur ray tracing.");
    Ok(())
}
