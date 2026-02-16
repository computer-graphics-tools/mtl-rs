# Resource loading

> [Apple Documentation](https://developer.apple.com/documentation/metal/resource-loading?language=objc)

## Overview

Metal 3 adds input/output command queues and buffers that make the most of a device’s storage hardware, including flash storage and the unified memory architecture of Apple silicon, when available. When you run a dedicated input/output queue alongside your GPU tasks, you can synchronize them with Metal shared events. With this approach, you can minimize load screen times by fetching the essential assets first and streaming the rest as you need them. You can also start multiple input/output command buffers to load different asset batches and later cancel the ones you don’t need. Ensure that time-sensitive assets, such as sound effects, load with lower latency by running those command buffers on higher-priority queues that you create.

First, create MTLIOCommandQueue instances by configuring an MTLIOCommandQueueDescriptor instance and passing it to an MTLDevice instance’s newIOCommandQueueWithDescriptor:error: method.

let commandQueueDescriptor = MTLIOCommandQueueDescriptor()


commandQueueDescriptor.type = .concurrent
commandQueueDescriptor.priority = .normal


let ioCommandQueue = try device.makeIOCommandQueue(descriptor:
commandQueueDescriptor)


For each queue, create one or more MTLIOCommandBuffer instances by calling the queue’s commandBuffer or commandBufferWithUnretainedReferences method. For each command buffer, load the assets you want by calling any of the MTLIOCommandBuffer protocol’s load methods. For example:

The loadBuffer:offset:size:sourceHandle:sourceHandleOffset: method loads an asset into an MTLBuffer.

The loadTexture:slice:level:size:sourceBytesPerRow:sourceBytesPerImage:destinationOrigin:sourceHandle:sourceHandleOffset: method loads an asset into an MTLTexture.

The loadBytes:size:sourceHandle:sourceHandleOffset: method loads an asset, such as an audio file, into a CPU-accessible memory buffer.

let ioCommandBuffer = ioCommandQueue.makeCommandBuffer()


ioCommandBuffer.load(texture,
slice: 0,
level: 0,
size: textureSize,
sourceBytesPerRow: bytesPerRow,
sourceBytesPerImage: bytesPerImage,
destinationOrigin: origin,
sourceHandle: fileHandle,
sourceHandleOffset: 0)


ioCommandBuffer.load(buffer,
offset: 0,
size: bufferSize,
sourceHandle: fileHandle,
sourceHandleOffset: 0)


ioCommandBuffer.commit()


For each asset, create an MTLIOFileHandle instance using the input/output command buffer’s load methods. To create a file handle for your asset, call an MTLDevice instance’s newIOHandleWithURL:error: or newIOHandleWithURL:compressionMethod:error: method.

func createHandleForFile(at url: URL, with device: MTLDevice) -> MTLIOFileHandle? {
return try? device.makeIOHandle(url: url)
}


Note

You need to create each file handle using the same MTLDevice instance that created the MTLIOCommandQueue and MTLIOCommandBuffer instances that load the files.

To help minimize your appʼs storage footprint, compress your assets at development time. First, create a new compression context with the MTLIOCreateCompressionContext function. Then, add data for an asset to the compression context using the MTLIOCompressionContextAppendData function. Finally, call the MTLIOFlushAndDestroyCompressionContext function to save the context to a compressed file that you add to your project.

I/O command queues
MTLIOCommandQueue
A command queue that schedules input/output commands for reading files in the file system, and writing to GPU resources and memory.
MTLIOCommandQueueDescriptor
A configuration template you use to create a new input/output command queue.
MTLIOPriority
MTLIOCommandQueueType
MTLIOScratchBufferAllocator
A protocol your app implements to provide scratch memory to an input/output command queue.
MTLIOScratchBuffer
A protocol your app implements that wraps a Metal buffer instance to serve as scratch memory for an input/output command queue.
I/O command buffers
MTLIOCommandBuffer
A command buffer that contains input/output commands that work with files in the file systems and Metal resources.
MTLIOFileHandle
Represents a raw or compressed file, such as a resource asset file in your app’s bundle.
MTLIOCommandBufferHandler
A convenience type that defines the signature of an input/output command buffer’s completion handler.
MTLIOStatus
Represents the state of an input/output command buffer.
MTLIOError
The error codes for creating an input/output file handle.
MTLIOErrorDomain
The domain for input/output command queue errors.
Asset compression
MTLIOCreateCompressionContext
Creates a compression context that you use to compress data into a single file.
MTLIOCompressionMethod
The compression codecs that Metal supports for input/output handles.
MTLIOCompressionContextDefaultChunkSize
Returns a compression chunk size you can use as a default for creating a compression context.
MTLIOCompressionContext
A pointer that represents the state of a file compression session in progress.
MTLIOCompressionContextAppendData
Adds data to a compression context.
MTLIOFlushAndDestroyCompressionContext
Finishes compressing and saves the file that a compression context represents.
MTLIOCompressionStatus
Represents the final state of a compression context.
See Also
Resources
Resource fundamentals
Control the common attributes of all Metal memory resources, including buffers and textures, and how to configure their underlying memory.
Buffers
Create and manage untyped data your app uses to exchange information with its shader functions.
Textures
Create and manage typed data your app uses to exchange information with its shader functions.
Memory heaps
Take control of your app’s GPU memory management by creating a large memory allocation for various buffers, textures, and other resources.
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
