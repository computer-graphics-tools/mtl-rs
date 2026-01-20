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
mtl-rs = "0.1.0"
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

## Platforms

- macOS (x86_64, aarch64)
- iOS (aarch64)

## License

MIT
