# GPU devices and work submission

> [Apple Documentation](https://developer.apple.com/documentation/metal/gpu-devices-and-work-submission?language=objc)

## Overview

You can use any available GPU’s MTLDevice instance in addition to the default instance that MTLCreateSystemDefaultDevice returns. For each device instance, get its MTLCommandQueue instance, and create one or more MTLCommandBuffer instances to send work to the GPU.

When the system suspends your app, use the command queue to finish command buffers already in progress. See Preparing your Metal app to run in the background for more information.

Locating and inspecting a GPU device
Getting the default GPU
Select the system’s default GPU device on which to run your Metal code.
Detecting GPU features and Metal software versions
Use the device object’s properties to determine how you perform tasks in Metal.
MTLCreateSystemDefaultDevice
Returns the device instance Metal selects as the default.
MTLDevice
The main Metal interface to a GPU that apps use to draw graphics and run computations in parallel.
Multi-GPU systems
Locate and work with internal and external GPUs and their displays, video memory, and performance tradeoffs.
Submitting work to a GPU with Metal 4
MTL4CommandQueue
An abstraction representing a command queue that you use commit and synchronize command buffers and to perform other GPU operations.
MTL4CommandQueueDescriptor
Groups together parameters for the creation of a new command queue.
MTL4CommandQueueError
Enumeration of kinds of errors that committing an array of command buffers instances can produce.
MTL4CommandQueueErrorDomain
MTL4CommandBuffer
Records a sequence of GPU commands.
MTL4CommandBufferOptions
Options to configure a command buffer before encoding work into it.
MTL4CommandEncoder
An encoder that writes GPU commands into a command buffer.
MTL4RenderEncoderOptions
Custom render pass options you specify at encoder creation time.
MTL4ArgumentTable
Provides a mechanism to manage and provide resource bindings for buffers, textures, sampler states and other Metal resources.
MTL4ArgumentTableDescriptor
Groups parameters for the creation of a Metal argument table.
MTL4CommandAllocator
Manages the memory backing the encoding of GPU commands into command buffers.
MTL4CommandAllocatorDescriptor
Groups together parameters for creating a command allocator.
MTL4CommitOptions
Represents options to configure a commit operation on a command queue.
MTL4CommitFeedback
Describes an object containing debug information from Metal to your app after completing a workload.
MTL4CommitFeedbackHandler
Defines the block signature for a callback Metal invokes to provide your app feedback after completing a workload.
MTL4CounterHeap
Represents an opaque, driver-controlled section of memory that can store GPU counter data.
MTL4CounterHeapDescriptor
Groups together parameters for configuring a counter heap object at creation time.
MTL4CounterHeapType
Defines the type of a MTL4CounterHeap and the contents of its entries.
MTL4TimestampHeapEntry
Represents a timestamp data entry in a counter heap of type MTL4CounterHeapTypeTimestamp.
MTL4TimestampGranularity
Provides a hint to the system about the desired accuracy when writing GPU counter timestamps.
Submitting work to a GPU
Setting up a command structure
MTLCommandQueue
An instance you use to create, submit, and schedule command buffers to a specific GPU device to run the commands within those buffers.
MTLCommandQueueDescriptor
A configuration that customizes the behavior for a new command queue.
MTLCommandBuffer
A container that stores a sequence of GPU commands that you encode into it.
MTLCommandBufferDescriptor
A configuration that customizes the behavior for a new command buffer.
MTLCommandBufferErrorDomain
The domain for Metal command buffer errors.
MTLCommandBufferError
Error codes that indicate why a GPU is unable to finish running a command buffer.
MTLCommandEncoder
An encoder that writes GPU commands into a command buffer.
Suspending work on a GPU
Preparing your Metal app to run in the background
Prepare your app to move into the background by pausing future GPU use and ensuring previous work is scheduled.
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
