# Loading textures and models using Metal fast resource loading

> [Apple Documentation](https://developer.apple.com/documentation/metal/loading-textures-and-models-using-metal-fast-resource-loading?language=objc)

Stream texture and buffer data directly from disk into Metal resources using fast resource loading.

## Run the Example

```bash
cargo run --example loading_textures_and_models_using_metal_fast_resource_loading
```

## Shader Files

- `Renderer/Shaders/AAPLShaders.metal`

## Resources

- `Assets/texture1-hires.png`
- `Assets/texture3-lores.png`
- `Assets/texture3-lores.ktx`
- `Assets/texture1-hires.ktx`
- `Assets/texture2-lores.png`
- `Assets/texture2-lores.ktx`
- `Assets/texture3-hires.png`
- `Assets/texture1-lores.png`
- `Assets/texture1-lores.ktx`
- `Assets/texture3-hires.ktx`
- `Assets/texture2-hires.png`
- `Assets/texture2-hires.ktx`

## Overview

> **Note:** This sample code project is associated with WWDC22 session [10104: Load resources faster with Metal 3](https://developer.apple.com/wwdc22/10104/).

### Configure the sample code project

This sample code project requires the following:

- macOS 13 or later, and a Mac with Apple silicon
- Xcode 14 or later
