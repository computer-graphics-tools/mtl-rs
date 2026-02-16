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
mtl-rs = "0.1.1"
```

## Usage

```rust
use metal::{create_system_default_device, MTLDevice};

fn main() {
    let device = create_system_default_device()
        .expect("No Metal device found");

    println!("Device: {:?}", device.name());
    println!("Unified memory: {}", device.has_unified_memory());
    println!("Max threadgroup size: {:?}", device.max_threads_per_threadgroup());
}
```

## Apple Sample Catalog Examples

The repository now includes a full Apple-sample catalog in Rust.

- List all catalog entries with status:
  - `cargo run --example catalog_index`
- Run a specific sample by key:
  - `cargo run --example <catalog_key>`

### Platform Requirements

- macOS with a Metal-capable GPU
- Xcode command line tools installed
- Rust toolchain with Apple targets

### Fallback Coverage

Some Apple samples depend on APIs outside current `mtl-rs` coverage and are implemented as explicit fallbacks:

- `achieving_smooth_frame_rates_with_display_link`
- `smooth_frame_rates_with_metal_display_link_duplicate`
- `metalfx_temporal_aa_upscaling`
- `migrating_opengl_to_metal`
- `mixing_metal_and_opengl`

### Catalog Keys

Run any of the following with `cargo run --example <key>`:

- `using_render_pipeline_to_render_primitives`
- `performing_calculations_on_a_gpu`
- `creating_and_sampling_textures`
- `calculating_primitive_visibility_using_depth_testing`
- `synchronizing_cpu_and_gpu_work`
- `customizing_render_pass_setup`
- `processing_a_texture_in_a_compute_function`
- `reading_pixel_data_from_a_drawable_texture`
- `achieving_smooth_frame_rates_with_display_link`
- `improving_edge_quality_with_msaa`
- `culling_occluded_geometry_visibility_buffer`
- `streaming_large_images_with_sparse_textures`
- `managing_resources_with_argument_buffers`
- `argument_buffers_with_resource_heaps`
- `encoding_argument_buffers_on_gpu`
- `indirect_command_buffers_cpu`
- `indirect_command_buffers_gpu`
- `adjusting_lod_with_mesh_shaders`
- `loading_with_fast_resource_loading`
- `metalfx_temporal_aa_upscaling`
- `creating_a_metal_dynamic_library`
- `function_pointers_and_stitching`
- `accelerating_ray_tracing`
- `intersection_queries_in_ray_tracing`
- `rendering_curve_primitives_ray_tracing`
- `basic_tessellation`
- `migrating_opengl_to_metal`
- `mixing_metal_and_opengl`
- `capturing_metal_commands_programmatically`
- `supporting_simulator_in_a_metal_app`
- `modern_deferred_tiled_rendering`
- `order_independent_transparency_image_blocks`
- `function_specialization_lod`
- `reduced_pass_reflections_layer_selection`
- `terrain_with_argument_buffers`
- `deferred_lighting_objective_c`
- `deferred_lighting_cpp`
- `deferred_lighting_swift`
- `tensorflow_pytorch_custom_operations`
- `starting_a_game_port_with_metal`
- `porting_tessellation_geometry_instancing_pipelines`
- `porting_ray_tracing_pipelines`
- `porting_ray_query_pipelines`
- `function_constants_and_framebuffer_fetch`
- `ray_tracing_with_intersection_function_buffer`
- `smooth_frame_rates_with_metal_display_link_duplicate`

## License

MIT
