# Adjusting the level of detail using Metal mesh shaders

> [Apple Documentation](https://developer.apple.com/documentation/metal/adjusting-the-level-of-detail-using-metal-mesh-shaders?language=objc)

Choose and render meshes with several levels of detail using object and mesh shaders.

## Run the Example

```bash
cargo run --example adjusting_the_level_of_detail_using_metal_mesh_shaders
```

## Shader Files

- `MeshShadersMetalCPP/Renderer/AAPLShaders.metal`

## Resources

- `MeshShadersMetalCPP/Application/iOS/Assets.xcassets/ColorMap.textureset/Universal.mipmapset/ColorMap.png`

## Overview

> **Note:** This sample code project is associated with WWDC22 session [10162: Transform your geometry with Metal mesh shaders](https://developer.apple.com/wwdc22/10162/).

### Configure the sample code project

To run this sample, you need Xcode 14 or later, and a physical device that supports [`MTLGPUFamilyMac2`](https://developer.apple.com/documentation/metal/mtlgpufamily/mac2?language=objc) or [`MTLGPUFamilyApple7`](https://developer.apple.com/documentation/metal/mtlgpufamily/apple7?language=objc), such as:

- A Mac running macOS 13 or later
- An iOS device with an A15 chip or later running iOS 16 or later

This sample can only run on a physical device because it uses mesh shader features, which Simulator doesn’t support.
