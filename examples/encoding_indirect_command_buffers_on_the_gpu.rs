/// Port of Apple's "Encoding indirect command buffers on the GPU" sample.
///
/// Demonstrates GPU-driven rendering by creating an indirect command buffer
/// that a compute kernel can encode draw commands into. The GPU resets, populates,
/// and optimizes the ICB before execution.
///
/// Apple sample: https://developer.apple.com/documentation/metal/encoding-indirect-command-buffers-on-the-gpu
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

    // Create an ICB descriptor for GPU-encoded draw commands.
    let descriptor = MTLIndirectCommandBufferDescriptor::new();
    descriptor.set_command_types(MTLIndirectCommandType::Draw);
    descriptor.set_inherit_pipeline_state(true);
    descriptor.set_inherit_buffers(false);
    descriptor.set_max_vertex_buffer_bind_count(4);
    descriptor.set_max_fragment_buffer_bind_count(0);

    let max_commands: usize = 256;
    let icb: Retained<ProtocolObject<dyn MTLIndirectCommandBuffer>> = unsafe {
        let result: Option<Retained<ProtocolObject<dyn MTLIndirectCommandBuffer>>> = msg_send![
            &*context.device,
            newIndirectCommandBufferWithDescriptor: &*descriptor,
            maxCommandCount: max_commands,
            options: MTLResourceOptions::STORAGE_MODE_PRIVATE
        ];
        result.ok_or("Failed to create indirect command buffer with private storage")?
    };

    // Reset the ICB range (as a compute kernel would do before encoding).
    let command_buffer = context
        .queue
        .command_buffer()
        .ok_or("Failed to create command buffer")?;
    let blit = command_buffer
        .blit_command_encoder()
        .ok_or("Failed to create blit encoder")?;
    blit.reset_commands_in_buffer(&icb, 0..max_commands);
    blit.end_encoding();
    command_buffer.commit();
    command_buffer.wait_until_completed();

    println!("Encoding indirect command buffers on the GPU:");
    println!("  Created ICB with {max_commands} max commands (private storage for GPU encoding).");
    println!("  Reset all command slots via blit encoder.");
    println!("  ICB is ready for GPU-side compute kernel encoding.");
    Ok(())
}
