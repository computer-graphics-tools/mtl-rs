use objc2::{
    extern_class, extern_conformance, extern_methods, msg_send,
    rc::{Allocated, Retained},
    runtime::NSObject,
};
use objc2_foundation::{NSCoding, NSObjectProtocol, NSSecureCoding, NSString};

extern_class!(
    /// Handle that can be sent across processes/devices for a shared event.
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLSharedEventHandle;
);

unsafe impl Send for MTLSharedEventHandle {}
unsafe impl Sync for MTLSharedEventHandle {}

extern_conformance!(
    unsafe impl NSCoding for MTLSharedEventHandle {}
);

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLSharedEventHandle {}
);

extern_conformance!(
    unsafe impl NSSecureCoding for MTLSharedEventHandle {}
);

impl MTLSharedEventHandle {
    extern_methods!();

    pub fn label(&self) -> Option<String> {
        let label: Option<Retained<NSString>> = unsafe { msg_send![self, label] };
        label.map(|label| label.to_string())
    }
}

/// Methods declared on superclass `NSObject`.
impl MTLSharedEventHandle {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub fn new() -> Retained<Self>;
    );
}
