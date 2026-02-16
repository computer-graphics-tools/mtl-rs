# Ray tracing with acceleration structures

> [Apple Documentation](https://developer.apple.com/documentation/metal/ray-tracing-with-acceleration-structures?language=objc)

## Overview

Ray tracing can improve your content’s realism by more accurately modeling the behavior of light than traditional rendering. You can also use ray tracing to implement similar techniques that rely on line-of-sight, such as sound obstruction or visually based AI functions.

To apply ray tracing in your app:

Create acceleration structures that represent objects in a scene.

Define a ray’s behavior when it collides into parts of an acceleration structure by creating either intersectors or intersection queries.

Generate rays into the scene from a new or existing shader.

An intersector uses a table of your intersection functions that define the custom behavior for each intersection type. An intersection query returns to your calling function to handle the custom behavior for all intersection types.

Intersectors work with compute kernels on all GPUs, and with render shaders only on Apple silicon GPUs. Alternatively, your app can use intersection queries on non-Apple GPUs, or for porting code from other graphics APIs.

Ray tracing samples
Accelerating ray tracing using Metal
Implement ray-traced rendering using GPU-based parallel processing.
Control the ray tracing process using intersection queries
Explicitly enumerate a ray’s intersections with acceleration structures by creating an intersection query object.
Rendering reflections in real time using ray tracing
Implement realistic real-time lighting by dynamically generating reflection maps by encoding a ray-tracing compute pass.
Rendering a curve primitive in a ray tracing scene
Implement ray traced rendering using GPU-based parallel processing.
Acceleration structures
Improving ray-tracing data access using per-primitive data
Simplify data access and improve GPU utilization by storing custom primitive data directly in the acceleration structure.
MTLAccelerationStructure
A collection of model data for GPU-accelerated intersection of rays with the model.
MTL4AccelerationStructureDescriptor
Base class for Metal 4 acceleration structure descriptors.
MTLAccelerationStructureDescriptor
A base class for classes that define the configuration for a new acceleration structure.
MTL4PrimitiveAccelerationStructureDescriptor
Descriptor for a primitive acceleration structure that directly references geometric shapes, such as triangles and bounding boxes.
MTLPrimitiveAccelerationStructureDescriptor
A description of an acceleration structure that contains geometry primitives.
MTL4InstanceAccelerationStructureDescriptor
Descriptor for an instance acceleration structure.
MTLInstanceAccelerationStructureDescriptor
A description of an acceleration structure that derives from instances of primitive acceleration structures.
MTLAccelerationStructureCommandEncoder
Encodes commands that build and refit acceleration structures for a single pass.
MTLAccelerationStructureUsage
Options that affect how Metal builds an acceleration structure and the behavior of that acceleration structure.
MTLAccelerationStructureRefitOptions
Acceleration structures passes
MTLAccelerationStructurePassDescriptor
MTLAccelerationStructurePassSampleBufferAttachmentDescriptor
MTLAccelerationStructurePassSampleBufferAttachmentDescriptorArray
Geometry descriptors
MTL4AccelerationStructureGeometryDescriptor
Base class for all Metal 4 acceleration structure geometry descriptors.
MTLAccelerationStructureGeometryDescriptor
A base class for descriptors that contain geometry data to convert into a ray-tracing acceleration structure.
MTL4AccelerationStructureTriangleGeometryDescriptor
Describes triangle geometry suitable for ray tracing.
MTLAccelerationStructureTriangleGeometryDescriptor
A description of a list of triangle primitives to turn into an acceleration structure.
MTL4AccelerationStructureCurveGeometryDescriptor
Describes curve geometry suitable for ray tracing.
MTLAccelerationStructureCurveGeometryDescriptor
A descriptor you configure with curve geometry for building acceleration structures.
MTLCurveType
MTLCurveBasis
MTLCurveEndCaps
MTL4AccelerationStructureBoundingBoxGeometryDescriptor
Describes bounding-box geometry suitable for ray tracing.
MTLAccelerationStructureBoundingBoxGeometryDescriptor
A description of a list of bounding boxes to turn into an acceleration structure.
Motion geometry descriptors
MTL4AccelerationStructureMotionTriangleGeometryDescriptor
Describes motion triangle geometry, suitable for motion ray tracing.
MTLAccelerationStructureMotionTriangleGeometryDescriptor
A description of a list of triangle primitives, as motion keyframe data, to turn into an acceleration structure.
MTL4AccelerationStructureMotionCurveGeometryDescriptor
Describes motion curve geometry, suitable for motion ray tracing.
MTLAccelerationStructureMotionCurveGeometryDescriptor
MTL4AccelerationStructureMotionBoundingBoxGeometryDescriptor
Describes motion bounding box geometry, suitable for motion ray tracing.
MTLAccelerationStructureMotionBoundingBoxGeometryDescriptor
A description of a list of bounding boxes, as motion keyframe data, to turn into an acceleration structure.
MTLMotionKeyframeData
Geometry data for a specific keyframe to use in a moving instance.
Instance descriptors
MTLAccelerationStructureInstanceDescriptor
A description of an instance in an instanced geometry acceleration structure.
MTLAccelerationStructureUserIDInstanceDescriptor
A description of an instance in an instanced geometry acceleration structure, with the instance including a user identifier for the instance.
MTLAccelerationStructureMotionInstanceDescriptor
A description of an instance in an instanced geometry acceleration structure, with the instance including a user identifier and motion data for the instance.
MTLAccelerationStructureInstanceOptions
Options for adjusting the behavior of an instanced acceleration structure.
MTL4IndirectInstanceAccelerationStructureDescriptor
Descriptor for an “indirect” instance acceleration structure that allows providing the instance count and motion transform count indirectly, through buffer references.
MTLIndirectInstanceAccelerationStructureDescriptor
A description of an acceleration structure that Metal derives from instances of primitive acceleration structures that the GPU can populate.
MTLIndirectAccelerationStructureInstanceDescriptor
A description of an instance in an instanced geometry acceleration structure that the GPU can populate.
MTLIndirectAccelerationStructureMotionInstanceDescriptor
A description of an instance in an acceleration structure that the GPU can populate, with motion data for the instance.
Intersection function tables
MTLIntersectionFunctionTable
A table of intersection functions that Metal calls to perform ray-tracing intersection tests.
MTLIntersectionFunctionTableDescriptor
A specification of how to create an intersection function table.
MTLIntersectionFunctionDescriptor
A description of an intersection function that performs an intersection test.
MTLIntersectionFunctionSignature
Constants for specifying different types of custom intersection functions.
MTLIntersectionFunctionBufferArguments
MTLAxisAlignedBoundingBox
The bounds for an axis-aligned bounding box.
MTLPackedFloat3
A structure that contains three 32-bit floating-point values with no additional padding.
MTLPackedFloat4x3
A structure that contains the top three rows of a 4x4 matrix of 32-bit floating-point values, in column-major order.
MTLPackedFloat3Make
Returns a new packed vector with three floating-point values.
MTL4BufferRange
MTL4BufferRangeMake
See Also
Command encoders
Render passes
Encode a render pass to draw graphics into an image.
Compute passes
Encode a compute pass that runs computations in parallel on a thread grid, processing and manipulating Metal resource data on multiple cores of a GPU.
Machine-learning passes
Add machine-learning model inference to your Metal app’s GPU workflow.
Blit passes
Encode a block information transfer pass to adjust and copy data to and from GPU resources, such as buffers and textures.
Indirect command encoding
Store draw commands in Metal buffers and run them at a later time on the GPU, either once or repeatedly.
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
