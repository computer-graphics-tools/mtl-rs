mod common;

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

    let descriptor = MTLDepthStencilDescriptor::new();
    descriptor.set_depth_compare_function(MTLCompareFunction::Less);
    descriptor.set_depth_write_enabled(true);
    let _state = context
        .device
        .new_depth_stencil_state_with_descriptor(&descriptor)
        .ok_or_else(|| "Failed to create depth-stencil state".to_owned())?;

    println!("Depth testing: created depth-stencil state with Less compare function.");
    Ok(())
}
