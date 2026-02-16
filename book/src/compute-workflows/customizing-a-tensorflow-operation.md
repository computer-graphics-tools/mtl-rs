# Customizing a TensorFlow operation

> [Apple Documentation](https://developer.apple.com/documentation/metal/customizing-a-tensorflow-operation?language=objc)

## Run the Example

```bash
cargo run --example customizing_a_tensorflow_operation
```

## Shader Files

- `hash_encoder/hash_encoder_kernel.metal`

## Overview

Note

This sample code project is associated with WWDC22 session 10063: Accelerate machine learning with Metal.

Configure the sample code

Follow the instructions in Getting started with tensorflow-metal.

Install ffmpeg using brew.

brew install ffmpeg


Install the required Python packages.

pip install -r requirements.txt


Use make to build the custom operation with Xcode.

cd hash_encoder
make
cd ..


Run the sample.

python tiny_nerf_hash.py


View the resutls in the result_nerf_hash folder.

To compare the performance benefits provided by this sample, you can run the original NeRF sample code included with the project. View the resutls in the result_nerf_mlp folder.

python tiny_nerf_mlp.py


Note

The sample uses low-resolution (100x100) images by default. You can alternatively use a high-resolution version of the data to produce a clearer rendering.

See Also
Compute workflows
Performing calculations on a GPU
Use Metal to find GPUs and perform calculations on them.
Selecting device objects for compute processing
Switch dynamically between multiple GPUs to efficiently execute a compute-intensive simulation.
Customizing a PyTorch operation
Implement a custom operation in PyTorch that uses Metal kernels to improve performance.
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
