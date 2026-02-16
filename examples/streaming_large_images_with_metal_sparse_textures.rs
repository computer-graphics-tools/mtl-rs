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

    let descriptor =
        MTLTextureDescriptor::texture_2d_descriptor_with_pixel_format_width_height_mipmapped(
            MTLPixelFormat::RGBA8Unorm,
            256,
            256,
            true,
        );
    descriptor.set_usage(MTLTextureUsage::SHADER_READ | MTLTextureUsage::SHADER_WRITE);
    descriptor.set_placement_sparse_page_size(MTLSparsePageSize::KB64);
    let _ = context.device.new_texture_with_descriptor(&descriptor);

    println!("Sparse textures: created 256x256 sparse texture with 64KB page size.");
    Ok(())
}
