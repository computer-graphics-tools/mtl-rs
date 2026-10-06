use objc2::{
    extern_class, extern_conformance, extern_methods, msg_send,
    rc::{Allocated, Retained},
    runtime::NSObject,
};
use objc2_foundation::{NSNumber, NSObjectProtocol};

extern_class!(
    /// Helper object for convenient access to samples stored in an array.
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLRasterizationRateSampleArray;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLRasterizationRateSampleArray {}
);

impl MTLRasterizationRateSampleArray {
    /// Retrieves the sample value at `index`, or `0.0` if the index is out of range.
    pub fn get(
        &self,
        index: usize,
    ) -> f32 {
        let value: Retained<NSNumber> = unsafe { msg_send![self, objectAtIndexedSubscript: index] };
        value.as_f32()
    }

    /// Stores `value` as the sample at `index`.
    ///
    /// Metal keeps the sample as a single-precision floating point value.
    pub fn set(
        &self,
        index: usize,
        value: f32,
    ) {
        let value = NSNumber::new_f32(value);
        unsafe {
            let _: () = msg_send![self, setObject: &*value, atIndexedSubscript: index];
        }
    }
}

/// Methods declared on superclass `NSObject`.
impl MTLRasterizationRateSampleArray {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub fn new() -> Retained<Self>;
    );
}
