# Using function specialization to build pipeline variants

> [Apple Documentation](https://developer.apple.com/documentation/metal/using-function-specialization-to-build-pipeline-variants?language=objc)

Create pipelines for different levels of detail from a common shader source.

## Run the Example

```bash
cargo run --example using_function_specialization_to_build_pipeline_variants
```

## Shader Files

- `Renderer/AAPLShaders.metal`

## Resources

- `Documentation/lod-with-function-constants-1-LODWithFunctionSpecialization.png`
- `Documentation/lod-with-function-constants-2-LevelsOfDetail.png`
- `Renderer/Assets.xcassets/AccessoriesAOMap.textureset/Universal.mipmapset/AccessoriesAOMap.png`
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
- `Renderer/Assets.xcassets/BodyNormalMap.textureset/Universal.mipmapset/BodyNormalMap.png`
- `Renderer/Assets.xcassets/AccessoriesMetallicMap.textureset/Universal.mipmapset/AccessoriesMetallicMap.png`
- `Renderer/Assets.xcassets/AccessoriesRoughnessMap.textureset/Universal.mipmapset/AccessoriesRoughnessMap.png`
- `Renderer/Models/firetruck.mtl`
- `Renderer/Models/firetruck.obj`

## Overview

A high-quality gaming experience has to manage trade-offs between great graphics and great performance. High-quality models look great, but their complexity requires a significant amount of processing power. By increasing or decreasing the level of detail (LOD) of a model, games can selectively manage both graphics and performance.

![LOD with Function Specialization](https://docs-assets.developer.apple.com/published/bb6ef3f8d62dddb3c52f3f6e0241987b/lod-with-function-constants-1-LODWithFunctionSpecialization%402x.png)

Instead of selecting a fixed LOD at build time, games can dynamically select between a range of LODs at runtime based on certain model-view conditions. For example, a focal-point foreground model could have a high LOD, whereas a fast-moving background model could have a low LOD.

This sample demonstrates dynamic LOD selection for a fire truck model, based on its distance from the scene’s camera. When the model is closer to the camera, the renderer uses a higher LOD; when the model is further from the camera, the renderer uses a lower LOD.

![Levels of Detail](https://docs-assets.developer.apple.com/published/3cb45549fabb172c758319ac9d4dd546/lod-with-function-constants-2-LevelsOfDetail%402x.png)

### Getting started

The Xcode project contains schemes for running the sample on macOS, iOS, or tvOS.

### GPU branch statements

Unlike CPU code, graphics processing unit (GPU) branch statements like `if` and `else` are very expensive. The massively parallel architecture of GPUs isn’t particularly well suited to handling GPU functions that have many branches. More branches result in more register allocations and thus decrease the number of GPU threads that can execute concurrently. Nevertheless, branch statements are useful programming constructs, particularly for functions that share a lot of code. In fact, a common problem for graphics functions that share code is how to handle a branch condition that differs only between draw calls, not between individual threads executing within a single draw call.

Traditionally, branches that differ between draw calls are mitigated in one of these ways:

- Writing per-branch functions. Each branch is written as a complete and separate function, and the render loop determines which function to use at runtime. This approach greatly increases code duplication because all possible outcomes of each branch condition require their own standalone function. For example, a single `if` statement requires one function for the `true` outcome and another function for the `false` outcome.
- Using preprocessor directives. Instead of using a regular `if` statement, functions can use the `#if` preprocessor directive that selectively compiles a function after evaluating its branch conditions. This approach avoids code duplication but reduces the performance benefits of precompiled Metal shading language code. Because the branch conditions can only be evaluated at runtime, the functions can’t be precompiled at build time.

The Metal *function specialization* feature reduces branch performance costs, avoids code duplication, and leverages build time compilation. Function specialization allows you to create multiple executable versions of a single source function. You create specialized functions by declaring *function constants* in your Metal shading language code and setting their values at runtime. Doing so allows the front-end compiler to precompile your source function at build time and the back-end compiler to compile the specialized function at runtime, when the pipeline is created.

### Define your LOD selection criteria

This sample demonstrates function specialization by creating different render pipelines for different LODs. All of the pipelines share the same source function, but function constants determine LOD-specific paths and inputs for each pipeline. Specifically, the sample demonstrates dynamic LOD selection for a fire truck model, based on its distance from the scene’s camera. When the fire truck is close to the camera, it occupies more pixels on the screen; therefore, the sample uses a high-quality render pipeline. When the fire truck is far from the camera, it occupies fewer pixels on the screen; therefore, the sample uses a low-quality render pipeline.

The fire truck model in this sample uses many types of textures, such as *albedo*, *normal*, *metallic*, *roughness*, *ambient occlusion*, and *irradiance*. It’s too wasteful to sample from each of these textures when the model is far from the camera because the detail provided by the full combination of textures isn’t seen. The sample uses various function constant values to create specialized functions that sample from more or fewer textures, depending on the selected LOD. Additionally, specialized functions that sample from fewer textures also perform less complex computations and result in a faster render pipeline.

The `isTexturedProperty:atQualityLevel:` method controls whether a material property is set by sampling from a texture or by reading a constant value.

**AAPLMesh.m**

```objective-c
+ (BOOL)isTexturedProperty:(AAPLFunctionConstant)propertyIndex atQualityLevel:(AAPLQualityLevel)quality
{
    AAPLQualityLevel minLevelForProperty = AAPLQualityLevelHigh;

    switch(propertyIndex)
    {
        case AAPLFunctionConstantBaseColorMapIndex:
        case AAPLFunctionConstantIrradianceMapIndex:
            minLevelForProperty = AAPLQualityLevelMedium;
            break;
        default:
            break;
    }

    return quality <= minLevelForProperty;
}
```

### Implement specialized functions

The sample uses six function constants to control the various inputs available to the `fragmentLighting` fragment function.

**AAPLShaders.metal**

```metal
constant bool has_base_color_map        [[ function_constant(AAPLFunctionConstantBaseColorMapIndex) ]];
constant bool has_normal_map            [[ function_constant(AAPLFunctionConstantNormalMapIndex) ]];
constant bool has_metallic_map          [[ function_constant(AAPLFunctionConstantMetallicMapIndex) ]];
constant bool has_roughness_map         [[ function_constant(AAPLFunctionConstantRoughnessMapIndex) ]];
constant bool has_ambient_occlusion_map [[ function_constant(AAPLFunctionConstantAmbientOcclusionMapIndex) ]];
constant bool has_irradiance_map        [[ function_constant(AAPLFunctionConstantIrradianceMapIndex) ]];
```

The sample also declares a derived function constant, `has_any_map`, that’s used in the `vertexTransform` vertex function. This value determines whether the render pipeline requires the vertex function to output a texture coordinate to the `ColorInOut.texCoord` return value.

**AAPLShaders.metal**

```metal
constant bool has_any_map = (has_base_color_map        ||
                             has_normal_map            ||
                             has_metallic_map          ||
                             has_roughness_map         ||
                             has_ambient_occlusion_map ||
                             has_irradiance_map);
```

When the value of `has_any_map` is `false`, the vertex function does not write a value to the `texCoord` member.

**AAPLShaders.metal**

```metal
if (has_any_map)
{
    out.texCoord = in.texCoord;
}
```

The function constants control the source of a parameter to the lighting computation in the `calculateParameters()` function. When you use the `[[function_constant(index)]]` attribute, this function can determine whether it should sample from a texture. The function only samples from a texture if the attribute indicates that a texture parameter is present; otherwise, it reads a value from the `materialData` buffer.

**AAPLShaders.metal**

```metal
LightingParameters calculateParameters(                 ColorInOut in,
                                       constant AAPLFrameData    & frameData,
                                       constant AAPLMaterialData & materialData,
                                       texture2d<float>   baseColorMap        [[ function_constant(has_base_color_map) ]],
                                       texture2d<float>   normalMap           [[ function_constant(has_normal_map) ]],
                                       texture2d<float>   metallicMap         [[ function_constant(has_metallic_map) ]],
                                       texture2d<float>   roughnessMap        [[ function_constant(has_roughness_map) ]],
                                       texture2d<float>   ambientOcclusionMap [[ function_constant(has_ambient_occlusion_map) ]],
                                       texturecube<float> irradianceMap       [[ function_constant(has_irradiance_map) ]])
```

The corresponding inputs to the fragment function also use the same function constants.

**AAPLShaders.metal**

```metal
fragment float4
fragmentLighting(                 ColorInOut in           [[ stage_in ]],
                 constant AAPLFrameData    & frameData    [[ buffer(AAPLBufferIndexFrameData) ]],
                 constant AAPLMaterialData & materialData [[ buffer(AAPLBufferIndexMaterialData) ]],
                 texture2d<float>   baseColorMap          [[ texture(AAPLTextureIndexBaseColor),        function_constant(has_base_color_map) ]],
                 texture2d<float>   normalMap             [[ texture(AAPLTextureIndexNormal),           function_constant(has_normal_map) ]],
                 texture2d<float>   metallicMap           [[ texture(AAPLTextureIndexMetallic),         function_constant(has_metallic_map) ]],
                 texture2d<float>   roughnessMap          [[ texture(AAPLTextureIndexRoughness),        function_constant(has_roughness_map) ]],
                 texture2d<float>   ambientOcclusionMap   [[ texture(AAPLTextureIndexAmbientOcclusion), function_constant(has_ambient_occlusion_map) ]],
                 texturecube<float> irradianceMap         [[ texture(AAPLTextureIndexIrradianceMap),    function_constant(has_irradiance_map)]])
```

### Create different pipelines

This sample uses three different `MTLRenderPipelineState` objects, each representing a different LOD. Specializing functions and building pipelines is expensive, so the sample performs these tasks asynchronously before starting the render loop. When the `AAPLRenderer` object is initialized, each LOD pipeline is created asynchronously by using dispatch groups, completion handlers, and notification blocks.

The sample creates six specialized functions overall: one vertex and one fragment function for each of the three LODs. This task is monitored by the `specializationGroup` dispatch group, and each function is specialized by calling the `newFunctionWithName:constantValues:completionHandler:` method.

**AAPLRenderer.m**

```objective-c
for (uint qualityLevel = 0; qualityLevel < AAPLNumQualityLevels; qualityLevel++)
{
    dispatch_group_enter(specializationGroup);

    MTLFunctionConstantValues* constantValues = [self functionConstantsForQualityLevel:qualityLevel];

    [defaultLibrary newFunctionWithName:@"fragmentLighting" constantValues:constantValues
                      completionHandler:^(id<MTLFunction> newFunction, NSError *error )
     {
         NSAssert(newFunction, @"Failed to specialize function: %@", error);

         self->_fragmentFunctions[qualityLevel] = newFunction;
         dispatch_group_leave(specializationGroup);
     }];

    dispatch_group_enter(specializationGroup);

    [defaultLibrary newFunctionWithName:@"vertexTransform" constantValues:constantValues
                      completionHandler:^(id<MTLFunction> newFunction, NSError *error )
     {
         NSAssert(newFunction, @"Failed to specialize function: %@", error);

         self->_vertexFunctions[qualityLevel] = newFunction;
         dispatch_group_leave(specializationGroup);
     }];
}
```

The `notifyBlock` block builds the three render pipelines. This task is monitored by the `_pipelineCreationGroup` dispatch group, and each pipeline is built by calling the `newRenderPipelineStateWithDescriptor:completionHandler:` method.

**AAPLRenderer.m**

```objective-c
dispatch_group_enter(_pipelineCreationGroup);

void (^notifyBlock)(void) = ^void()
{
    const id<MTLDevice> device  = self->_device;
    const dispatch_group_t pipelineCreationGroup = self->_pipelineCreationGroup;

    MTLRenderPipelineDescriptor *pipelineStateDescriptors[AAPLNumQualityLevels];

    dispatch_group_wait(specializationGroup, DISPATCH_TIME_FOREVER);

    for (uint qualityLevel = 0; qualityLevel < AAPLNumQualityLevels; qualityLevel++)
    {
        dispatch_group_enter(pipelineCreationGroup);

        pipelineStateDescriptors[qualityLevel] = [pipelineStateDescriptor copy];
        pipelineStateDescriptors[qualityLevel].fragmentFunction = self->_fragmentFunctions[qualityLevel];
        pipelineStateDescriptors[qualityLevel].vertexFunction = self->_vertexFunctions[qualityLevel];

        [device newRenderPipelineStateWithDescriptor:pipelineStateDescriptors[qualityLevel]
                                   completionHandler:^(id<MTLRenderPipelineState> newPipelineState, NSError *error )
         {
             NSAssert(newPipelineState, @"Failed to create pipeline state, error %@", error);

             self->_pipelineStates[qualityLevel] = newPipelineState;
             dispatch_group_leave(pipelineCreationGroup);
         }];
    }

    dispatch_group_leave(pipelineCreationGroup);
};

dispatch_group_notify(specializationGroup, pipelineQueue, notifyBlock);
```

### Render with a specific LOD

At the beginning of the render loop, for each frame, the sample calls the `_calculateQualityAtDistance:` method to update the `_currentQualityLevel` value. This value defines the LOD for the frame based on the distance between the model and the camera. The `_calculateQualityAtDistance:` method also sets a `_globalMapWeight` value that creates a smooth transition between LOD boundaries.

**AAPLRenderer.m**

```objective-c
- (void)calculateQualityAtDistance:(float)distance
{
    static const float MediumQualityDepth     = 150.f;
    static const float LowQualityDepth        = 650.f;
    static const float TransitionDepthAmount  = 50.f;

    assert(distance >= 0.0f);
    if (distance < MediumQualityDepth)
    {
        static const float TransitionDepth = MediumQualityDepth - TransitionDepthAmount;
        if(distance > TransitionDepth)
        {
            _globalMapWeight = distance - TransitionDepth;
            _globalMapWeight /= TransitionDepthAmount;
            _globalMapWeight = 1.0 - _globalMapWeight;
        }
        else
        {
            _globalMapWeight = 1.0;
        }
        _currentQualityLevel = AAPLQualityLevelHigh;
    }
    else if (distance < LowQualityDepth)
    {
        static const float TransitionDepth = LowQualityDepth - TransitionDepthAmount;
        if(distance > TransitionDepth)
        {
            _globalMapWeight = distance - (TransitionDepth);
            _globalMapWeight /= TransitionDepthAmount;
            _globalMapWeight = 1.0 - _globalMapWeight;
        }
        else
        {
            _globalMapWeight = 1.0;
        }
        _currentQualityLevel = AAPLQualityLevelMedium;
    }
    else
    {
        _currentQualityLevel = AAPLQualityLevelLow;
        _globalMapWeight = 0.0;
    }
}
```

The updated `_currentQualityLevel` value is used to set the corresponding `MTLRenderPipelineState` object for the frame.

**AAPLRenderer.m**

```objective-c
[renderEncoder setRenderPipelineState:_pipelineStates[_currentQualityLevel]];
```

The updated `_globalMapWeight` value is used to interpolate between quality levels and prevent abrupt LOD transitions.

**AAPLRenderer.m**

```objective-c
[submesh computeTextureWeightsForQualityLevel:_currentQualityLevel
                          withGlobalMapWeight:_globalMapWeight];
```

Finally, the render loop draws each submesh in the model with the specific LOD pipeline.

**AAPLRenderer.m**

```objective-c
[renderEncoder drawIndexedPrimitives:metalKitSubmesh.primitiveType
                          indexCount:metalKitSubmesh.indexCount
                           indexType:metalKitSubmesh.indexType
                         indexBuffer:metalKitSubmesh.indexBuffer.buffer
                   indexBufferOffset:metalKitSubmesh.indexBuffer.offset];
```
