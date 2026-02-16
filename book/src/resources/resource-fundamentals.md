# Resource fundamentals

> [Apple Documentation](https://developer.apple.com/documentation/metal/resource-fundamentals?language=objc)

## Overview

A resource is a memory asset, such as an MTLBuffer or MTLTexture, that a GPU can access (see Buffers and Textures).

You can either allocate a resource from an MTLDevice instance or an MTLHeap instance (see Memory heaps). Metal sets a resource’s hazardTrackingMode property to MTLHazardTrackingModeDefault if you don’t select another tracking mode. The default value depends on what Metal instance creates the resource.

Important

The value of an MTLResource instance’s hazardTrackingMode property has no effect on the work you submit to an MTL4CommandQueue (see Resource synchronization) or resources that commands access through an argument buffer.

Each resource your app creates typically uses one of these storage modes:

MTLStorageModePrivate

Apps can only access resources in private storage from the GPU.

MTLStorageModeShared

Apps can access resources in shared storage from both the CPU and the GPU.

MTLStorageModeManaged

Apps can access resources in managed storage from both the CPU and the GPU, just like shared storage. However, the GPU backs resources in managed mode with memory in private storage.

Private mode resources give your app optimization opportunities that shared mode resources don’t. Managed mode resources also give your app the same opportunities and allow your to app access them from the CPU.

Resource management
Setting resource storage modes
Set a storage mode that defines the memory location and access permissions of a resource.
Choosing a resource storage mode for Apple GPUs
Select an appropriate storage mode for your textures and buffers on Apple GPUs.
Choosing a resource storage mode for Intel and AMD GPUs
Select an appropriate storage mode for your textures and buffers on AMD and Intel GPUs.
Copying data to a private resource
Use a blit command encoder to copy buffer or texture data to a private resource.
Synchronizing a managed resource in macOS
Manually synchronize memory for a Metal resource in apps.
Transferring data between connected GPUs
Use high-speed connections between GPUs to transfer data quickly.
Reducing the memory footprint of Metal apps
Learn best practices for using memory efficiently in iOS and tvOS.
Residency sets
Simplifying GPU resource management with residency sets
Organize your resources into groups and influence when they become accessible to the GPU.
MTLResidencySet
A collection of resource allocations that can move in and out of resident memory.
MTLResidencySetDescriptor
A configuration that customizes the behavior for a residency set.
View pools
MTLResourceViewPool
Contains views over resources of a specific type, and allows you to manage those views.
MTLResourceViewPoolDescriptor
Provides parameters for creating a resource view pool.
MTLTextureViewPool
A pool of lightweight texture views.
MTLTextureViewDescriptor
Tensors
MTLTensor
A resource representing a multi-dimensional array that you can use with machine learning workloads.
MTLTensorDescriptor
A configuration type for creating new tensor instances.
MTLTensorExtents
An array of length matching the rank, holding the dimensions of a tensor.
MTLTensorReferenceType
An object that represents a tensor in the shading language in a struct or array.
MTLTensorUsage
The type that represents the different contexts for a tensor.
MTLTensorDomain
An error domain for errors that pertain to creating a tensor.
MTLTensorBinding
An object that represents a tensor bound to a graphics or compute function or a machine learning function.
MTLTensorError
The error codes that Metal can raise when you create a tensor.
MTLTensorDataType
The possible data types for the elements of a tensor.
MTLTensorDomain
An error domain for errors that pertain to creating a tensor.
MTL_TENSOR_MAX_RANK
Sparse resources
MTLBufferSparseTier
Enumerates the different support levels for sparse buffers.
MTL4CopySparseBufferMappingOperation
Groups together arguments for an operation to copy a sparse buffer mapping.
MTL4UpdateSparseBufferMappingOperation
Groups together arguments for an operation to update a sparse buffer mapping.
MTLTextureSparseTier
Enumerates the different support levels for sparse textures.
MTL4CopySparseTextureMappingOperation
Groups together arguments for an operation to copy a sparse texture mapping.
MTL4UpdateSparseTextureMappingOperation
Groups together arguments for an operation to update a sparse texture mapping.
Common resource functionality
MTLGPUAddress
A 64-bit unsigned integer type appropriate for storing GPU addresses.
MTLAllocation
A memory allocation from a Metal GPU device, such as a memory heap, texture, or data buffer.
MTLResource
An allocation of memory accessible to a GPU.
MTLResourceOptions
Optional arguments used to set the behavior of a resource.
MTLResourceUsage
Options that describe how a graphics or compute function uses an argument buffer’s resource.
MTLResourceID
See Also
Resources
Buffers
Create and manage untyped data your app uses to exchange information with its shader functions.
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
