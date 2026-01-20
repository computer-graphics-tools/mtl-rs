use objc2::{
    extern_class, extern_conformance, extern_methods, msg_send,
    rc::{Allocated, Retained},
    runtime::{NSObject, ProtocolObject},
};
use objc2_foundation::{CopyingHelper, NSArray, NSCopying, NSObjectProtocol, NSString};

use crate::MTLFunctionStitchingFunctionNode;

extern_class!(
    /// Function graph describing a DAG used to produce a stitched function.
    ///
    /// Availability: macOS 12.0+, iOS 15.0+
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLFunctionStitchingGraph;
);

extern_conformance!(
    unsafe impl NSCopying for MTLFunctionStitchingGraph {}
);

unsafe impl CopyingHelper for MTLFunctionStitchingGraph {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLFunctionStitchingGraph {}
);

impl MTLFunctionStitchingGraph {
    extern_methods!(
        #[unsafe(method(nodes))]
        #[unsafe(method_family = none)]
        pub unsafe fn nodes(&self) -> Retained<NSArray<MTLFunctionStitchingFunctionNode>>;

        /// Setter for [`nodes`][Self::nodes].
        #[unsafe(method(setNodes:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_nodes(&self, nodes: &NSArray<MTLFunctionStitchingFunctionNode>);

        #[unsafe(method(outputNode))]
        #[unsafe(method_family = none)]
        pub unsafe fn output_node(&self) -> Option<Retained<MTLFunctionStitchingFunctionNode>>;

        /// Setter for [`output_node`][Self::output_node].
        #[unsafe(method(setOutputNode:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_output_node(
            &self,
            output_node: Option<&MTLFunctionStitchingFunctionNode>,
        );

        #[unsafe(method(attributes))]
        #[unsafe(method_family = none)]
        pub unsafe fn attributes(
            &self,
        ) -> Retained<NSArray<ProtocolObject<dyn super::MTLFunctionStitchingAttribute>>>;

        /// Setter for [`attributes`][Self::attributes].
        #[unsafe(method(setAttributes:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_attributes(
            &self,
            attributes: &NSArray<ProtocolObject<dyn super::MTLFunctionStitchingAttribute>>,
        );

        #[unsafe(method(initWithFunctionName:nodes:outputNode:attributes:))]
        #[unsafe(method_family = init)]
        pub unsafe fn init_with_function_name_nodes_output_node_attributes(
            this: Allocated<Self>,
            function_name: &NSString,
            nodes: &NSArray<MTLFunctionStitchingFunctionNode>,
            output_node: Option<&MTLFunctionStitchingFunctionNode>,
            attributes: &NSArray<ProtocolObject<dyn super::MTLFunctionStitchingAttribute>>,
        ) -> Retained<Self>;
    );
}

#[allow(unused)]
impl MTLFunctionStitchingGraph {
    /// The name of the function to call.
    fn function_name(&self) -> String {
        let s: Retained<NSString> = unsafe { msg_send![self, functionName] };
        s.to_string()
    }

    /// Setter for [`function_name`][Self::function_name].
    fn set_function_name(&self, name: &str) {
        unsafe {
            let _: () = msg_send![self, setFunctionName: &*NSString::from_str(name)];
        }
    }
}
