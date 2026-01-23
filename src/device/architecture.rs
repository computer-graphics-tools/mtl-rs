use objc2::{
    extern_class, extern_conformance, extern_methods, msg_send,
    rc::{Allocated, Retained},
    runtime::NSObject,
};
use objc2_foundation::{CopyingHelper, NSCopying, NSObjectProtocol, NSString};

extern_class!(
    /// Contains information about the device's architecture
    ///
    /// Apple's docs: `https://developer.apple.com/documentation/metal/mtlarchitecture?language=objc`
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLArchitecture;
);

extern_conformance!(
    unsafe impl NSCopying for MTLArchitecture {}
);

unsafe impl CopyingHelper for MTLArchitecture {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLArchitecture {}
);

/// Methods declared on superclass `NSObject`.
impl MTLArchitecture {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub fn new() -> Retained<Self>;
    );
}

#[allow(unused)]
impl MTLArchitecture {
    /// The device's architecture name.
    fn name(&self) -> String {
        let ns: Retained<NSString> = unsafe { msg_send![self, name] };
        ns.to_string()
    }
}
