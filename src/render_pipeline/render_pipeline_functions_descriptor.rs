use objc2::{
    extern_class, extern_conformance, extern_methods, msg_send,
    rc::{Allocated, Retained},
    runtime::{NSObject, ProtocolObject},
};
use objc2_foundation::{CopyingHelper, NSArray, NSCopying, NSObjectProtocol};

use crate::MTLFunction;

extern_class!(
    /// Additional binary functions for incrementally creating a render pipeline state.
    ///
    /// Availability: macOS 12.0+, iOS 15.0+
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLRenderPipelineFunctionsDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for MTLRenderPipelineFunctionsDescriptor {}
);

unsafe impl CopyingHelper for MTLRenderPipelineFunctionsDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLRenderPipelineFunctionsDescriptor {}
);

impl MTLRenderPipelineFunctionsDescriptor {
    /// Additional binary functions available to the vertex function.
    pub fn vertex_additional_binary_functions(&self) -> Option<Box<[Retained<ProtocolObject<dyn MTLFunction>>]>> {
        let functions: Option<Retained<NSArray<ProtocolObject<dyn MTLFunction>>>> =
            unsafe { msg_send![self, vertexAdditionalBinaryFunctions] };
        functions.map(|functions| functions.to_vec().into_boxed_slice())
    }

    /// Setter for [`vertex_additional_binary_functions`][Self::vertex_additional_binary_functions].
    pub fn set_vertex_additional_binary_functions(
        &self,
        functions: Option<&[&ProtocolObject<dyn MTLFunction>]>,
    ) {
        let functions = functions.map(NSArray::from_slice);
        unsafe {
            let _: () = msg_send![self, setVertexAdditionalBinaryFunctions: functions.as_deref()];
        }
    }

    /// Additional binary functions available to the fragment function.
    pub fn fragment_additional_binary_functions(&self) -> Option<Box<[Retained<ProtocolObject<dyn MTLFunction>>]>> {
        let functions: Option<Retained<NSArray<ProtocolObject<dyn MTLFunction>>>> =
            unsafe { msg_send![self, fragmentAdditionalBinaryFunctions] };
        functions.map(|functions| functions.to_vec().into_boxed_slice())
    }

    /// Setter for [`fragment_additional_binary_functions`][Self::fragment_additional_binary_functions].
    pub fn set_fragment_additional_binary_functions(
        &self,
        functions: Option<&[&ProtocolObject<dyn MTLFunction>]>,
    ) {
        let functions = functions.map(NSArray::from_slice);
        unsafe {
            let _: () = msg_send![self, setFragmentAdditionalBinaryFunctions: functions.as_deref()];
        }
    }

    /// Additional binary functions available to the tile function.
    pub fn tile_additional_binary_functions(&self) -> Option<Box<[Retained<ProtocolObject<dyn MTLFunction>>]>> {
        let functions: Option<Retained<NSArray<ProtocolObject<dyn MTLFunction>>>> =
            unsafe { msg_send![self, tileAdditionalBinaryFunctions] };
        functions.map(|functions| functions.to_vec().into_boxed_slice())
    }

    /// Setter for [`tile_additional_binary_functions`][Self::tile_additional_binary_functions].
    pub fn set_tile_additional_binary_functions(
        &self,
        functions: Option<&[&ProtocolObject<dyn MTLFunction>]>,
    ) {
        let functions = functions.map(NSArray::from_slice);
        unsafe {
            let _: () = msg_send![self, setTileAdditionalBinaryFunctions: functions.as_deref()];
        }
    }
}

impl MTLRenderPipelineFunctionsDescriptor {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub fn new() -> Retained<Self>;
    );
}
