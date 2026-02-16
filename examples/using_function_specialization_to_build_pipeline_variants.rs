mod common;

use std::{ffi::c_void, ptr::NonNull};

use metal::*;
use objc2_foundation::NSError;

use common::{ExampleContext, compile_library_from_source, ns_error_message};

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
        constant bool kUseRed [[function_constant(0)]];
        fragment float4 fs_color() {
            return kUseRed ? float4(1,0,0,1) : float4(0,0,1,1);
        }
        "#,
    )?;

    let constants = MTLFunctionConstantValues::new();
    let mut use_red = true;
    let ptr = NonNull::new((&mut use_red as *mut bool).cast::<c_void>())
        .ok_or_else(|| "Failed to create function-constant pointer".to_owned())?;
    constants.set_constant_value_type_at_index(ptr, MTLDataType::Bool, 0);

    let mut error: *mut NSError = std::ptr::null_mut();
    let function =
        library.new_function_with_name_constant_values_error("fs_color", &constants, &mut error);
    if function.is_none() {
        return Err(ns_error_message(error));
    }

    println!("Function constants: specialized fragment function with kUseRed=true.");
    Ok(())
}
