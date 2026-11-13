use objc2::{Message, extern_protocol, msg_send, rc::Retained, runtime::ProtocolObject};
use objc2_foundation::{NSArray, NSDictionary, NSObjectProtocol, NSString};

use crate::{
    MTLArgument, MTLArgumentEncoder, MTLAttribute, MTLDevice, MTLFunctionConstant, MTLFunctionOptions, MTLFunctionType,
    MTLPatchType, MTLVertexAttribute,
};

extern_protocol!(
    /// A Metal shader function used to create a render or compute pipeline.
    ///
    /// Availability: macOS 10.11+, iOS 8.0+
    ///
    /// # Safety
    ///
    /// Implementors must be Objective-C objects that conform to the
    /// `MTLFunction` protocol.
    ///
    /// # Thread safety
    ///
    /// [Apple's declaration](https://developer.apple.com/documentation/metal/mtlfunction):
    ///
    /// > `protocol MTLFunction : NSObjectProtocol, Sendable`
    ///
    /// The `Send` and `Sync` bounds rely on this guarantee.
    pub unsafe trait MTLFunction: NSObjectProtocol + Send + Sync {
        /// The device that created the function.
        #[unsafe(method(device))]
        #[unsafe(method_family = none)]
        fn device(&self) -> Retained<ProtocolObject<dyn MTLDevice>>;

        /// The function's shader stage.
        #[unsafe(method(functionType))]
        #[unsafe(method_family = none)]
        fn function_type(&self) -> MTLFunctionType;

        /// The tessellation patch type, or `MTLPatchType::None` when the
        /// function isn't a post-tessellation vertex function.
        #[unsafe(method(patchType))]
        #[unsafe(method_family = none)]
        fn patch_type(&self) -> MTLPatchType;

        /// The number of tessellation patch control points, or `-1` when the
        /// shader doesn't specify one.
        #[unsafe(method(patchControlPointCount))]
        #[unsafe(method_family = none)]
        fn patch_control_point_count(&self) -> isize;

        /// Creates an argument encoder for the argument buffer at `buffer_index`.
        #[unsafe(method(newArgumentEncoderWithBufferIndex:))]
        #[unsafe(method_family = new)]
        fn new_argument_encoder_with_buffer_index(
            &self,
            buffer_index: usize,
        ) -> Retained<ProtocolObject<dyn MTLArgumentEncoder>>;

        /// Creates an argument encoder and optionally returns deprecated
        /// `MTLArgument` reflection information.
        ///
        /// Deprecated on macOS 13.0 and iOS 16.0. Use
        /// `MTLDevice::new_argument_encoder_with_buffer_binding` instead.
        ///
        #[deprecated(note = "use MTLDevice::new_argument_encoder_with_buffer_binding instead")]
        #[unsafe(method(newArgumentEncoderWithBufferIndex:reflection:))]
        #[unsafe(method_family = new)]
        fn new_argument_encoder_with_buffer_index_reflection(
            &self,
            buffer_index: usize,
            reflection: Option<&mut Option<Retained<MTLArgument>>>,
        ) -> Retained<ProtocolObject<dyn MTLArgumentEncoder>>;

        /// The options used to create the function.
        #[unsafe(method(options))]
        #[unsafe(method_family = none)]
        fn options(&self) -> MTLFunctionOptions;
    }
);

/// Rust-native accessors for values that Metal represents with Foundation
/// collection and string types.
pub trait MTLFunctionExt: MTLFunction + Message {
    /// A string that identifies the function.
    fn label(&self) -> Option<String>;

    /// Sets the string that identifies the function.
    fn set_label(
        &self,
        label: Option<&str>,
    );

    /// Reflection information for the function's vertex attributes.
    fn vertex_attributes(&self) -> Option<Box<[Retained<MTLVertexAttribute>]>>;

    /// Reflection information for the function's stage-input attributes.
    fn stage_input_attributes(&self) -> Option<Box<[Retained<MTLAttribute>]>>;

    /// The function's name in the Metal shading language.
    fn name(&self) -> String;

    /// Function-constant reflection information keyed by constant name.
    fn function_constants_dictionary(&self) -> Box<[(String, Retained<MTLFunctionConstant>)]>;
}

impl<T> MTLFunctionExt for T
where
    T: MTLFunction + Message + ?Sized,
{
    fn label(&self) -> Option<String> {
        let label: Option<Retained<NSString>> = unsafe { msg_send![self, label] };
        label.map(|label| label.to_string())
    }

    fn set_label(
        &self,
        label: Option<&str>,
    ) {
        unsafe {
            let _: () = msg_send![self, setLabel: label.map(NSString::from_str).as_deref()];
        }
    }

    fn vertex_attributes(&self) -> Option<Box<[Retained<MTLVertexAttribute>]>> {
        let attributes: Option<Retained<NSArray<MTLVertexAttribute>>> = unsafe { msg_send![self, vertexAttributes] };
        attributes.map(|attributes| attributes.to_vec().into_boxed_slice())
    }

    fn stage_input_attributes(&self) -> Option<Box<[Retained<MTLAttribute>]>> {
        let attributes: Option<Retained<NSArray<MTLAttribute>>> = unsafe { msg_send![self, stageInputAttributes] };
        attributes.map(|attributes| attributes.to_vec().into_boxed_slice())
    }

    fn name(&self) -> String {
        let name: Retained<NSString> = unsafe { msg_send![self, name] };
        name.to_string()
    }

    fn function_constants_dictionary(&self) -> Box<[(String, Retained<MTLFunctionConstant>)]> {
        let constants: Retained<NSDictionary<NSString, MTLFunctionConstant>> =
            unsafe { msg_send![self, functionConstantsDictionary] };
        let (names, values) = constants.to_vecs();
        names
            .into_iter()
            .zip(values)
            .map(|(name, value)| (name.to_string(), value))
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }
}
