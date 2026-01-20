use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
    runtime::NSObject,
};
use objc2_foundation::{CopyingHelper, NSCopying, NSObjectProtocol};

use crate::MTLPixelFormat;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtltilerenderpipelinecolorattachmentdescriptor?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLTileRenderPipelineColorAttachmentDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for MTLTileRenderPipelineColorAttachmentDescriptor {}
);

unsafe impl CopyingHelper for MTLTileRenderPipelineColorAttachmentDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLTileRenderPipelineColorAttachmentDescriptor {}
);

impl MTLTileRenderPipelineColorAttachmentDescriptor {
    extern_methods!(
        /// Pixel format.  Defaults to MTLPixelFormatInvalid
        #[unsafe(method(pixelFormat))]
        #[unsafe(method_family = none)]
        pub unsafe fn pixel_format(&self) -> MTLPixelFormat;

        /// Setter for [`pixelFormat`][Self::pixelFormat].
        #[unsafe(method(setPixelFormat:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_pixel_format(&self, pixel_format: MTLPixelFormat);
    );
}

/// Methods declared on superclass `NSObject`.
impl MTLTileRenderPipelineColorAttachmentDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}
