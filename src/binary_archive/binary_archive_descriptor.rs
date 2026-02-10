use std::path::{Path, PathBuf};

use objc2::{
    extern_class, extern_conformance, extern_methods, msg_send,
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
    );

    /// The file path from which to open a `MTLBinaryArchive`, or `None` to create an empty archive.
    ///
    /// Availability: macOS 11.0+, iOS 14.0+
    pub fn path(&self) -> Option<PathBuf> {
        let url: Option<Retained<NSURL>> = unsafe { msg_send![self, url] };
        url.and_then(|url| url.to_file_path())
    }

    /// Setter for `path`.
    ///
    /// Availability: macOS 11.0+, iOS 14.0+
    pub fn set_path(&self, path: Option<&Path>) {
        let url = path.and_then(NSURL::from_file_path);
        unsafe {
            let _: () = msg_send![self, setUrl: url.as_deref()];
        }
    }
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
