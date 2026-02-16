# Memory heaps

> [Apple Documentation](https://developer.apple.com/documentation/metal/memory-heaps?language=objc)

## Overview

Use an MTLHeap to quickly create and destroy GPU resources. Heaps can also help your apps save memory by aliasing portions of it in multiple places.

Create a heap by calling an MTLDevice instance’s newHeapWithDescriptor: method.

Note

Metal only synchronizes resources that you create from a Metal heap and that have the hazardTrackingMode property set to MTLHazardTrackingModeTracked.

Resource memory allocation and management
Using argument buffers with resource heaps
Reduce CPU overhead by using arrays inside argument buffers and combining them with resource heaps.
Implementing a multistage image filter using heaps and events
Use events to synchronize access to resources allocated on a heap.
Implementing a multistage image filter using heaps and fences
Use fences to synchronize access to resources allocated on a heap.
MTLHeap
A memory pool from which you can suballocate resources.
MTLHeapDescriptor
A configuration that customizes the behavior for a Metal memory heap.
MTLHeapType
The options you use to choose the heap type.
MTLSizeAndAlign
The size and alignment of a resource, in bytes.
See Also
Resources
Resource fundamentals
Control the common attributes of all Metal memory resources, including buffers and textures, and how to configure their underlying memory.
Buffers
Create and manage untyped data your app uses to exchange information with its shader functions.
Textures
Create and manage typed data your app uses to exchange information with its shader functions.
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
