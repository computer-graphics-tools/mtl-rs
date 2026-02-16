# Metal Programming Guide for Rust

This book mirrors [Apple's Metal documentation](https://developer.apple.com/documentation/metal?language=objc)
with Rust examples using the [mtl-rs](https://github.com/computer-graphics-tools/mtl-rs) crate.

Each chapter corresponds to an Apple Metal documentation article. Where Apple provides
sample code, we provide a Rust port that you can run with `cargo run --example <name>`.

## Getting Started

```bash
git clone https://github.com/computer-graphics-tools/mtl-rs.git
cd mtl-rs
cargo run --example drawing_a_triangle_with_metal_4
```
