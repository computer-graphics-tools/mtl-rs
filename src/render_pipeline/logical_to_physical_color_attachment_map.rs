use objc2::{extern_class, extern_conformance, extern_methods, runtime::NSObject};
use objc2_foundation::NSObjectProtocol;

extern_class!(
    /// Logical to physical color attachment mapping helper.
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLLogicalToPhysicalColorAttachmentMap;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLLogicalToPhysicalColorAttachmentMap {}
);

impl MTLLogicalToPhysicalColorAttachmentMap {
    extern_methods!(
        #[unsafe(method(setPhysicalIndex:forLogicalIndex:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_physical_for_logical(&self, physical_index: usize, logical_index: usize);

        #[unsafe(method(getPhysicalIndexForLogicalIndex:))]
        #[unsafe(method_family = none)]
        pub unsafe fn physical_for_logical(&self, logical_index: usize) -> usize;

        #[unsafe(method(reset))]
        #[unsafe(method_family = none)]
        pub unsafe fn reset(&self);
    );
}
