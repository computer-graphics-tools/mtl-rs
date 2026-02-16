# Summary
[Introduction](introduction.md)

# Essentials
- [Essentials](essentials/README.md)
  - [Understanding the Metal 4 core API](essentials/understanding-the-metal-4-core-api.md)
  - [Drawing a triangle with Metal 4](essentials/drawing-a-triangle-with-metal-4.md)
  - [Performing calculations on a GPU](essentials/performing-calculations-on-a-gpu.md)
  - [Using Metal to draw a view’s contents](essentials/using-metal-to-draw-a-view's-contents.md)

# Samples
- [Samples](samples/README.md)
  - [Metal sample code library](samples/metal-sample-code-library.md)

# GPU devices
- [GPU devices](gpu-devices/README.md)
  - [GPU devices and work submission](gpu-devices/gpu-devices-and-work-submission.md)

# Command encoders
- [Command encoders](command-encoders/README.md)
  - [Render passes](command-encoders/render-passes.md)
  - [Compute passes](command-encoders/compute-passes.md)
  - [Machine-learning passes](command-encoders/machine-learning-passes.md)
  - [Blit passes](command-encoders/blit-passes.md)
  - [Indirect command encoding](command-encoders/indirect-command-encoding.md)
  - [Ray tracing with acceleration structures](command-encoders/ray-tracing-with-acceleration-structures.md)

# Resources
- [Resources](resources/README.md)
  - [Resource fundamentals](resources/resource-fundamentals.md)
  - [Buffers](resources/buffers.md)

# Textures
- [Textures](textures/README.md)
  - [Memory heaps](textures/memory-heaps.md)
  - [Resource loading](textures/resource-loading.md)
  - [Resource synchronization](textures/resource-synchronization.md)
  - [Combining blit and compute operations in a single pass](textures/combining-blit-and-compute-operations-in-a-single-pass.md)
  - [Reading pixel data from a drawable texture](textures/reading-pixel-data-from-a-drawable-texture.md)
  - [Creating and sampling textures](textures/creating-and-sampling-textures.md)
  - [Streaming large images with Metal sparse textures](textures/streaming-large-images-with-metal-sparse-textures.md)

# Shader compilation and libraries
- [Shader compilation and libraries](shader-compilation-and-libraries/README.md)
  - [Using the Metal 4 compilation API](shader-compilation-and-libraries/using-the-metal-4-compilation-api.md)
  - [Shader libraries](shader-compilation-and-libraries/shader-libraries.md)
  - [Using function specialization to build pipeline variants](shader-compilation-and-libraries/using-function-specialization-to-build-pipeline-variants.md)

# Presentation
- [Presentation](presentation/README.md)
  - [Managing your game window for Metal in macOS](presentation/managing-your-game-window-for-metal-in-macos.md)
  - [Managing your Metal app window in iPadOS](presentation/managing-your-metal-app-window-in-ipados.md)
  - [Adapting your game interface for smaller screens](presentation/adapting-your-game-interface-for-smaller-screens.md)
  - [Onscreen presentation](presentation/onscreen-presentation.md)
  - [HDR content](presentation/hdr-content.md)

# Developer tools
- [Developer tools](developer-tools/README.md)
  - [Supporting Simulator in a Metal app](developer-tools/supporting-simulator-in-a-metal-app.md)
  - [Capturing Metal commands programmatically](developer-tools/capturing-metal-commands-programmatically.md)
  - [Logging shader debug messages](developer-tools/logging-shader-debug-messages.md)
  - [Developing Metal apps that run in Simulator](developer-tools/developing-metal-apps-that-run-in-simulator.md)
  - [Improving your game’s graphics performance and settings](developer-tools/improving-your-games-graphics-performance-and-settings.md)
  - [GPU counters and counter sample buffers](developer-tools/gpu-counters-and-counter-sample-buffers.md)
  - [Metal debugging types](developer-tools/metal-debugging-types.md)

# Apple silicon
- [Apple silicon](apple-silicon/README.md)
  - [Tailor your apps for Apple GPUs and tile-based deferred rendering](apple-silicon/tailor-your-apps-for-apple-gpus-and-tile-based-deferred-rendering.md)
  - [Metal structures](apple-silicon/metal-structures.md)
  - [Metal enumerations](apple-silicon/metal-enumerations.md)
  - [Metal constants](apple-silicon/metal-constants.md)
  - [Metal functions](apple-silicon/metal-functions.md)
  - [Metal data types](apple-silicon/metal-data-types.md)
  - [Metal variables](apple-silicon/metal-variables.md)
  - [Metal macros](apple-silicon/metal-macros.md)

# Compute workflows
- [Compute workflows](compute-workflows/README.md)
  - [Performing calculations on a GPU](compute-workflows/performing-calculations-on-a-gpu.md)
  - [Selecting device objects for compute processing](compute-workflows/selecting-device-objects-for-compute-processing.md)
  - [Customizing a TensorFlow operation](compute-workflows/customizing-a-tensorflow-operation.md)
  - [Customizing a PyTorch operation](compute-workflows/customizing-a-pytorch-operation.md)
  - [Running a machine learning model on the GPU timeline](compute-workflows/running-a-machine-learning-model-on-the-gpu-timeline.md)

# Render workflows
- [Render workflows](render-workflows/README.md)
  - [Using Metal to draw a view’s contents](render-workflows/using-metal-to-draw-a-view's-contents.md)
  - [Drawing a triangle with Metal 4](render-workflows/drawing-a-triangle-with-metal-4.md)
  - [Selecting device objects for graphics rendering](render-workflows/selecting-device-objects-for-graphics-rendering.md)
  - [Customizing render pass setup](render-workflows/customizing-render-pass-setup.md)
  - [Creating a custom Metal view](render-workflows/creating-a-custom-metal-view.md)
  - [Calculating primitive visibility using depth testing](render-workflows/calculating-primitive-visibility-using-depth-testing.md)
  - [Encoding indirect command buffers on the CPU](render-workflows/encoding-indirect-command-buffers-on-the-cpu.md)
  - [Implementing order-independent transparency with image blocks](render-workflows/implementing-order-independent-transparency-with-image-blocks.md)
  - [Loading textures and models using Metal fast resource loading](render-workflows/loading-textures-and-models-using-metal-fast-resource-loading.md)
  - [Adjusting the level of detail using Metal mesh shaders](render-workflows/adjusting-the-level-of-detail-using-metal-mesh-shaders.md)
  - [Creating a 3D application with hydra rendering](render-workflows/creating-a-3d-application-with-hydra-rendering.md)
  - [Culling occluded geometry using the visibility result buffer](render-workflows/culling-occluded-geometry-using-the-visibility-result-buffer.md)
  - [Improving edge-rendering quality with multisample antialiasing (MSAA)](render-workflows/improving-edge-rendering-quality-with-multisample-antialiasing-msaa.md)
  - [Achieving smooth frame rates with a Metal display link](render-workflows/achieving-smooth-frame-rates-with-a-metal-display-link.md)

# Argument buffers
- [Argument buffers](argument-buffers/README.md)
  - [Managing groups of resources with argument buffers](argument-buffers/managing-groups-of-resources-with-argument-buffers.md)
  - [Using argument buffers with resource heaps](argument-buffers/using-argument-buffers-with-resource-heaps.md)
  - [Encoding argument buffers on the GPU](argument-buffers/encoding-argument-buffers-on-the-gpu.md)
  - [Rendering terrain dynamically with argument buffers](argument-buffers/rendering-terrain-dynamically-with-argument-buffers.md)

# Shaders
- [Shaders](shaders/README.md)
  - [Creating a Metal dynamic library](shaders/creating-a-metal-dynamic-library.md)
  - [Using function specialization to build pipeline variants](shaders/using-function-specialization-to-build-pipeline-variants.md)

# Synchronization
- [Synchronization](synchronization/README.md)
  - [Synchronizing CPU and GPU work](synchronization/synchronizing-cpu-and-gpu-work.md)
  - [Implementing a multistage image filter using heaps and events](synchronization/implementing-a-multistage-image-filter-using-heaps-and-events.md)
  - [Implementing a multistage image filter using heaps and fences](synchronization/implementing-a-multistage-image-filter-using-heaps-and-fences.md)

# Lighting techniques
- [Lighting techniques](lighting-techniques/README.md)
  - [Rendering a scene with forward plus lighting using tile shaders](lighting-techniques/rendering-a-scene-with-forward-plus-lighting-using-tile-shaders.md)
  - [Rendering a scene with deferred lighting in Objective-C](lighting-techniques/rendering-a-scene-with-deferred-lighting-in-objective-c.md)
  - [Rendering a scene with deferred lighting in Swift](lighting-techniques/rendering-a-scene-with-deferred-lighting-in-swift.md)
  - [Rendering a scene with deferred lighting in C++](lighting-techniques/rendering-a-scene-with-deferred-lighting-in-c++.md)
  - [Rendering reflections with fewer render passes](lighting-techniques/rendering-reflections-with-fewer-render-passes.md)

# Multiple techniques
- [Multiple techniques](multiple-techniques/README.md)
  - [Modern rendering with Metal](multiple-techniques/modern-rendering-with-metal.md)
  - [Encoding indirect command buffers on the GPU](multiple-techniques/encoding-indirect-command-buffers-on-the-gpu.md)

# Ray tracing
- [Ray tracing](ray-tracing/README.md)
  - [Rendering reflections in real time using ray tracing](ray-tracing/rendering-reflections-in-real-time-using-ray-tracing.md)
  - [Accelerating ray tracing using Metal](ray-tracing/accelerating-ray-tracing-using-metal.md)
  - [Control the ray tracing process using intersection queries](ray-tracing/control-the-ray-tracing-process-using-intersection-queries.md)
  - [Accelerating ray tracing and motion blur using Metal](ray-tracing/accelerating-ray-tracing-and-motion-blur-using-metal.md)
  - [Rendering a curve primitive in a ray tracing scene](ray-tracing/rendering-a-curve-primitive-in-a-ray-tracing-scene.md)

# HDR
- [HDR](hdr/README.md)
  - [Processing HDR images with Metal](hdr/processing-hdr-images-with-metal.md)

# OpenGL
- [OpenGL](opengl/README.md)
  - [Migrating OpenGL code to Metal](opengl/migrating-opengl-code-to-metal.md)
  - [Mixing Metal and OpenGL rendering in a view](opengl/mixing-metal-and-opengl-rendering-in-a-view.md)
