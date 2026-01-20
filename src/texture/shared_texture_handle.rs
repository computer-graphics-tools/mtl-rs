use objc2::{
    extern_class, extern_conformance, extern_methods, msg_send,
    rc::Retained,
    runtime::{NSObject, ProtocolObject},
};
use objc2_foundation::{NSCoding, NSObjectProtocol, NSSecureCoding, NSString};

use crate::MTLDevice;

extern_class!(
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtlsharedtexturehandle?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLSharedTextureHandle;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLSharedTextureHandle {}
);

extern_conformance!(
    unsafe impl NSCoding for MTLSharedTextureHandle {}
);

extern_conformance!(
    unsafe impl NSSecureCoding for MTLSharedTextureHandle {}
);

impl MTLSharedTextureHandle {
    extern_methods!(
        /// The device this texture was created against.
        ///
        /// This shared texture handle can only be used with this device.
        #[unsafe(method(device))]
        #[unsafe(method_family = none)]
        pub fn device(&self) -> Retained<ProtocolObject<dyn MTLDevice>>;
    );
}

#[allow(unused)]
impl MTLSharedTextureHandle {
    fn label(&self) -> Option<String> {
        let label: Option<Retained<NSString>> = unsafe { msg_send![self, label] };
        label.map(|label| label.to_string())
    }
}
