# Blit passes

> [Apple Documentation](https://developer.apple.com/documentation/metal/blit-passes?language=objc)

## Overview

Your app can use a block information transfer (blit) pass to manage data within, and copy data between, buffers, textures, and other Metal resources. Start by creating a blit command encoder by calling an MTLCommandBuffer instance’s blitCommandEncoder method. Then use the MTLBlitCommandEncoder instance’s methods to encode individual commands of your blit pass.

You also have the option to customize a blit pass’s runtime behavior, such as sampling GPU hardware data. To enable these options, configure an MTLBlitPassDescriptor instance and pass it to the command buffer’s blitCommandEncoderWithDescriptor: method. For more information about sampling GPU hardware data in a blit pass, see the articles in GPU counters and counter sample buffers, including:

Sampling GPU data into counter sample buffers

Converting a GPU’s counter data into a readable format

Encoding a blit pass
MTLBlitCommandEncoder
Encodes commands that copy and modify resources for a single blit pass.
MTLBlitOption
The options that enable behavior for some blit operations.
Configuring a blit command encoder
MTLBlitPassDescriptor
A configuration you create to customize a blit command encoder, which affects the runtime behavior of the blit pass you encode with it.
MTLBlitPassSampleBufferAttachmentDescriptor
A configuration that instructs the GPU where to store counter data from the beginning and end of a blit pass.
MTLBlitPassSampleBufferAttachmentDescriptorArray
A container that stores an array of sample buffer attachments for a blit pass.
See Also
Command encoders
Render passes
Encode a render pass to draw graphics into an image.
Compute passes
Encode a compute pass that runs computations in parallel on a thread grid, processing and manipulating Metal resource data on multiple cores of a GPU.
Machine learning passes
Add machine learning model inference to your Metal app’s GPU workflow.
Indirect command encoding
Store draw commands in Metal buffers and run them at a later time on the GPU, either once or repeatedly.
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
