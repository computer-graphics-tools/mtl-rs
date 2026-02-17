# Rendering reflections in real time using ray tracing

> [Apple Documentation](https://developer.apple.com/documentation/metal/rendering-reflections-in-real-time-using-ray-tracing?language=objc)

Implement realistic real-time lighting by dynamically generating reflection maps by encoding a ray-tracing compute pass.

## Run the Example

```bash
cargo run --example rendering_reflections_in_real_time_using_ray_tracing
```

## Shader Files

- `Renderer/AAPLShaders.metal`

## Resources

- `Renderer/kloppenheim_06_4k.hdr`
- `Renderer/Assets.xcassets/checkerboard_gray.textureset/Universal.mipmapset/checkerboard_gray.png`
- `Renderer/Assets.xcassets/AccessoriesAOMap.textureset/Universal.mipmapset/AccessoriesAOMap.png`
- `Renderer/Assets.xcassets/gray.textureset/Universal.mipmapset/gray.png`
- `Renderer/Assets.xcassets/BodyBaseColor.textureset/Universal.mipmapset/BodyBaseColor.png`
- `Renderer/Assets.xcassets/BodyMetallicMap.textureset/Universal.mipmapset/BodyMetallicMap.png`
- `Renderer/Assets.xcassets/AccessoriesBaseColor.textureset/Universal.mipmapset/AccessoriesBaseColor.png`
- `Renderer/Assets.xcassets/BodyAOMap.textureset/Universal.mipmapset/BodyAOMap.png`
- `Renderer/Assets.xcassets/AccessoriesNormalMap.textureset/Universal.mipmapset/AccessoriesNormalMap.png`
- `Renderer/Assets.xcassets/BodyRoughnessMap.textureset/Universal.mipmapset/BodyRoughnessMap.png`
- `Renderer/Assets.xcassets/IrradianceMap.cubetextureset/Universal +X.mipmapset/tileLevel30.png`
- `Renderer/Assets.xcassets/IrradianceMap.cubetextureset/Universal +X.mipmapset/tileLevel20.png`
- `Renderer/Assets.xcassets/IrradianceMap.cubetextureset/Universal +X.mipmapset/tileLevel50.png`
- `Renderer/Assets.xcassets/IrradianceMap.cubetextureset/Universal +X.mipmapset/tileLevel40.png`
- `Renderer/Assets.xcassets/IrradianceMap.cubetextureset/Universal +X.mipmapset/tileLevel80.png`
- `Renderer/Assets.xcassets/IrradianceMap.cubetextureset/Universal +X.mipmapset/tileLevel70.png`
- `Renderer/Assets.xcassets/IrradianceMap.cubetextureset/Universal +X.mipmapset/tileLevel60.png`
- `Renderer/Assets.xcassets/IrradianceMap.cubetextureset/Universal +X.mipmapset/tileLevel10.png`
- `Renderer/Assets.xcassets/IrradianceMap.cubetextureset/Universal +X.mipmapset/tile0.png`
- `Renderer/Assets.xcassets/IrradianceMap.cubetextureset/Universal -Z.mipmapset/tileLevel25.png`
- `Renderer/Assets.xcassets/IrradianceMap.cubetextureset/Universal -Z.mipmapset/tileLevel35.png`
- `Renderer/Assets.xcassets/IrradianceMap.cubetextureset/Universal -Z.mipmapset/tileLevel85.png`
- `Renderer/Assets.xcassets/IrradianceMap.cubetextureset/Universal -Z.mipmapset/tileLevel45.png`
- `Renderer/Assets.xcassets/IrradianceMap.cubetextureset/Universal -Z.mipmapset/tileLevel55.png`
- `Renderer/Assets.xcassets/IrradianceMap.cubetextureset/Universal -Z.mipmapset/tileLevel65.png`
- `Renderer/Assets.xcassets/IrradianceMap.cubetextureset/Universal -Z.mipmapset/tileLevel75.png`
- `Renderer/Assets.xcassets/IrradianceMap.cubetextureset/Universal -Z.mipmapset/tileLevel15.png`
- `Renderer/Assets.xcassets/IrradianceMap.cubetextureset/Universal -Z.mipmapset/tile5.png`
- `Renderer/Assets.xcassets/IrradianceMap.cubetextureset/Universal +Y.mipmapset/tileLevel32.png`
- `Renderer/Assets.xcassets/IrradianceMap.cubetextureset/Universal +Y.mipmapset/tileLevel22.png`
- `Renderer/Assets.xcassets/IrradianceMap.cubetextureset/Universal +Y.mipmapset/tileLevel52.png`
- `Renderer/Assets.xcassets/IrradianceMap.cubetextureset/Universal +Y.mipmapset/tileLevel82.png`
- `Renderer/Assets.xcassets/IrradianceMap.cubetextureset/Universal +Y.mipmapset/tileLevel42.png`
- `Renderer/Assets.xcassets/IrradianceMap.cubetextureset/Universal +Y.mipmapset/tileLevel72.png`
- `Renderer/Assets.xcassets/IrradianceMap.cubetextureset/Universal +Y.mipmapset/tileLevel62.png`
- `Renderer/Assets.xcassets/IrradianceMap.cubetextureset/Universal +Y.mipmapset/tile2.png`
- `Renderer/Assets.xcassets/IrradianceMap.cubetextureset/Universal +Y.mipmapset/tileLevel12.png`
- `Renderer/Assets.xcassets/IrradianceMap.cubetextureset/Universal -Y.mipmapset/tileLevel33.png`
- `Renderer/Assets.xcassets/IrradianceMap.cubetextureset/Universal -Y.mipmapset/tileLevel23.png`
- `Renderer/Assets.xcassets/IrradianceMap.cubetextureset/Universal -Y.mipmapset/tileLevel53.png`
- `Renderer/Assets.xcassets/IrradianceMap.cubetextureset/Universal -Y.mipmapset/tileLevel83.png`
- `Renderer/Assets.xcassets/IrradianceMap.cubetextureset/Universal -Y.mipmapset/tileLevel43.png`
- `Renderer/Assets.xcassets/IrradianceMap.cubetextureset/Universal -Y.mipmapset/tileLevel73.png`
- `Renderer/Assets.xcassets/IrradianceMap.cubetextureset/Universal -Y.mipmapset/tileLevel63.png`
- `Renderer/Assets.xcassets/IrradianceMap.cubetextureset/Universal -Y.mipmapset/tileLevel13.png`
- `Renderer/Assets.xcassets/IrradianceMap.cubetextureset/Universal -Y.mipmapset/tile3.png`
- `Renderer/Assets.xcassets/IrradianceMap.cubetextureset/Universal +Z.mipmapset/tileLevel24.png`
- `Renderer/Assets.xcassets/IrradianceMap.cubetextureset/Universal +Z.mipmapset/tileLevel34.png`
- `Renderer/Assets.xcassets/IrradianceMap.cubetextureset/Universal +Z.mipmapset/tileLevel84.png`
- `Renderer/Assets.xcassets/IrradianceMap.cubetextureset/Universal +Z.mipmapset/tileLevel44.png`
- `Renderer/Assets.xcassets/IrradianceMap.cubetextureset/Universal +Z.mipmapset/tileLevel54.png`
- `Renderer/Assets.xcassets/IrradianceMap.cubetextureset/Universal +Z.mipmapset/tileLevel64.png`
- `Renderer/Assets.xcassets/IrradianceMap.cubetextureset/Universal +Z.mipmapset/tileLevel74.png`
- `Renderer/Assets.xcassets/IrradianceMap.cubetextureset/Universal +Z.mipmapset/tile4.png`
- `Renderer/Assets.xcassets/IrradianceMap.cubetextureset/Universal +Z.mipmapset/tileLevel14.png`
- `Renderer/Assets.xcassets/IrradianceMap.cubetextureset/Universal -X.mipmapset/tileLevel31.png`
- `Renderer/Assets.xcassets/IrradianceMap.cubetextureset/Universal -X.mipmapset/tileLevel21.png`
- `Renderer/Assets.xcassets/IrradianceMap.cubetextureset/Universal -X.mipmapset/tileLevel51.png`
- `Renderer/Assets.xcassets/IrradianceMap.cubetextureset/Universal -X.mipmapset/tileLevel41.png`
- `Renderer/Assets.xcassets/IrradianceMap.cubetextureset/Universal -X.mipmapset/tileLevel81.png`
- `Renderer/Assets.xcassets/IrradianceMap.cubetextureset/Universal -X.mipmapset/tileLevel71.png`
- `Renderer/Assets.xcassets/IrradianceMap.cubetextureset/Universal -X.mipmapset/tileLevel61.png`
- `Renderer/Assets.xcassets/IrradianceMap.cubetextureset/Universal -X.mipmapset/tile1.png`
- `Renderer/Assets.xcassets/IrradianceMap.cubetextureset/Universal -X.mipmapset/tileLevel11.png`
- `Renderer/Assets.xcassets/white.textureset/Universal.mipmapset/white.png`
- `Renderer/Assets.xcassets/black.textureset/Universal.mipmapset/black.png`
- `Renderer/Assets.xcassets/BodyNormalMap.textureset/Universal.mipmapset/BodyNormalMap.png`
- `Renderer/Assets.xcassets/AccessoriesMetallicMap.textureset/Universal.mipmapset/AccessoriesMetallicMap.png`
- `Renderer/Assets.xcassets/checkerboard.textureset/Universal.mipmapset/checkerboard.png`
- `Renderer/Assets.xcassets/AccessoriesRoughnessMap.textureset/Universal.mipmapset/AccessoriesRoughnessMap.png`
- `Renderer/Models/firetruck.mtl`
- `Renderer/Models/firetruck.obj`

## Overview

This sample code project relates to multiple WWDC sessions, including:

- [10089: Bring your advanced games to Apple platforms](https://developer.apple.com/wwdc24/10089/)
- [10101: Go bindless with Metal 3](https://developer.apple.com/wwdc22/10101/)
- [10286: Explore bindless rendering in Metal](https://developer.apple.com/wwdc21/10286/)
- [10150: Explore hybrid rendering with Metal ray tracing](https://developer.apple.com/wwdc21/10150/)

### Configure the sample code project

To run this sample app, you need the following:

- A Mac with macOS 13 or later, and Xcode 15.3 or later
- An iOS device with iOS 16 or later

> **Note:** This sample doesn’t support running in Simulator.
