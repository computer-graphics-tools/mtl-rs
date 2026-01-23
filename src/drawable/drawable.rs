use objc2::{Message, extern_protocol, msg_send};
use objc2_foundation::NSObjectProtocol;

use crate::MTLDrawablePresentedHandler;

extern_protocol!(
    /// All "drawable" objects (such as those coming from CAMetalLayer) are expected to conform to this protocol.
    ///
    /// Availability: macOS 10.11+, iOS 8.0+
    pub unsafe trait MTLDrawable: NSObjectProtocol {
        /// Present this drawable immediately.
        #[unsafe(method(present))]
        #[unsafe(method_family = none)]
        fn present(&self);

        /// Present this drawable at a specific host time.
        ///
        /// # Safety
        ///
        /// The `presentation_time` must be a valid host time value.
        #[unsafe(method(presentAtTime:))]
        #[unsafe(method_family = none)]
        fn present_at_time(&self, presentation_time: f64);

        /// Present this drawable while setting a minimum duration in seconds
        /// before allowing this drawable to appear on the display.
        ///
        /// # Safety
        ///
        /// The `duration` must be a non-negative duration in seconds.
        ///
        /// Availability: macOS 10.15.4+, iOS 10.3+, Mac Catalyst 13.4+
        #[unsafe(method(presentAfterMinimumDuration:))]
        #[unsafe(method_family = none)]
        fn present_after_minimum_duration(&self, duration: f64);

        /// The host time that this drawable was presented on screen.
        /// Returns 0 if a frame has not been presented or has been skipped.
        ///
        /// Availability: macOS 10.15.4+, iOS 10.3+, Mac Catalyst 13.4+
        #[unsafe(method(presentedTime))]
        #[unsafe(method_family = none)]
        fn presented_time(&self) -> f64;

        /// The monotonically incremented ID for all drawable objects created
        /// from the same CAMetalLayer object. The value starts from 0.
        ///
        /// Availability: macOS 10.15.4+, iOS 10.3+, Mac Catalyst 13.4+
        #[unsafe(method(drawableID))]
        #[unsafe(method_family = none)]
        fn drawable_id(&self) -> usize;
    }
);

#[allow(unused)]
pub trait MTLDrawableExt: MTLDrawable + Message {
    fn add_presented_handler(&self, handler: &MTLDrawablePresentedHandler);
}

impl<T> MTLDrawableExt for T
where
    T: MTLDrawable + Message,
{
    fn add_presented_handler(&self, handler: &MTLDrawablePresentedHandler) {
        unsafe {
            let _: () = msg_send![self, addPresentedHandler: &**handler];
        }
    }
}
