# Migrating OpenGL code to Metal

> [Apple Documentation](https://developer.apple.com/documentation/metal/migrating-opengl-code-to-metal?language=objc)

Replace your app’s deprecated OpenGL code with Metal.

## Run the Example

```bash
cargo run --example migrating_opengl_code_to_metal
```

## Shader Files

- `Metal/AAPLShaders.metal`

## Resources

- `Common/Meshes/FoliageBaseColor.png`
- `Common/Meshes/Temple.obj`
- `Common/Meshes/StructureBaseColor.png`
- `Common/Meshes/Temple.mtl`

## Overview

> **Note:** This sample code project is associated with WWDC 2019 session [611: Bringing OpenGL Apps to Metal](https://developer.apple.com/videos/play/wwdc2019/611/).

### Configure the sample code project

To run the app:

- Build the project with Xcode 11 or later.
- Target an iOS device or simulator with iOS 11 or later.
