# Encoding indirect command buffers on the GPU

> [Apple Documentation](https://developer.apple.com/documentation/metal/encoding-indirect-command-buffers-on-the-gpu?language=objc)

## Run the Example

```bash
cargo run --example encoding_indirect_command_buffers_on_the_gpu
```

## Shader Files

- `Renderer/AAPLShaders.metal`

## Resources

- `Documentation/icbs-with-gpu-encoding-2-CpuRoundTrip.png`
- `Documentation/icbs-with-gpu-encoding-1-GpuDrivenPipeline.png`

## Overview

This sample app demonstrates how to use indirect command buffers (ICB) to issue rendering instructions from the GPU. When you have a rendering algorithm that runs in a compute kernel, use ICBs to generate draw calls based on your algorithm’s results. The sample app uses a compute kernel to remove invisible objects submitted for rendering, and generates draw commands only for the objects currently visible in the scene.

Without ICBs, you can’t submit rendering commands on the GPU. Instead, the CPU waits for your compute kernel’s results before generating the render commands. Then, the GPU waits for the rendering commands to make it across the CPU to GPU bridge. The following diagram shows how this creates a slower round trip:

The sample code project, Encoding Indirect Command Buffers on the CPU introduces ICBs by creating a single ICB to reuse its commands every frame. While the former sample saved expensive command-encoding time by reusing commands, this sample uses ICBs to effect a GPU-driven rendering pipeline.

The techniques shown by this sample include issuing draw calls from the GPU, and the process of executing a select set of draws.

Getting started

This project contains targets for macOS and iOS. Run the iOS scheme on a physical device because Metal isn’t supported in the simulator.

The sample calls an MTLComputeCommandEncoder instances’s
dispatchThreads:threadsPerThreadgroup: method, which is available to a GPU that supports the following feature sets and later:

MTLFeatureSet_iOS_GPUFamily4_v2

MTLFeatureSet_macOS_GPUFamily2_v1

Define the data read by the ICB

In an ideal scenario, you store each mesh in its own buffer. However, on iOS, kernels running on the GPU can only access a limited number of data buffers per execution. To reduce the number of buffers needed during the ICBs execution, you pack all meshes into a single buffer at varying offsets. Then, use another buffer to store the offset and size of each mesh. The process to do this follows.

At initialization, create the data for each mesh:

AAPLRenderer.m
for(int objectIdx = 0; objectIdx < AAPLNumObjects; objectIdx++)
{
uint32_t numTeeth = random() % 50 + 3;
float innerRatio = 0.2 + (random() / (1.0 * RAND_MAX)) * 0.7;
float toothWidth = 0.1 + (random() / (1.0 * RAND_MAX)) * 0.4;
float toothSlope = (random() / (1.0 * RAND_MAX)) * 0.2;


tempMeshes[objectIdx] = [self newGearMeshWithNumTeeth:numTeeth
innerRatio:innerRatio
toothWidth:toothWidth
toothSlope:toothSlope];
}


Count the individual and accumulated mesh sizes and create the container buffer:

AAPLRenderer.m
size_t bufferSize = 0;


for(int objectIdx = 0; objectIdx < AAPLNumObjects; objectIdx++)
{
size_t meshSize = sizeof(AAPLVertex) * tempMeshes[objectIdx].numVerts;
bufferSize += meshSize;
}


_vertexBuffer = [_device newBufferWithLength:bufferSize options:0];


Finally, insert each mesh into the container buffer while noting its offset and size in the second buffer:

AAPLRenderer.m
for(int objectIdx = 0; objectIdx < AAPLNumObjects; objectIdx++)
{


params[objectIdx].numVertices = tempMeshes[objectIdx].numVerts;


size_t meshSize = sizeof(AAPLVertex) * tempMeshes[objectIdx].numVerts;


params[objectIdx].startVertex = currentStartVertex;




AAPLVertex* meshStartAddress = ((AAPLVertex*)_vertexBuffer.contents) + currentStartVertex;


memcpy(meshStartAddress, tempMeshes[objectIdx].vertices, meshSize);


currentStartVertex += tempMeshes[objectIdx].numVerts;


free(tempMeshes[objectIdx].vertices);




vector_float2 gridPos = (vector_float2){objectIdx % AAPLGridWidth, objectIdx / AAPLGridWidth};
params[objectIdx].position = gridPos * AAPLObjecDistance;


params[objectIdx].boundingRadius = AAPLObjectSize / 2.0;
}

Update the data read by the ICB dynamically

By culling non-visible vertices from the data fed to the rendering pipeline, you save significant rendering time and effort. To do that, use the same compute kernel that encodes the ICB’s commands to continually update the ICB’s data buffers:

AAPLShaders.metal
kernel void
cullMeshesAndEncodeCommands(uint                         objectIndex   [[ thread_position_in_grid ]],
constant AAPLFrameState     *frame_state   [[ buffer(AAPLKernelBufferIndexFrameState) ]],
device AAPLObjectPerameters *object_params [[ buffer(AAPLKernelBufferIndexObjectParams)]],
device AAPLVertex           *vertices      [[ buffer(AAPLKernelBufferIndexVertices) ]],
device ICBContainer         *icb_container [[ buffer(AAPLKernelBufferIndexCommandBufferContainer) ]])
{
float2 worldObjectPostion  = frame_state->translation + object_params[objectIndex].position;
float2 clipObjectPosition  = frame_state->aspectScale * AAPLViewScale * worldObjectPostion;


const float rightBounds =  1.0;
const float leftBounds  = -1.0;
const float upperBounds =  1.0;
const float lowerBounds = -1.0;


bool visible = true;


const float2 boundingRadius = frame_state->aspectScale * AAPLViewScale * object_params[objectIndex].boundingRadius;


if(clipObjectPosition.x + boundingRadius.x < leftBounds  ||
clipObjectPosition.x - boundingRadius.x > rightBounds ||
clipObjectPosition.y + boundingRadius.y < lowerBounds ||
clipObjectPosition.y - boundingRadius.y > upperBounds)
{
visible = false;
}
render_command cmd(icb_container->commandBuffer, objectIndex);


if(visible)
{
cmd.set_vertex_buffer(frame_state, AAPLVertexBufferIndexFrameState);
cmd.set_vertex_buffer(object_params, AAPLVertexBufferIndexObjectParams);
cmd.set_vertex_buffer(vertices, AAPLVertexBufferIndexVertices);


cmd.draw_primitives(primitive_type::triangle,
object_params[objectIndex].startVertex,
object_params[objectIndex].numVertices, 1,
objectIndex);
}


}


The parallel nature of the GPU partitions the compute task for you, resulting in multiple offscreen meshes getting culled concurrently.

Pass an ICB to a compute kernel using an argument buffer

To get an ICB on the GPU and make it accessible to a compute kernel, you pass it through an argument buffer, as follows:

Define the container argument buffer as a structure that contains one member, the ICB:

AAPLShaders.metal
struct ICBContainer
{
command_buffer commandBuffer [[ id(AAPLArgumentBufferIDCommandBuffer) ]];
};


Encode the ICB into the argument buffer:

AAPLRenderer.m
id<MTLArgumentEncoder> argumentEncoder =
[GPUCommandEncodingKernel newArgumentEncoderWithBufferIndex:AAPLKernelBufferIndexCommandBufferContainer];


_icbArgumentBuffer = [_device newBufferWithLength:argumentEncoder.encodedLength
options:MTLResourceStorageModeShared];
_icbArgumentBuffer.label = @"ICB Argument Buffer";


[argumentEncoder setArgumentBuffer:_icbArgumentBuffer offset:0];


[argumentEncoder setIndirectCommandBuffer:_indirectCommandBuffer
atIndex:AAPLArgumentBufferIDCommandBuffer];


Pass the ICB (_indirectCommandBuffer) to the kernel by setting the argument buffer on the kernel’s compute command encoder:

AAPLRenderer.m
[computeEncoder setBuffer:_icbArgumentBuffer offset:0 atIndex:AAPLKernelBufferIndexCommandBufferContainer];


Because you pass the ICB through an argument buffer, standard argument buffer rules apply. Call useResource on the ICB to tell Metal to prepare its use:

AAPLRenderer.m
[computeEncoder useResource:_indirectCommandBuffer usage:MTLResourceUsageWrite];

Encode and optimize ICB commands

Reset the ICB’s commands to their initial before beginning encoding:

AAPLRenderer.m
[resetBlitEncoder resetCommandsInBuffer:_indirectCommandBuffer
withRange:NSMakeRange(0, AAPLNumObjects)];


Encode the ICB’s commands by dispatching the compute kernel:

AAPLRenderer.m
[computeEncoder dispatchThreads:MTLSizeMake(AAPLNumObjects, 1, 1)
threadsPerThreadgroup:MTLSizeMake(threadExecutionWidth, 1, 1)];


Optimize your ICB commands to remove empty commands or redundant state by calling optimizeIndirectCommandBuffer:withRange::

AAPLRenderer.m
[optimizeBlitEncoder optimizeIndirectCommandBuffer:_indirectCommandBuffer
withRange:NSMakeRange(0, AAPLNumObjects)];


This sample optimizes ICB commands because redundant state results from the kernel setting a buffer for each draw, and encoding empty commands for each invisible object. By removing the empty commands, you can free up a significant number of blank spaces in the command buffer that Metal otherwise spends time skipping at runtime.

Note

If you optimize an indirect command buffer, you won’t be able to call executeCommandsInBuffer:withRange: with a range that starts in the optimized region. Instead, specify a range staring at the beginning and finishing within or at the end of the optimized region.

Execute the ICB

Draw the onscreen meshes by calling executeCommandsInBuffer on your render command encoder:

AAPLRenderer.m
[renderEncoder executeCommandsInBuffer:_indirectCommandBuffer withRange:NSMakeRange(0, AAPLNumObjects)];


While you can encode an ICB’s commands in a compute kernel, you call executeCommandsInBuffer from your host app to encode a single command that contains all of the commands encoded by the compute kernel. By doing this, you choose the queue and buffer that the ICB’s commands go into. When you call executeIndirectCommandBuffer determines the placement of the ICB’s commands among any other commands you may also encode in the same buffer.

See Also
Indirect command buffers
Creating an indirect command buffer
Configure a descriptor to specify the properties of an indirect command buffer.
Specifying drawing and dispatch arguments indirectly
Use indirect commands if you don’t know your draw or dispatch call arguments when you encode the command.
Encoding indirect command buffers on the CPU
Reduce CPU overhead and simplify your command execution by reusing commands.
MTLIndirectCommandBuffer
A command buffer containing reusable commands, encoded either on the CPU or GPU.
MTLIndirectCommandBufferDescriptor
A configuration you create to customize an indirect command buffer.
MTLIndirectCommandType
The types of commands that you can encode into the indirect command buffer.
MTLIndirectCommandBufferExecutionRange
A range of commands in an indirect command buffer.
MTLIndirectCommandBufferExecutionRangeMake
Creates a command execution range.
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
