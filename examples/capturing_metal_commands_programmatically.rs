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

    let manager = MTLCaptureManager::shared_capture_manager();
    let scope = manager.new_capture_scope_with_command_queue(&*context.queue);
    manager.set_default_capture_scope(Some(&*scope));

    println!("Capture: created capture scope and set as default.");
    Ok(())
}
