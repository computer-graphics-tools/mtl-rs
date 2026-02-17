# Modern rendering with Metal

> [Apple Documentation](https://developer.apple.com/documentation/metal/modern-rendering-with-metal?language=objc)

Use advanced Metal features such as indirect command buffers, sparse textures, and variable rate rasterization to implement complex rendering techniques.

## Run the Example

```bash
cargo run --example modern_rendering_with_metal
```

## Shader Files

- `Renderer/Shaders/AAPLDebug.metal`
- `Renderer/Shaders/AAPLAmbientObscurance.metal`
- `Renderer/Shaders/AAPLLightCulling.metal`
- `Renderer/Shaders/AAPLSky.metal`
- `Renderer/Shaders/AAPLLighting.metal`
- `Renderer/Shaders/AAPLScatterVolume.metal`
- `Renderer/Shaders/AAPLSimple.metal`
- `Renderer/Shaders/AAPLDepthPyramid.metal`
- `Renderer/Shaders/AAPLCulling.metal`
- `Renderer/Shaders/AAPLMeshRenderer.metal`
- `Renderer/Shaders/AAPLResolve.metal`

## Resources

- `Assets/blueNoise.png`
- `Assets/Perlin.ktx`
- `Assets/san_giuseppe_bridge_4k_ibl.ktx`
- `Assets/DFGLUT.ktx`

## Overview

> **Note:** This sample code project is associated with the [Metal Enhancements for A13 Bionic](http://developer.apple.com/tech-talks/608) tech talk.

### Configure the sample code project

Because this sample app uses indirect command buffers, you can’t run this sample in the Simulator — you need to run it on a device. To run this sample, use one of the following:

- A Mac from mid-2016 and later with macOS 11 and later
- An iPad with A11 Bionic and later using iPadOS 14.1 and later
- An iOS device with A11 Bionic and later using iOS 14.1 and later
- Xcode 12 and later
