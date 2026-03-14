use objc2::{
    extern_class, extern_conformance, extern_methods, msg_send,
    rc::{Allocated, DefaultRetained, Retained},
    runtime::{NSObject, ProtocolObject},
};
use objc2_foundation::{CopyingHelper, NSArray, NSCopying, NSDictionary, NSObjectProtocol, NSString};

use crate::library::MTLFunction;

extern_class!(
    /// Functions to be linked into a pipeline.
    ///
    /// Availability: macOS 11.0+, iOS 14.0+
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLLinkedFunctions;
);

extern_conformance!(
    unsafe impl NSCopying for MTLLinkedFunctions {}
);

unsafe impl CopyingHelper for MTLLinkedFunctions {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLLinkedFunctions {}
);

impl MTLLinkedFunctions {
    extern_methods!(
        /// Create an autoreleased LinkedFunctions object.
        ///
        /// Availability: macOS 11.0+, iOS 14.0+
        #[unsafe(method(linkedFunctions))]
        #[unsafe(method_family = none)]
        pub fn linked_functions() -> Retained<MTLLinkedFunctions>;
    );

    /// Array of functions to be AIR linked.
    pub fn functions(&self) -> Option<Box<[Retained<ProtocolObject<dyn MTLFunction>>]>> {
        let functions: Option<Retained<NSArray<ProtocolObject<dyn MTLFunction>>>> =
            unsafe { msg_send![self, functions] };
        functions.map(|functions| functions.to_vec().into_boxed_slice())
    }

    pub fn set_functions(
        &self,
        functions: Option<&[&ProtocolObject<dyn MTLFunction>]>,
    ) {
        let functions = functions.map(NSArray::from_slice);
        unsafe {
            let _: () = msg_send![self, setFunctions: functions.as_deref()];
        }
    }

    /// Array of functions compiled to binary to be linked.
    ///
    /// Availability: macOS 11.0+, iOS 14.0+, tvOS 16.0+
    pub fn binary_functions(&self) -> Option<Box<[Retained<ProtocolObject<dyn MTLFunction>>]>> {
        let binary_functions: Option<Retained<NSArray<ProtocolObject<dyn MTLFunction>>>> =
            unsafe { msg_send![self, binaryFunctions] };
        binary_functions.map(|functions| functions.to_vec().into_boxed_slice())
    }

    pub fn set_binary_functions(
        &self,
        binary_functions: Option<&[&ProtocolObject<dyn MTLFunction>]>,
    ) {
        let binary_functions = binary_functions.map(NSArray::from_slice);
        unsafe {
            let _: () = msg_send![self, setBinaryFunctions: binary_functions.as_deref()];
        }
    }

    /// Groups of functions, keyed by callsite name.
    pub fn groups(&self) -> Option<Box<[(String, Box<[Retained<ProtocolObject<dyn MTLFunction>>]>)]>> {
        let groups: Option<Retained<NSDictionary<NSString, NSArray<ProtocolObject<dyn MTLFunction>>>>> =
            unsafe { msg_send![self, groups] };
        groups.map(|groups| {
            let (group_names, group_functions) = groups.to_vecs();
            group_names
                .into_iter()
                .zip(group_functions)
                .map(|(name, functions)| (name.to_string(), functions.to_vec().into_boxed_slice()))
                .collect::<Vec<_>>()
                .into_boxed_slice()
        })
    }

    pub fn set_groups(
        &self,
        groups: Option<&[(&str, &[&ProtocolObject<dyn MTLFunction>])]>,
    ) {
        let groups = groups.map(|groups| {
            let group_names: Vec<Retained<NSString>> =
                groups.iter().map(|(name, _)| NSString::from_str(name)).collect();
            let group_name_refs: Vec<&NSString> = group_names.iter().map(|name| &**name).collect();
            let group_functions: Vec<Retained<NSArray<ProtocolObject<dyn MTLFunction>>>> =
                groups.iter().map(|(_, functions)| NSArray::from_slice(functions)).collect();
            let group_function_refs: Vec<&NSArray<ProtocolObject<dyn MTLFunction>>> =
                group_functions.iter().map(|functions| &**functions).collect();
            NSDictionary::from_slices(&group_name_refs, &group_function_refs)
        });
        unsafe {
            let _: () = msg_send![self, setGroups: groups.as_deref()];
        }
    }

    /// Private functions to be AIR linked but not exported as function handles.
    ///
    /// Availability: macOS 12.0+, iOS 15.0+
    pub fn private_functions(&self) -> Option<Box<[Retained<ProtocolObject<dyn MTLFunction>>]>> {
        let private_functions: Option<Retained<NSArray<ProtocolObject<dyn MTLFunction>>>> =
            unsafe { msg_send![self, privateFunctions] };
        private_functions.map(|functions| functions.to_vec().into_boxed_slice())
    }

    pub fn set_private_functions(
        &self,
        private_functions: Option<&[&ProtocolObject<dyn MTLFunction>]>,
    ) {
        let private_functions = private_functions.map(NSArray::from_slice);
        unsafe {
            let _: () = msg_send![self, setPrivateFunctions: private_functions.as_deref()];
        }
    }
}

impl MTLLinkedFunctions {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub fn new() -> Retained<Self>;
    );
}

impl DefaultRetained for MTLLinkedFunctions {
    #[inline]
    fn default_retained() -> Retained<Self> {
        Self::new()
    }
}
