use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
    runtime::NSObject,
};
use objc2_foundation::{CopyingHelper, NSCopying, NSObjectProtocol, NSURL};

extern_class!(
    /// A class used to indicate how an archive should be created.
    ///
    /// Availability: macOS 11.0+, iOS 14.0+
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLBinaryArchiveDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for MTLBinaryArchiveDescriptor {}
);

unsafe impl CopyingHelper for MTLBinaryArchiveDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLBinaryArchiveDescriptor {}
);

impl MTLBinaryArchiveDescriptor {
    extern_methods!(
        /// The file URL from which to open a MTLBinaryArchive, or nil to create an empty MTLBinaryArchive.
        ///
        /// Availability: macOS 11.0+, iOS 14.0+
        #[unsafe(method(url))]
        #[unsafe(method_family = none)]
        pub fn url(&self) -> Option<Retained<NSURL>>;

        /// Setter for `url`. Copied when set.
        ///
        /// Availability: macOS 11.0+, iOS 14.0+
        #[unsafe(method(setUrl:))]
        #[unsafe(method_family = none)]
        pub fn set_url(&self, url: Option<&NSURL>);
    );
}

impl MTLBinaryArchiveDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub fn new() -> Retained<Self>;
    );
}
