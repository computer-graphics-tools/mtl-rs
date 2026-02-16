# Resource synchronization

> [Apple Documentation](https://developer.apple.com/documentation/metal/resource-synchronization?language=objc)

## Overview

By design, GPUs can run multiple commands in parallel. Many of those commands access underlying memory of resources, including buffers and textures, with read and write operations. Commands can have an access conflict when one or more of them has a memory write, or store, operation and at least one other command has a memory read, or load, operation.

Synchronize commands submitted to an MTL4CommandQueue instance when they have an access conflict with a resource. Access conflicts can cause problems in your app, such as nondeterministic behavior. For example, without synchronization, a draw command that reads from a texture to get the results of an earlier draw command might start loading from the texture’s memory before the other command finishes writing its output to that texture.

Important

The value of an MTLResource instance’s hazardTrackingMode property has no effect on the work you submit to an MTL4CommandQueue.

Look for resources with access conflicts

Start by identifying the commands that access the same resource, such as an MTLBuffer or an MTLTexture instance. Consider any resource that multiple passes can access concurrently by any means, including:

Resource bindings, which you configure directly with the MTLCommandEncoder or MTL4ArgumentTable protocols

Argument buffers, which you create and configure (see Managing groups of resources with argument buffers)

Attachments for a render pass, which are textures that store rendering information, such as color, depth, or stencil data

It’s okay for multiple commands to load data from the same resource memory at the same time because they all read from memory without modifying it. For example, multiple commands can load segments of a buffer at the same time, even if those segments overlap, because none of them are writing to that memory.

However, an app can introduce an access conflict when it encodes commands that both read and write to the same memory of a resource.

Locate potential access conflicts by checking which resources apply to multiple commands, where at least one of those commands modifies the resource with a store operation. Commands with an access conflict that run concurrently create a race condition that can yield inconsistent results. This is because any overlapping memory load and store operations don’t always run in the same order relative to each other. Each time a GPU runs a batch of commands without synchronization, a load operation from one command can run before, during, or after a store operation.

Note

Even though race conditions typically arise from an implementation mistake, some apps intentionally introduce them as an optimization technique, such as two commands that don’t need synchronization because they store the same modifications to a resource.

Check render pass commands that access its attachments

A render pass that writes to an attachment may introduce an access conflict because a render pass can have implicit load and store operations for that attachment. For render passes, look for potential conflicts with its attachment textures by:

Noting the attachments the pass loads and stores at the beginning and end of the pass, respectively

Finding any commands that read or modify those attachment textures

Render command encoders add a load operation, a store operation, or both for each applicable texture attachment of the render pass it encodes. You configure which attachments, if any, the GPU loads at the beginning of the pass when you configure the loadAction property of the MTLRenderPassAttachmentDescriptor instance that applies to each attachment. Similarly, you configure which attachments, if any, the GPU stores at the end of the pass by setting the storeAction property of the MTLRenderPassAttachmentDescriptor instance that applies to each attachment.

Note which attachment textures that have a load action that’s equal to MTLLoadActionLoad, or a store action that’s equal to MTLStoreActionStore, then look for commands that also load and store those attachments.

Note

You can use any combination of load and store actions for each attachment.

Check compute pass commands that can run concurrently

An MTL4ComputeCommandEncoder instance creates a compute pass that runs commands concurrently on the GPU, which can introduce access conflicts.

By default, an MTLComputeCommandEncoder encodes a compute pass that runs its commands serially, However, you can create one that encodes a concurrent compute pass by configuring an MTLComputePassDescriptor instance’s dispatchType property to MTLDispatchTypeConcurrent.

Ignore memory operations the system already guarantees

Metal provides several built-in resource ordering guarantees within compute and render passes, which your app doesn’t need to synchronize.

For example, you don’t need to synchronize compute or render passes when they access an instance of an atomic type because they serially access those instances. See section 2.6 Atomic Data Types in the Metal Shading Language Specification (PDF) for more information.

Render passes also order memory operations for specific cases, including:

A render-pass attachment’s load and store operations run in primitive order for each fragment, which is the order of your app’s draw commands and the order of each primitive within a draw call.

A fragment shader’s load and store operations for a raster-order group run in primitive order for each fragment.

A tile shader’s load and store operations run in the same order as your app’s tile dispatch calls and on a per-tile basis.

Resolve access conflicts with synchronization

You can address access conflicts with one or more synchronization mechanisms. Each synchronization mechanism forces the GPU to pause before it runs a stage that accesses a resource, until another stage finishes. This means the memory operations from one stage completely finish before another stage can run its memory operations.

You can choose one of the following synchronization mechanisms, which are in order of increasing scope:

Intrapass barriers

An intrapass barrier has the smallest scope because it only applies to commands within the same pass. For more information, see Synchronizing stages within a pass.

Fences

A fence synchronizes resource memory operations across different passes within a command queue. Update a fence from a producing pass that modifies one or more resources, and then wait for that fence in a consuming pass that needs the output in those resources from the producing pass. For more information, see Synchronizing passes with a fence.

Intraqueue barriers

An intraqueue barrier also synchronizes resource memory operations across different passes within a command queue, but is more coarse than a fence. Metal has two kinds of intraqueue barrier that synchronize different dependencies

A consumer queue barrier indicates which stages of a pass depend on and consume output from specific stages of one or more previous passes in the same queue. For more information, see Synchronizing passes with consumer barriers.

A producer queue barrier indicates which stages of the a pass produce output that subsequent passes on the same queue depend on. For more information, see Synchronizing passes with producer barriers.

Events

An MTLEvent instance synchronizes resource memory operations in passes across all command queues.

Shared events

An MTLSharedEvent instance has the largest scope because it synchronizes resource memory operations from other parts of the app, including:

Code that runs on the CPU

Passes in any command queue from the same MTLDevice instance

Passes in any command queue from other MTLDevice instances

Tip

Select the synchronizing mechanism with smallest scope that can resolve the access conflict because larger scopes pause the GPU from doing more work than smaller scopes.

Track hazards with the framework prior to Metal 4

The Metal framework automatically synchronizes resource access conflicts for the commands you submit to an MTLCommandQueue instance, and only for the resources that:

You configure its hazardTrackingMode property to MTLHazardTrackingModeTracked

You directly bind that resource to an encoder type that adopts the MTLCommandEncoder protocol

Resources you create from an MTLDevice instance default to MTLHazardTrackingModeTracked, and the resources you create from an MTLHeap instance default to MTLHazardTrackingModeUntracked. For more information, see Resource fundamentals and Memory heaps.

Synchronizing with barriers and fences
Synchronizing stages within a pass
Block GPU stages in the a pass from running until other stages in the same pass finish.
Synchronizing passes with a fence
Block GPU stages in a pass until another pass unblocks it by signaling a fence.
Synchronizing passes with consumer barriers
Block GPU stages in a pass, and all subsequent passes, from running until stages from earlier passes finish.
Synchronizing passes with producer barriers
Block GPU stages in subsequent passes from running until stages in a pass, and earlier passes, finish.
Synchronizing CPU and GPU work
Avoid stalls between CPU and GPU work by using multiple instances of a resource.
Implementing a multistage image filter using heaps and fences
Use fences to synchronize access to resources allocated on a heap.
MTLStages
The segments of command execution within the Metal pass types.
MTLFence
A synchronization mechanism that orders memory operations between GPU passes.
MTLRenderStages
The stages in a render pass that triggers a synchronization command.
MTLBarrierScope
Describes the types of resources that a barrier operates on.
MTL4VisibilityOptions
Memory consistency options for synchronization commands.
Synchronizing with events
Implementing a multistage image filter using heaps and events
Use events to synchronize access to resources allocated on a heap.
About synchronization events
Synchronize access to resources in your app by signaling events.
Synchronizing events within a single device
Use nonshareable events to synchronize your app’s work within a single device.
Synchronizing events across multiple devices or processes
Use shareable events to synchronize your app’s work across multiple devices or processes.
Synchronizing events between a GPU and the CPU
Use shareable events to synchronize your app’s work between a GPU and the CPU.
MTLEvent
A type that synchronizes memory operations to one or more resources within a single Metal device.
MTLSharedEvent
A type that synchronizes memory operations to one or more resources across multiple CPUs, GPUs, and processes.
MTLSharedEventHandle
An instance you use to recreate a shareable event.
MTLSharedEventListener
A listener for shareable event notifications.
MTLSharedEventNotificationBlock
A block of code invoked after a shareable event’s signal value equals or exceeds a given value.
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
Resource loading
Load assets in your games and apps quickly by running a dedicated input/output queue alongside your GPU tasks.
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
