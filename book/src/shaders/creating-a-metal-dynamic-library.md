# Creating a Metal dynamic library

> [Apple Documentation](https://developer.apple.com/documentation/metal/creating-a-metal-dynamic-library?language=objc)

Compile a library of shaders and write it to a file as a dynamically linked library.

## Run the Example

```bash
cargo run --example creating_a_metal_dynamic_library
```

## Shader Files

- `Renderer/AAPLUserDylib.metal`
- `Renderer/AAPLUserCompiledFunction.metal`
- `Renderer/AAPLShaders.metal`

## Resources

- `Assets.xcassets/ColorMap.textureset/Universal.mipmapset/ColorMap.png`

## Overview

> **Note:** This sample code project is associated with WWDC20 session [10615: Build GPU Binaries with Metal](https://developer.apple.com/wwdc20/10615/).

### Configure the sample code project

This sample requires the following system and software configuration:

- macOS 10.16 or later
- iOS 14 or later
- Xcode 12 or later
