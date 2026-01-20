use core::ptr::NonNull;

use objc2::{extern_protocol, rc::Retained, runtime::ProtocolObject};

use super::{MTLEvent, MTLSharedEventHandle, MTLSharedEventListener};

pub type SharedEventNotificationBlock =
    *mut block2::DynBlock<dyn Fn(NonNull<ProtocolObject<dyn MTLSharedEvent>>, u64)>;

extern_protocol!(
    /// Shared event that can be signaled and waited on across devices.
    ///
    /// Availability: macOS 10.14+, iOS 12.0+
    pub unsafe trait MTLSharedEvent: MTLEvent {
        /// Register a callback for when the event reaches a value.
        #[unsafe(method(notifyListener:atValue:block:))]
        #[unsafe(method_family = none)]
        unsafe fn notify_listener_at_value_block(
            &self,
            listener: &MTLSharedEventListener,
            value: u64,
            block: SharedEventNotificationBlock,
        );

        /// Convenience method for creating a shared event handle that may be passed to other processes via XPC.
        #[unsafe(method(newSharedEventHandle))]
        #[unsafe(method_family = new)]
        unsafe fn new_shared_event_handle(&self) -> Retained<MTLSharedEventHandle>;

        /// Synchronously wait for the signaled value to be >= `value`, with a timeout in milliseconds.
        #[unsafe(method(waitUntilSignaledValue:timeoutMS:))]
        #[unsafe(method_family = none)]
        unsafe fn wait_until_signaled_value_timeout_ms(
            &self,
            value: u64,
            milliseconds: u64,
        ) -> bool;

        /// Read the current signaled value.
        #[unsafe(method(signaledValue))]
        #[unsafe(method_family = none)]
        unsafe fn signaled_value(&self) -> u64;

        /// Set the event's signaled value.
        #[unsafe(method(setSignaledValue:))]
        #[unsafe(method_family = none)]
        unsafe fn set_signaled_value(&self, signaled_value: u64);
    }
);
