use objc2::{
    extern_class, extern_conformance, extern_methods, msg_send,
    rc::{Allocated, Retained},
};
use objc2_foundation::{CopyingHelper, NSCopying, NSObject, NSObjectProtocol, NSString};

use crate::*;

extern_class!(
    /// Represents options to configure a commit operation on a command queue.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metal/mtl4commitoptions?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTL4CommitOptions;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for MTL4CommitOptions {}
);

impl MTL4CommitOptions {
    extern_methods!(
        /// Registers a commit feedback handler that Metal calls with feedback data when available.
        ///
        /// # Safety
        ///
        /// `block` must be a valid pointer.
        #[unsafe(method(addFeedbackHandler:))]
        #[unsafe(method_family = none)]
        pub fn add_feedback_handler(&self, block: MTL4CommitFeedbackHandler);
    );
}

/// Methods declared on superclass `NSObject`.
impl MTL4CommitOptions {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub fn new() -> Retained<Self>;
    );
}

extern_class!(
    /// Groups together parameters for the creation of a new command queue.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metal/mtl4commandqueuedescriptor?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTL4CommandQueueDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for MTL4CommandQueueDescriptor {}
);

unsafe impl CopyingHelper for MTL4CommandQueueDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTL4CommandQueueDescriptor {}
);

impl MTL4CommandQueueDescriptor {
    extern_methods!(
        /// Assigns an optional label to the command queue instance for debugging purposes.
        #[unsafe(method(label))]
        #[unsafe(method_family = none)]
        pub fn label(&self) -> Option<Retained<NSString>>;

        /// Setter for [`label`][Self::label].
        ///
        /// This is [copied][objc2_foundation::NSCopying::copy] when set.
        #[unsafe(method(setLabel:))]
        #[unsafe(method_family = none)]
        pub fn set_label(&self, label: Option<&NSString>);

        /// Assigns a dispatch queue to which Metal submits feedback notification blocks.
        ///
        /// # Safety
        ///
        /// This is not retained internally, you must ensure the object is still alive.
        #[unsafe(method(feedbackQueue))]
        #[unsafe(method_family = none)]
        pub fn feedback_queue(&self) -> Option<Retained<dispatch2::DispatchQueue>>;

        /// Setter for [`feedbackQueue`][Self::feedbackQueue].
        ///
        /// # Safety
        ///
        /// This is unretained, you must ensure the object is kept alive while in use.
        #[unsafe(method(setFeedbackQueue:))]
        #[unsafe(method_family = none)]
        pub fn set_feedback_queue(&self, feedback_queue: Option<&dispatch2::DispatchQueue>);
    );
}

/// Methods declared on superclass `NSObject`.
impl MTL4CommandQueueDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub fn new() -> Retained<Self>;
    );
}

impl MTL4CommandQueueDescriptor {
    /// Optional label for debug purposes.
    pub fn label_str(&self) -> Option<String> {
        let s: Option<Retained<NSString>> = unsafe { msg_send![self, label] };
        s.map(|v| v.to_string())
    }

    /// Setter for label.
    pub fn set_label_str(&self, label: Option<&str>) {
        unsafe {
            let _: () = msg_send![self, setLabel: label.map(NSString::from_str).as_deref()];
        }
    }
}
