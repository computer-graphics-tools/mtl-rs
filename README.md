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
mtl-rs = "0.1.7"
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

Run with `cargo run --example <name>`:

- `drawing_a_triangle_with_metal_4` — Render a rotating triangle using Metal

### Platform requirements

- macOS with a Metal-capable GPU
- Xcode command line tools installed
- Rust toolchain with Apple targets

## License

MIT
