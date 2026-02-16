mod common;

use metal::*;
use objc2::{msg_send, rc::Retained, runtime::ProtocolObject};

use common::ExampleContext;

fn main() {
    if let Err(error) = run() {
        eprintln!("Example failed: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let context = ExampleContext::new()?;

    let descriptor = MTLIndirectCommandBufferDescriptor::new();
    descriptor.set_command_types(MTLIndirectCommandType::Draw);
    descriptor.set_inherit_pipeline_state(false);
    descriptor.set_inherit_buffers(false);
    descriptor.set_max_vertex_buffer_bind_count(4);
    descriptor.set_max_fragment_buffer_bind_count(4);

    let _icb: Option<Retained<ProtocolObject<dyn MTLIndirectCommandBuffer>>> = unsafe {
        msg_send![
            &*context.device,
            newIndirectCommandBufferWithDescriptor: &*descriptor,
            maxCommandCount: 8usize,
            options: MTLResourceOptions::empty()
        ]
    };

    println!("Indirect command buffers: created ICB with 8 max commands.");
    Ok(())
}
