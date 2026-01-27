use objc2::{
    extern_class, extern_conformance, extern_methods, msg_send,
    rc::{Allocated, Retained},
};
use objc2_foundation::{CopyingHelper, NSCopying, NSObject, NSObjectProtocol, NSString};

extern_class!(
    /// Groups parameters for the creation of a Metal argument table.
    ///
    /// Argument tables provide resource bindings to your Metal pipeline states.
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metal/mtl4argumenttabledescriptor?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTL4ArgumentTableDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for MTL4ArgumentTableDescriptor {}
);

unsafe impl CopyingHelper for MTL4ArgumentTableDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTL4ArgumentTableDescriptor {}
);

impl MTL4ArgumentTableDescriptor {
    extern_methods!(
        /// Determines the number of buffer-binding slots for the argument table.
        ///
        /// The maximum value of this parameter is 31.
        #[unsafe(method(maxBufferBindCount))]
        #[unsafe(method_family = none)]
        pub fn max_buffer_bind_count(&self) -> usize;

        /// Setter for [`maxBufferBindCount`][Self::maxBufferBindCount].
        #[unsafe(method(setMaxBufferBindCount:))]
        #[unsafe(method_family = none)]
        pub fn set_max_buffer_bind_count(&self, max_buffer_bind_count: usize);

        /// Determines the number of texture-binding slots for the argument table.
        ///
        /// The maximum value of this parameter is 128.
        #[unsafe(method(maxTextureBindCount))]
        #[unsafe(method_family = none)]
        pub fn max_texture_bind_count(&self) -> usize;

        /// Setter for [`maxTextureBindCount`][Self::maxTextureBindCount].
        #[unsafe(method(setMaxTextureBindCount:))]
        #[unsafe(method_family = none)]
        pub fn set_max_texture_bind_count(&self, max_texture_bind_count: usize);

        /// Determines the number of sampler state-binding slots for the argument table.
        ///
        /// The maximum value of this parameter is 16.
        #[unsafe(method(maxSamplerStateBindCount))]
        #[unsafe(method_family = none)]
        pub fn max_sampler_state_bind_count(&self) -> usize;

        /// Setter for [`maxSamplerStateBindCount`][Self::maxSamplerStateBindCount].
        #[unsafe(method(setMaxSamplerStateBindCount:))]
        #[unsafe(method_family = none)]
        pub fn set_max_sampler_state_bind_count(&self, max_sampler_state_bind_count: usize);

        /// Configures whether Metal initializes the bindings to nil values upon creation of argument table.
        ///
        /// The default value of this property is false.
        #[unsafe(method(initializeBindings))]
        #[unsafe(method_family = none)]
        pub fn initialize_bindings(&self) -> bool;

        /// Setter for [`initializeBindings`][Self::initializeBindings].
        #[unsafe(method(setInitializeBindings:))]
        #[unsafe(method_family = none)]
        pub fn set_initialize_bindings(&self, initialize_bindings: bool);

        /// Controls whether Metal should reserve memory for attribute strides in the argument table.
        ///
        /// Set this value to true if you intend to provide dynamic attribute strides when binding vertex
        /// array buffers to the argument table by calling ``MTL4ArgumentTable/setAddress:attributeStride:atIndex:``
        ///
        /// The default value of this property is false.
        #[unsafe(method(supportAttributeStrides))]
        #[unsafe(method_family = none)]
        pub fn support_attribute_strides(&self) -> bool;

        /// Setter for [`supportAttributeStrides`][Self::supportAttributeStrides].
        #[unsafe(method(setSupportAttributeStrides:))]
        #[unsafe(method_family = none)]
        pub fn set_support_attribute_strides(&self, support_attribute_strides: bool);
    );
}

/// Methods declared on superclass `NSObject`.
impl MTL4ArgumentTableDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub fn new() -> Retained<Self>;
    );
}

impl MTL4ArgumentTableDescriptor {
    /// Optional label for debug purposes.
    pub fn label(&self) -> Option<String> {
        let s: Option<Retained<NSString>> = unsafe { msg_send![self, label] };
        s.map(|v| v.to_string())
    }

    /// Setter for label.
    pub fn set_label(&self, label: Option<&str>) {
        unsafe {
            let _: () = msg_send![self, setLabel: label.map(NSString::from_str).as_deref()];
        }
    }
}
