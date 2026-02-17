# Rendering a scene with forward plus lighting using tile shaders

> [Apple Documentation](https://developer.apple.com/documentation/metal/rendering-a-scene-with-forward-plus-lighting-using-tile-shaders?language=objc)

Implement a forward plus renderer using the latest features on Apple GPUs.

## Run the Example

```bash
cargo run --example rendering_a_scene_with_forward_plus_lighting_using_tile_shaders
```

## Shader Files

- `Renderer/AAPLFairy.metal`
- `Renderer/AAPLForward.metal`
- `Renderer/AAPLCulling.metal`
- `Renderer/AAPLDepthPass.metal`

## Resources

- `Renderer/Assets.xcassets/StructureSpecularMap.textureset/Universal.mipmapset/StructureSpecular.png`
- `Renderer/Assets.xcassets/FoliageBaseColorMap.textureset/Universal.mipmapset/FoliageBaseColor.png`
- `Renderer/Assets.xcassets/StructureBaseColorMap.textureset/Universal.mipmapset/StructureBaseColor.png`
- `Renderer/Assets.xcassets/FoliageSpecularMap.textureset/Universal.mipmapset/FoliageSpecular.png`
- `Renderer/Assets.xcassets/StructureNormalMap.textureset/Universal.mipmapset/StructureNormals.png`
- `Renderer/Assets.xcassets/FoliageNormalMap.textureset/Universal.mipmapset/FoliageNormals.png`
- `Renderer/Meshes/Temple.obj`
- `Renderer/Meshes/Temple.mtl`

## Overview

> **Note:** This sample code project is associated with WWDC 2019 session [601: Modern Rendering with Metal](https://developer.apple.com/videos/play/wwdc19/601/).

### Configure the sample code project

To run the app:

- Build the project with Xcode 11 or later.
- Target an iOS device with an A11 chip or later and iOS 11 or later.
