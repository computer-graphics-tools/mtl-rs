use objc2::{extern_class, extern_methods, msg_send, rc::Retained, runtime::NSObject};
use objc2_foundation::{NSArray, NSString};

use crate::argument::MTLStructMember;

extern_class!(
    /// Reflection for a struct type used in argument/pipeline reflection.
    ///
    /// Availability: macOS 10.11+, iOS 8.0+
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLStructType;
);

impl MTLStructType {
    extern_methods!();
}

#[allow(unused)]
pub trait MTLStructTypeExt {
    fn member_by_name(&self, name: &str) -> Option<Retained<MTLStructMember>>;
    fn members(&self) -> Box<[Retained<MTLStructMember>]>;
}

impl MTLStructTypeExt for MTLStructType {
    fn member_by_name(&self, name: &str) -> Option<Retained<MTLStructMember>> {
        let ns = NSString::from_str(name);
        unsafe { msg_send![self, memberByName: &*ns] }
    }
    fn members(&self) -> Box<[Retained<MTLStructMember>]> {
        let array: Retained<NSArray<MTLStructMember>> = unsafe { msg_send![self, members] };
        array.to_vec().into_boxed_slice()
    }
}
