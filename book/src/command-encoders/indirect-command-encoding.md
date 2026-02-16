# Indirect command encoding

> [Apple Documentation](https://developer.apple.com/documentation/metal/indirect-command-encoding?language=objc)

## Overview

You can use an MTLIndirectCommandBuffer instance to store draw commands and invoke them at a later time. Metal executes all the draw commands in an indirect command buffer each time you submit it. This means you can use indirect command buffers multiple times, unlike MTLCommandBuffer instances, which are all single-use.

You can encode an indirect command buffer to run on either the CPU or the GPU. However, the GPU gives you the ability to immediately use the output of one pass as the input of a subsequent pass. For example, you can create an indirect command buffer with commands that conditionally draw visible items by running:

A compute kernel that identifies visible geometry and saves it to a result buffer

An indirect command buffer that uses the result buffer as its input to make decisions on what to draw

Indirect command buffers
Creating an indirect command buffer
Configure a descriptor to specify the properties of an indirect command buffer.
Specifying drawing and dispatch arguments indirectly
Use indirect commands if you don’t know your draw or dispatch call arguments when you encode the command.
Encoding indirect command buffers on the CPU
Reduce CPU overhead and simplify your command execution by reusing commands.
Encoding indirect command buffers on the GPU
Maximize CPU to GPU parallelization by generating render commands on the GPU.
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
Indirect compute commands
MTLIndirectComputeCommand
A compute command in an indirect command buffer.
MTLRegion
The bounds for a subset of an instance’s elements.
MTLSize
A type that represents one, two, or three dimensions of a type instance, such as an array or texture.
MTLOrigin
The coordinates for the front upper-left corner of a region.
MTLStageInRegionIndirectArguments
The data layout required for the arguments needed to specify the stage-in region.
MTLDispatchThreadgroupsIndirectArguments
The data layout required for arguments needed to specify the size of threadgroups.
Render compute commands
MTLIndirectRenderCommand
A render command in an indirect command buffer.
MTLDrawPatchIndirectArguments
The data layout required for drawing patches via indirect buffer calls.
MTLDrawPrimitivesIndirectArguments
The data layout required for drawing primitives via indirect buffer calls.
MTLDrawIndexedPrimitivesIndirectArguments
The data layout required for drawing indexed primitives via indirect buffer calls.
See Also
Command encoders
Render passes
Encode a render pass to draw graphics into an image.
Compute passes
Encode a compute pass that runs computations in parallel on a thread grid, processing and manipulating Metal resource data on multiple cores of a GPU.
Machine learning passes
Add machine learning model inference to your Metal app’s GPU workflow.
Blit passes
Encode a block information transfer pass to adjust and copy data to and from GPU resources, such as buffers and textures.
Ray tracing with acceleration structures
Build a representation of your scene’s geometry using triangles and bounding volumes to quickly trace rays through the scene.
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
