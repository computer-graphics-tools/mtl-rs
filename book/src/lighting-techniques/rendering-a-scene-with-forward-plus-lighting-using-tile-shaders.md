# Rendering a scene with forward plus lighting using tile shaders

> [Apple Documentation](https://developer.apple.com/documentation/metal/rendering-a-scene-with-forward-plus-lighting-using-tile-shaders?language=objc)

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

Note

This sample code project is associated with WWDC 2019 session 601: Modern Rendering with Metal.

Configure the sample code project

To run the app:

Build the project with Xcode 11 or later.

Target an iOS device with an A11 chip or later and iOS 11 or later.

See Also
Rendering a scene with deferred lighting in Objective-C
Avoid expensive lighting calculations by implementing a deferred lighting renderer optimized for immediate mode and tile-based deferred renderer GPUs.
Rendering a scene with deferred lighting in Swift
Avoid expensive lighting calculations by implementing a deferred lighting renderer optimized for immediate mode and tile-based deferred renderer GPUs.
Rendering a scene with deferred lighting in C++
Avoid expensive lighting calculations by implementing a deferred lighting renderer optimized for immediate mode and tile-based deferred renderer GPUs.
Rendering reflections with fewer render passes
Use layer selection to reduce the number of render passes needed to generate an environment map.
Apple
TestFlight
Xcode
Xcode Cloud
SF Symbols
Accessibility
Accessories
App Extension
App Store
Audio & Video
Augmented Reality
Distribution
Education
Fonts
Games
Health & Fitness
In-App Purchase
Localization
Maps & Location
Machine Learning & AI
Open Source
Security
Safari & Web
Resources
Tutorials
Downloads
Forums
Videos
Contact Us
Bug Reporting
System Status
App Store Connect
Certificates, IDs, & Profiles
Feedback Assistant
Programs
App Store Small Business Program
MFi Program
Video Partner Program
Security Bounty Program
Security Research Device Program
Events
Meet with Apple
App Store Awards
WWDC
