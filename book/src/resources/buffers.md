# Buffers

> [Apple Documentation](https://developer.apple.com/documentation/metal/buffers?language=objc)

## Overview

Each MTLBuffer instance represents a general purpose, typeless memory allocation that your app uses to send and retrieve data from a shader. Your app decides how to use and interpret the buffer’s underlying bytes.

You create buffers from either an MTLDevice or MTLHeap instance.

C++
let deviceBuffer = device.makeBuffer(length: bufferSize,
options: .storageModeShared)


let heapBuffer = heap.makeBuffer(length: bufferSize,
options: .storageModePrivate)


Buffers inherently support the MTLResource protocol’s properties and methods, including storageMode, which controls how the GPU handles its memory (see Resource fundamentals).

General purpose buffers
Store arbitrary data in a buffer, such as vertex locations or your own custom data structure.
MTLBuffer
A resource that stores data in a format defined by your app.
Argument buffers
Group resources together into an argument buffer.
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
Encoding argument buffers on the GPU
Use a compute pass to encode an argument buffer and access its arguments in a subsequent render pass.
Using argument buffers with resource heaps
Reduce CPU overhead by using arrays inside argument buffers and combining them with resource heaps.
MTLArgumentDescriptor
A representation of an argument within an argument buffer.
MTLArgumentEncoder
An interface you can use to encode argument data into an argument buffer.
MTLAttributeStrideStatic
Model I/O interoperability
Load complex 3D meshes and textures from Model I/O assets, and prepare to draw them in your Metal render pipelines.
MTKMesh
A container for the vertex data of a Model I/O mesh, suitable for use in a Metal app.
MTKMeshBuffer
A buffer that backs the vertex data of a Model I/O mesh, suitable for use in a Metal app.
MTKMeshBufferAllocator
An interface for allocating a MetalKit buffer that backs the vertex data of a Model I/O mesh, suitable for use in a Metal app.
MTKSubmesh
A container for the index data of a Model I/O submesh, suitable for use in a Metal app.
MTKModelError
Constants used to declare Model Errors.
MTKMetalVertexFormatFromModelIO
Returns a converted Metal vertex format.
MTKModelIOVertexFormatFromMetal
Returns a converted Model I/O vertex format.
MTKMetalVertexDescriptorFromModelIO
Returns a partially converted Metal vertex descriptor.
MTKMetalVertexDescriptorFromModelIOWithError
Returns a partially converted Metal vertex descriptor, reporting any error that occurs.
MTKModelIOVertexDescriptorFromMetal
Returns a partially converted Model I/O vertex descriptor.
MTKModelIOVertexDescriptorFromMetalWithError
Returns a partially converted Model I/O vertex descriptor, reporting any error that occurs.
See Also
Resources
Resource fundamentals
Control the common attributes of all Metal memory resources, including buffers and textures, and how to configure their underlying memory.
Textures
Create and manage typed data your app uses to exchange information with its shader functions.
Memory heaps
Take control of your app’s GPU memory management by creating a large memory allocation for various buffers, textures, and other resources.
Resource loading
Load assets in your games and apps quickly by running a dedicated input/output queue alongside your GPU tasks.
Resource synchronization
Prevent multiple commands that can access the same resources simultaneously by coordinating those reads and writes with barriers, fences, or events.
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
