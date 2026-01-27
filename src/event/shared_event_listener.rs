use dispatch2::DispatchQueue;
use objc2::{
    extern_class, extern_conformance, extern_methods,
    rc::{Allocated, Retained},
    runtime::NSObject,
};
use objc2_foundation::NSObjectProtocol;

extern_class!(
    /// Listener for shared event notifications from Metal.
    ///
    /// Availability: macOS 10.14+, iOS 12.0+
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLSharedEventListener;
);

unsafe impl Send for MTLSharedEventListener {}
unsafe impl Sync for MTLSharedEventListener {}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLSharedEventListener {}
);

impl MTLSharedEventListener {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        /// Designated initializer with a dispatch queue used to deliver notifications.
        #[unsafe(method(initWithDispatchQueue:))]
        #[unsafe(method_family = init)]
        pub fn init_with_dispatch_queue(
            this: Allocated<Self>,
            dispatch_queue: &DispatchQueue,
        ) -> Retained<Self>;

        /// The dispatch queue used by this listener.
        #[unsafe(method(dispatchQueue))]
        #[unsafe(method_family = none)]
        pub fn dispatch_queue(&self) -> Retained<DispatchQueue>;

        #[unsafe(method(sharedListener))]
        #[unsafe(method_family = none)]
        /// A shared instance constructed with a standard serial dispatch queue.
        /// This instance can be used for short-running notifications without QoS requirements.
        ///
        /// Availability: macOS 26.0+, iOS 26.0+
        pub fn shared_listener() -> Retained<MTLSharedEventListener>;
    );
}

/// Methods declared on superclass `NSObject`.
impl MTLSharedEventListener {
    extern_methods!(
        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub fn new() -> Retained<Self>;
    );
}
