mod common;

use metal::prelude::*;

use common::{ExampleContext, compile_library_from_source, retained_error_message};

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
        kernel void noop(device uint* data [[buffer(0)]], uint tid [[thread_position_in_grid]]) {
            data[tid] = data[tid];
        }
        "#,
    )?;

    match context.device.new_dynamic_library(&*library) {
        Ok(_) => println!("Dynamic libraries: created dynamic library from compiled source."),
        Err(error) => {
            // Some devices/toolchains reject ad-hoc in-memory libraries for dynamic conversion.
            let msg = retained_error_message(&error);
            println!(
                "Dynamic libraries: compiled library successfully (dynamic library creation not supported: {msg})."
            );
        }
    }

    Ok(())
}
