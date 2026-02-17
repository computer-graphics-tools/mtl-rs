/// Port of Apple's "Running a machine learning model on the GPU timeline" sample.
///
/// Demonstrates creating Metal 4 tensor descriptors and configuring them for
/// machine learning workloads. The full ML pipeline requires a .mtlpackage
/// model file; this example shows the tensor descriptor API.
///
/// Apple sample: https://developer.apple.com/documentation/metal/running-a-machine-learning-model-on-the-gpu-timeline
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

    // Check Metal 4 support.
    if !context.device.supports_family(MTLGPUFamily::Metal4) {
        println!("Running a machine learning model on the GPU timeline:");
        println!("  Metal 4 not supported on this device — skipping ML pipeline demo.");
        return Ok(());
    }

    // Create tensor descriptors for a matrix multiply: C[4,3] = A[4,8] * B[8,3].
    let dims_a = MTLTensorExtents::new_with_rank_values(2, Some(&[4, 8]))
        .ok_or("Failed to create extents A")?;
    let dims_b = MTLTensorExtents::new_with_rank_values(2, Some(&[8, 3]))
        .ok_or("Failed to create extents B")?;
    let dims_c = MTLTensorExtents::new_with_rank_values(2, Some(&[4, 3]))
        .ok_or("Failed to create extents C")?;

    let desc_a = MTLTensorDescriptor::new();
    desc_a.set_data_type(MTLTensorDataType::Float32);
    desc_a.set_dimensions(&dims_a);
    desc_a.set_usage(MTLTensorUsage::MACHINE_LEARNING);

    let desc_b = MTLTensorDescriptor::new();
    desc_b.set_data_type(MTLTensorDataType::Float32);
    desc_b.set_dimensions(&dims_b);
    desc_b.set_usage(MTLTensorUsage::MACHINE_LEARNING);

    let desc_c = MTLTensorDescriptor::new();
    desc_c.set_data_type(MTLTensorDataType::Float32);
    desc_c.set_dimensions(&dims_c);
    desc_c.set_usage(MTLTensorUsage::MACHINE_LEARNING);

    println!("Running a machine learning model on the GPU timeline:");
    println!("  Metal 4 supported.");
    println!(
        "  Tensor A: [{}, {}] Float32",
        dims_a.extent_at_dimension_index(0),
        dims_a.extent_at_dimension_index(1)
    );
    println!(
        "  Tensor B: [{}, {}] Float32",
        dims_b.extent_at_dimension_index(0),
        dims_b.extent_at_dimension_index(1)
    );
    println!(
        "  Tensor C: [{}, {}] Float32",
        dims_c.extent_at_dimension_index(0),
        dims_c.extent_at_dimension_index(1)
    );
    println!("  ML pipeline setup requires a .mtlpackage model file for execution.");
    Ok(())
}
