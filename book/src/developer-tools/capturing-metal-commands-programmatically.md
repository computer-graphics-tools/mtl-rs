# Capturing Metal commands programmatically

> [Apple Documentation](https://developer.apple.com/documentation/metal/capturing-metal-commands-programmatically?language=objc)

Invoke a Metal frame capture from your app, then save the resulting GPU trace to a file or view it in Xcode.

## Run the Example

```bash
cargo run --example capturing_metal_commands_programmatically
```

## Shader Files

- `Renderer/AAPLShaders.metal`

## Resources

- `Application/Media.xcassets/ColorMap.imageset/ColorMap.png`

## Overview

> **Note:** This sample code project is associated with WWDC 2019 session [606: Delivering Optimized Metal Apps and Games](https://developer.apple.com/videos/play/wwdc2019/606/).

### Configure the sample code project

To run the app:

- Build the project with Xcode 11 or later.
