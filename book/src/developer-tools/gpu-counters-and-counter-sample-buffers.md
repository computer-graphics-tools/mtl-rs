# GPU counters and counter sample buffers

> [Apple Documentation](https://developer.apple.com/documentation/metal/gpu-counters-and-counter-sample-buffers?language=objc)

## Overview

A GPU counter (MTLCounter) is typically a hardware feature that tracks a specific performance metric, such as timestamps before and after an important rendering stage. A counter set (MTLCounterSet) is a collection of related counters. A counter sample buffer (MTLCounterSampleBuffer) represents the memory where a GPU device stores the data for a specific counter set.

You can retrieve and inspect data from a GPU’s counter set with the following steps:

Inspect which GPU counter sets a GPU device supports (see Confirming which counters and counter sets a GPU supports).

Make a counter sample buffer to store the data (see Creating a counter sample buffer to store a GPU’s counter data during a pass).

Instruct the GPU to save the counter set data to the buffer during a pass or an immediate mode command (see Sampling GPU data into counter sample buffers).

Transform the counter set data into a standard type (see Converting a GPU’s counter data into a readable format).

If you’re sampling data from a timestamp counter set (MTLCommonCounterSetTimestamp), you may need to convert the timestamps from the GPU’s clock to the CPU’s clock. See Converting GPU timestamps into CPU time for more information.

Counters and counter sets
Confirming which counters and counter sets a GPU supports
Check whether a GPU produces the runtime performance data you want to sample.
MTLCounterSet
A collection of individual counters a GPU device supports for a counter set.
MTLCommonCounterSet
The name of a specific counter set that a GPU device can support.
MTLCounter
An individual counter a GPU device lists within one of its counter sets.
MTLCommonCounter
The name of a specific counter that can appear in a GPU device’s counter sets.
Counter sample buffers
Creating a counter sample buffer to store a GPU’s counter data during a pass
Make a buffer that provides a place for a GPU to save its runtime performance metrics as it runs a pass.
MTLCounterSampleBufferDescriptor
A group of properties that configures the counter sample buffers you create with it.
MTLCounterSampleBuffer
A specialized memory buffer that stores a GPU’s counter set data.
Sampling GPU data into counter sample buffers
Retrieve a GPU’s counter data at a time the GPU supports.
MTLCounterDontSample
A sentinel value that instructs an encoder to skip sampling a counter as the GPU runs the encoder’s pass.
Counter sample data output
Converting a GPU’s counter data into a readable format
Inspect and use the data within a GPU’s counter sample buffer by resolving it into a standard format.
MTLCounterResultTimestamp
The data structure for storing the data you resolve from a timestamp counter set.
MTLCounterResultStatistic
The data structure for storing the data you resolve from a statistic counter set.
MTLCounterResultStageUtilization
The data structure for storing the data you resolve from a stage-utilization counter set.
MTLCounterErrorValue
A sentinel value for an entry in a counter sample buffer that indicates the entry’s data is invalid.
Timestamp data
Converting GPU timestamps into CPU time
Correlate GPU events with CPU timelines by calculating the CPU time equivalents for GPU timestamps.
MTLTimestamp
The number of nanoseconds for a point in absolute time or Mach absolute time.
Counter sample buffer errors
MTLCounterSampleBufferError
The underlying error code type that indicates why a GPU driver can’t create a counter sample buffer.
MTLCounterErrorDomain
The domain for Metal counter sample buffer errors.
See Also
Configure alternative render paths in your Metal app to enable running your app in Simulator.
Capturing Metal commands programmatically
Invoke a Metal frame capture from your app, then save the resulting GPU trace to a file or view it in Xcode.
Logging shader debug messages
Print debugging messages that a shader generates using shader logging.
Prototype and test your Metal apps in Simulator.
Improving your game’s graphics performance and settings
Fix performance glitches and develop default settings for smooth experiences on Apple platforms using the powerful suite of Metal development tools.
Metal debugging types
Create capture managers and capture scopes, and review a GPU device’s log after it runs a command buffer.
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
