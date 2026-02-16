# Encoding argument buffers on the GPU

> [Apple Documentation](https://developer.apple.com/documentation/metal/encoding-argument-buffers-on-the-gpu?language=objc)

## Run the Example

```bash
cargo run --example encoding_argument_buffers_on_the_gpu
```

## Shader Files

- `Renderer/AAPLShaders.metal`

## Resources

- `Documentation/argument-buffers-gpu-encoding-2-ArgumentBuffer.png`
- `Documentation/argument-buffers-gpu-encoding-1-ArgumentBuffer.png`
- `Renderer/Assets.xcassets/Texture64.textureset/Universal.mipmapset/65FlyingSaucer.png`
- `Renderer/Assets.xcassets/Texture53.textureset/Universal.mipmapset/54ConcaveSquare.png`
- `Renderer/Assets.xcassets/Texture31.textureset/Universal.mipmapset/32Butterfly.png`
- `Renderer/Assets.xcassets/Texture6.textureset/Universal.mipmapset/07Pineapple.png`
- `Renderer/Assets.xcassets/Texture20.textureset/Universal.mipmapset/21Oak.png`
- `Renderer/Assets.xcassets/Texture17.textureset/Universal.mipmapset/18Triangles.png`
- `Renderer/Assets.xcassets/Texture42.textureset/Universal.mipmapset/43LightBulb.png`
- `Renderer/Assets.xcassets/Texture39.textureset/Universal.mipmapset/40Clock.png`
- `Renderer/Assets.xcassets/Texture3.textureset/Universal.mipmapset/04Bat.png`
- `Renderer/Assets.xcassets/Texture12.textureset/Universal.mipmapset/13Pumpkin.png`
- `Renderer/Assets.xcassets/Texture25.textureset/Universal.mipmapset/26Snail.png`
- `Renderer/Assets.xcassets/Texture47.textureset/Universal.mipmapset/48Spiral.png`
- `Renderer/Assets.xcassets/Texture56.textureset/Universal.mipmapset/57Pear.png`
- `Renderer/Assets.xcassets/Texture61.textureset/Universal.mipmapset/62Moon.png`
- `Renderer/Assets.xcassets/Texture34.textureset/Universal.mipmapset/35Penguin.png`
- `Renderer/Assets.xcassets/Texture28.textureset/Universal.mipmapset/Pine Tree.png`
- `Renderer/Assets.xcassets/Texture18.textureset/Universal.mipmapset/19Lamp.png`
- `Renderer/Assets.xcassets/Texture51.textureset/Universal.mipmapset/52Mountains.png`
- `Renderer/Assets.xcassets/Texture9.textureset/Universal.mipmapset/10Bird.png`
- `Renderer/Assets.xcassets/Texture66.textureset/Universal.mipmapset/67Corn.png`
- `Renderer/Assets.xcassets/Texture33.textureset/Universal.mipmapset/34Chair.png`
- `Renderer/Assets.xcassets/Texture15.textureset/Universal.mipmapset/16Flower.png`
- `Renderer/Assets.xcassets/Texture22.textureset/Universal.mipmapset/23House.png`
- `Renderer/Assets.xcassets/Texture40.textureset/Universal.mipmapset/41PalmTree.png`
- `Renderer/Assets.xcassets/Texture4.textureset/Universal.mipmapset/05Buldings.png`
- `Renderer/Assets.xcassets/Texture27.textureset/Universal.mipmapset/30Hexagon.png`
- `Renderer/Assets.xcassets/Texture10.textureset/Universal.mipmapset/11CoffeeCup.png`
- `Renderer/Assets.xcassets/Texture45.textureset/Universal.mipmapset/46Caterpillar.png`
- `Renderer/Assets.xcassets/Texture59.textureset/Universal.mipmapset/60FireHydrant.png`
- `Renderer/Assets.xcassets/Texture1.textureset/Universal.mipmapset/02Apple.png`
- `Renderer/Assets.xcassets/Texture48.textureset/Universal.mipmapset/49Rocket.png`
- `Renderer/Assets.xcassets/Texture63.textureset/Universal.mipmapset/64Cola.png`
- `Renderer/Assets.xcassets/Texture54.textureset/Universal.mipmapset/55Owl.png`
- `Renderer/Assets.xcassets/Texture36.textureset/Universal.mipmapset/37Castle.png`
- `Renderer/Assets.xcassets/Texture38.textureset/Universal.mipmapset/39Dolphin.png`
- `Renderer/Assets.xcassets/Texture2.textureset/Universal.mipmapset/03Arrow.png`
- `Renderer/Assets.xcassets/Texture13.textureset/Universal.mipmapset/14Kite.png`
- `Renderer/Assets.xcassets/Texture24.textureset/Universal.mipmapset/25HotAirBalloon.png`
- `Renderer/Assets.xcassets/Texture46.textureset/Universal.mipmapset/47Umbrella.png`
- `Renderer/Assets.xcassets/Texture57.textureset/Universal.mipmapset/58TeaCup.png`
- `Renderer/Assets.xcassets/Texture60.textureset/Universal.mipmapset/61Mouse.png`
- `Renderer/Assets.xcassets/Texture35.textureset/Universal.mipmapset/36Star.png`
- `Renderer/Assets.xcassets/Texture29.textureset/Universal.mipmapset/28Cloud.png`
- `Renderer/Assets.xcassets/Texture65.textureset/Universal.mipmapset/66Axe.png`
- `Renderer/Assets.xcassets/Texture52.textureset/Universal.mipmapset/53Evergreen.png`
- `Renderer/Assets.xcassets/Texture30.textureset/Universal.mipmapset/31Sword.png`
- `Renderer/Assets.xcassets/Texture7.textureset/Universal.mipmapset/08Whale.png`
- `Renderer/Assets.xcassets/Texture21.textureset/Universal.mipmapset/22WaterBottle.png`
- `Renderer/Assets.xcassets/Texture16.textureset/Universal.mipmapset/17PropelerPlane.png`
- `Renderer/Assets.xcassets/Texture43.textureset/Universal.mipmapset/44DumpTruck.png`
- `Renderer/Assets.xcassets/Texture26.textureset/Universal.mipmapset/27Extinguisher.png`
- `Renderer/Assets.xcassets/Texture11.textureset/Universal.mipmapset/12ConvexSquare.png`
- `Renderer/Assets.xcassets/Texture44.textureset/Universal.mipmapset/45Skull.png`
- `Renderer/Assets.xcassets/Texture0.textureset/Universal.mipmapset/01Circles.png`
- `Renderer/Assets.xcassets/Texture58.textureset/Universal.mipmapset/59DragonFly.png`
- `Renderer/Assets.xcassets/Texture49.textureset/Universal.mipmapset/50Milk.png`
- `Renderer/Assets.xcassets/Texture62.textureset/Universal.mipmapset/63Turtle.png`
- `Renderer/Assets.xcassets/Texture55.textureset/Universal.mipmapset/56Car.png`
- `Renderer/Assets.xcassets/Texture37.textureset/Universal.mipmapset/38JetPlane.png`
- `Renderer/Assets.xcassets/Texture19.textureset/Universal.mipmapset/20Cat.png`
- `Renderer/Assets.xcassets/Texture8.textureset/Universal.mipmapset/09Bicycle.png`
- `Renderer/Assets.xcassets/Texture50.textureset/Universal.mipmapset/51Diamond.png`
- `Renderer/Assets.xcassets/Texture32.textureset/Universal.mipmapset/33Sun.png`
- `Renderer/Assets.xcassets/Texture14.textureset/Universal.mipmapset/15Spider.png`
- `Renderer/Assets.xcassets/Texture23.textureset/Universal.mipmapset/24Vortex.png`
- `Renderer/Assets.xcassets/Texture41.textureset/Universal.mipmapset/42ConcaveHexagon.png`
- `Renderer/Assets.xcassets/Texture5.textureset/Universal.mipmapset/06Atom.png`

## Overview

In Using argument buffers with resource heaps, you learned how to combine argument buffers with arrays of resources and resource heaps.

In this sample, you’ll learn how to encode resources into argument buffers with a graphics or compute function. In particular, you’ll learn how to write data into an argument buffer from a compute pass and then read that data in a render pass. The sample renders a grid of multiple quad instances with two textures applied to each, where the textures slide from left to right within the quad and move from left to right between quads.

Getting started

The sample can run only on devices that support Tier 2 argument buffers. Tier 2 devices allow graphics or compute functions to encode data into an argument buffer, whereas Tier 1 devices only allow these functions to read data from an argument buffer. Additionally, Tier 2 devices can access more textures in an instanced draw call than Tier 1 devices. See Improving CPU performance by using argument buffers for more information about argument buffer tiers, limits, and capabilities.

This sample checks for Tier 2 argument buffer support when the renderer is initialized.

AAPLRenderer.m
if(_view.device.argumentBuffersSupport != MTLArgumentBuffersTier2)
{
NSAssert(0, @"This sample requires a Metal device that supports Tier 2 argument buffers.");
}

Encode data into argument buffers

During initialization, the sample encodes data with the CPU into an argument buffer defined by the SourceTextureArguments structure.

AAPLShaders.metal
struct SourceTextureArguments {
texture2d<float>    texture [[ id(AAPLArgumentBufferIDTexture) ]];
};


This argument buffer is backed by the _sourceTextures buffer and is accessed via the source_textures variable in the updateInstances function. source_textures is a pointer to an unbounded array of structures, each of which contains a reference to a texture.

After initialization, for each frame, the sample encodes data with the GPU into a separate argument buffer defined by the InstanceArguments structure.

AAPLShaders.metal
struct InstanceArguments {
vector_float2    position;
texture2d<float> left_texture;
texture2d<float> right_texture;
};


This argument buffer is backed by the _instanceParameters buffer and is accessed via the instance_params variable in the updateInstances, vertexShader, and fragmentShader functions. instance_params is an array of structures whose data is populated in a compute pass and then accessed in a render pass via an instanced draw call.

Create an array of argument buffer structures

The sample defines an InstanceArguments structure into which a compute function, updateInstances, encodes a vector and two textures.

AAPLShaders.metal
struct InstanceArguments {
vector_float2    position;
texture2d<float> left_texture;
texture2d<float> right_texture;
};


Previous argument buffer samples used the encodedLength property to directly determine the required size for the MTLBuffer that backs an argument buffer structure. However, this sample needs one instance of this structure for each quad rendered by a subsequent render pass. Therefore, the sample multiplies the value of encodedLength by the total number of instances, which is defined by the value of the AAPLNumInstances constant.

AAPLRenderer.m
NSUInteger instanceParameterLength = instanceParameterEncoder.encodedLength * AAPLNumInstances;


_instanceParameters = [_device newBufferWithLength:instanceParameterLength options:0];


Note

The [[id(n)]] attribute qualifier isn’t necessary to define the InstanceArguments structure in this sample. This qualifier is needed only when arguments are encoded with the CPU via the Metal API, and not when arguments are encoded with the GPU via a graphics or compute function.

Encode an argument buffer with a compute function

For each quad to be rendered, the sample executes the updateInstances compute function to determine the quad’s position and textures. The compute pass executed by the sample iterates through the instance_params array and encodes the correct data for each quad. The sample encodes data into instance_params by setting InstanceArguments values in the array element at the instanceID index value.

AAPLShaders.metal
device InstanceArguments & quad_params = instance_params[instanceID];


quad_params.position = position;


quad_params.left_texture = source_textures[left_texture_index].texture;
quad_params.right_texture = source_textures[right_texture_index].texture;

Render instances with an argument buffer

The sample issues an instanced draw call to render all the quads while incurring a minimal amount of CPU overhead. Combining this technique with an argument buffer allows the sample to use a unique set of resources for each quad within the same draw call, where each instance draws a single quad.

The sample declares an instanceID variable in both the vertex and fragment function’s signatures. The render pipeline uses instanceID to index into the instance_params array that was previously encoded by the updateInstances compute function.

In the vertex function, instanceID is defined as an argument with the [[instance_id]] attribute qualifier.

AAPLShaders.metal
vertex RasterizerData
vertexShader(uint                            vertexID        [[ vertex_id ]],
uint                            instanceID      [[ instance_id ]],
const device AAPLVertex        *vertices        [[ buffer(AAPLVertexBufferIndexVertices) ]],
const device InstanceArguments *instance_params [[ buffer(AAPLVertexBufferIndexInstanceParams) ]],
constant AAPLFrameState        &frame_state     [[ buffer(AAPLVertexBufferIndexFrameState) ]])


The vertex function reads position data from the argument buffer to render the quad in the right place in the drawable.

AAPLShaders.metal
float2 quad_position = instance_params[instanceID].position;


The vertex function then passes the instanceID variable to the fragment function, via the RasterizerData structure and the [[stage_in]] attribute qualifier. (In the fragment function, instanceID is accessed via the in argument.)

AAPLShaders.metal
fragment float4
fragmentShader(RasterizerData            in              [[ stage_in ]],
device InstanceArguments *instance_params [[ buffer(AAPLFragmentBufferIndexInstanceParams) ]],
constant AAPLFrameState  &frame_state     [[ buffer(AAPLFragmentBufferIndexFrameState) ]])


The fragment function samples from the two textures specified in the argument buffer and then chooses an output sample based on the value of slideFactor.

AAPLShaders.metal
texture2d<float> left_texture = instance_params[instanceID].left_texture;
texture2d<float> right_texture = instance_params[instanceID].right_texture;


float4 left_sample = left_texture.sample(texture_sampler, in.tex_coord);
float4 right_sample = right_texture.sample(texture_sampler, in.tex_coord);


if(frame_state.slideFactor < in.tex_coord.x)
{
output_color = left_sample;
}
else
{
output_color = right_sample;
}


The fragment function outputs the selected sample. The left texture slides in from the left and the right texture slides out to the right. After the right texture has completely slid off the quad, the sample assigns this texture as the left texture in the next compute pass. Thus, each texture moves from left to right across the grid of quads.

Next steps

In this sample, you learned how to encode resources into argument buffers with a graphics or compute function. In Rendering terrain dynamically with argument buffers, you’ll learn how to combine several argument buffer techniques to render a dynamic terrain in real time.

See Also
Argument buffers
Improving CPU performance by using argument buffers
Optimize your app’s performance by grouping your resources into argument buffers.
Managing groups of resources with argument buffers
Create argument buffers to organize related resources.
Tracking the resource residency of argument buffers
Optimize resource performance within an argument buffer.
Indexing argument buffers
Assign resource indices within an argument buffer.
Rendering terrain dynamically with argument buffers
Use argument buffers to render terrain in real time with a GPU-driven pipeline.
Using argument buffers with resource heaps
Reduce CPU overhead by using arrays inside argument buffers and combining them with resource heaps.
MTLArgumentDescriptor
A representation of an argument within an argument buffer.
MTLArgumentEncoder
An interface you can use to encode argument data into an argument buffer.
MTLAttributeStrideStatic
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
