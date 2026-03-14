# mtl-rs

Rust bindings for Apple's Metal API, built on the modern `objc2` ecosystem.

## Features

- Comprehensive Metal API coverage
- MTL4 (Metal 4) API bindings
- Built on `objc2` for safe and modern Objective-C interop
- Supports macOS and iOS

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
mtl-rs = "0.1.6"
```

## Usage

```rust
use metal::MTLDevice;

fn main() {
    let device = <dyn MTLDevice>::system_default()
        .expect("No Metal device found");

    println!("Device: {:?}", device.name());
    println!("Unified memory: {}", device.has_unified_memory());
    println!("Max threadgroup size: {:?}", device.max_threads_per_threadgroup());
}
```

## Examples

Run any example with `cargo run --example <name>`.

### Essentials

- `drawing_a_triangle_with_metal_4` — Render a rotating triangle using Metal 4

### Compute

- `performing_calculations_on_a_gpu` — Add two arrays on the GPU

### Render workflows

- `customizing_render_pass_setup` — Configure render pass descriptors
- `calculating_primitive_visibility_using_depth_testing` — Depth-stencil state setup
- `encoding_indirect_command_buffers_on_the_cpu` — Indirect command buffer encoding
- `improving_edge_rendering_quality_with_msaa` — Multisample antialiasing

### Textures

- `processing_a_texture_in_a_compute_function` — Grayscale conversion in a compute shader
- `streaming_large_images_with_metal_sparse_textures` — Sparse texture creation

### Argument buffers

- `managing_groups_of_resources_with_argument_buffers` — Argument encoder setup
- `using_argument_buffers_with_resource_heaps` — Heaps with argument buffers
- `encoding_argument_buffers_on_the_gpu` — GPU-side argument buffer encoding

### Shaders

- `creating_a_metal_dynamic_library` — Compile and use a dynamic Metal library
- `using_function_specialization_to_build_pipeline_variants` — Function constants for pipeline variants

### Synchronization

- `synchronizing_cpu_and_gpu_work` — Triple-buffered resource management

### Ray tracing

- `accelerating_ray_tracing_using_metal` — Acceleration structure setup

### Developer tools

- `capturing_metal_commands_programmatically` — GPU capture with `MTLCaptureManager`

### Platform requirements

- macOS with a Metal-capable GPU
- Xcode command line tools installed
- Rust toolchain with Apple targets

## License

MIT
