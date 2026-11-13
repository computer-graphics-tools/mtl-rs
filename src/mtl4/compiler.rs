use std::path::Path;

use block2::RcBlock;
use objc2::{
    Message, extern_class, extern_conformance, extern_methods, extern_protocol, msg_send,
    rc::{Allocated, Retained},
    runtime::ProtocolObject,
};
use objc2_foundation::{CopyingHelper, NSArray, NSCopying, NSError, NSObject, NSObjectProtocol, NSString};

use crate::{CallbackBlock, *};

pub struct MTLNewLibraryCompletionHandler(RcBlock<dyn Fn(*mut ProtocolObject<dyn MTLLibrary>, *mut NSError)>);

impl MTLNewLibraryCompletionHandler {
    /// Creates a callback with captures that can be transferred and shared with Metal's worker threads.
    ///
    /// [Apple's library-creation callback declaration](https://developer.apple.com/documentation/metal/mtldevice/makelibrary(source:options:completionhandler:)) includes:
    ///
    /// > `completionHandler: @escaping @Sendable`
    pub fn new<F>(handler: F) -> Self
    where
        F: Fn(Option<Retained<ProtocolObject<dyn MTLLibrary>>>, Option<MetalError>) + Send + Sync + 'static,
    {
        Self(RcBlock::new(move |library_ptr: *mut ProtocolObject<dyn MTLLibrary>, error: *mut NSError| {
            let library = unsafe { Retained::retain(library_ptr) };
            let error = unsafe { MetalError::from_unretained(error) };
            handler(library, error);
        }))
    }
}

pub struct MTLNewDynamicLibraryCompletionHandler(
    RcBlock<dyn Fn(*mut ProtocolObject<dyn MTLDynamicLibrary>, *mut NSError)>,
);

impl MTLNewDynamicLibraryCompletionHandler {
    /// Creates a callback with captures that can be transferred and shared with Metal's worker threads.
    ///
    /// [Apple's compiler documentation](https://developer.apple.com/documentation/metal/mtl4compiler/newdynamiclibrary:completionhandler:):
    ///
    /// > Creates a new dynamic Metal library instance asynchronously.
    ///
    /// The copied block may outlive this call and execute on a compiler thread,
    /// so its shared `Fn` closure requires `Send + Sync` captures.
    pub fn new<F>(handler: F) -> Self
    where
        F: Fn(Option<Retained<ProtocolObject<dyn MTLDynamicLibrary>>>, Option<MetalError>) + Send + Sync + 'static,
    {
        Self(RcBlock::new(move |library_ptr: *mut ProtocolObject<dyn MTLDynamicLibrary>, error: *mut NSError| {
            let library = unsafe { Retained::retain(library_ptr) };
            let error = unsafe { MetalError::from_unretained(error) };
            handler(library, error);
        }))
    }
}

pub struct MTLNewComputePipelineStateCompletionHandler(
    RcBlock<dyn Fn(*mut ProtocolObject<dyn MTLComputePipelineState>, *mut NSError)>,
);

impl MTLNewComputePipelineStateCompletionHandler {
    /// Creates a callback with captures that can be transferred and shared with Metal's worker threads.
    ///
    /// [Apple's compute-pipeline callback declaration](https://developer.apple.com/documentation/metal/mtldevice/makecomputepipelinestate(function:completionhandler:)) includes:
    ///
    /// > `completionHandler: @escaping @Sendable`
    pub fn new<F>(handler: F) -> Self
    where
        F: Fn(Option<Retained<ProtocolObject<dyn MTLComputePipelineState>>>, Option<MetalError>)
            + Send
            + Sync
            + 'static,
    {
        Self(RcBlock::new(
            move |pipeline_ptr: *mut ProtocolObject<dyn MTLComputePipelineState>, error: *mut NSError| {
                let pipeline = unsafe { Retained::retain(pipeline_ptr) };
                let error = unsafe { MetalError::from_unretained(error) };
                handler(pipeline, error);
            },
        ))
    }
}

extern_class!(
    /// Groups together properties for creating a compiler context.
    ///
    /// Availability: macOS 26.0+, iOS 26.0+
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metal/mtl4compilerdescriptor?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTL4CompilerDescriptor;
);

extern_conformance!(
    unsafe impl NSCopying for MTL4CompilerDescriptor {}
);

unsafe impl CopyingHelper for MTL4CompilerDescriptor {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTL4CompilerDescriptor {}
);

impl MTL4CompilerDescriptor {
    extern_methods!(
        /// Assigns a pipeline data set serializer into which this compiler stores data for all pipelines it creates.
        #[unsafe(method(pipelineDataSetSerializer))]
        #[unsafe(method_family = none)]
        pub fn pipeline_data_set_serializer(
            &self
        ) -> Option<Retained<ProtocolObject<dyn MTL4PipelineDataSetSerializer>>>;

        /// Setter for [`pipelineDataSetSerializer`][Self::pipelineDataSetSerializer].
        #[unsafe(method(setPipelineDataSetSerializer:))]
        #[unsafe(method_family = none)]
        pub fn set_pipeline_data_set_serializer(
            &self,
            pipeline_data_set_serializer: Option<&ProtocolObject<dyn MTL4PipelineDataSetSerializer>>,
        );
    );

    pub fn label(&self) -> Option<String> {
        let label: Option<Retained<NSString>> = unsafe { msg_send![self, label] };
        label.map(|label| label.to_string())
    }

    pub fn set_label(
        &self,
        label: Option<&str>,
    ) {
        unsafe {
            let _: () = msg_send![self, setLabel: label.map(NSString::from_str).as_deref()];
        }
    }
}

/// Methods declared on superclass `NSObject`.
impl MTL4CompilerDescriptor {
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
    /// Configuration that affects an individual compiler task.
    ///
    /// Availability: macOS 26.0+, iOS 26.0+
    ///
    /// See [Apple's documentation](https://developer.apple.com/documentation/metal/mtl4compilertaskoptions?language=objc).
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTL4CompilerTaskOptions;
);

extern_conformance!(
    unsafe impl NSCopying for MTL4CompilerTaskOptions {}
);

unsafe impl CopyingHelper for MTL4CompilerTaskOptions {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTL4CompilerTaskOptions {}
);

impl MTL4CompilerTaskOptions {
    /// Archive instances this compilation process uses to accelerate the build process.
    pub fn lookup_archives(&self) -> Option<Box<[Retained<ProtocolObject<dyn MTL4Archive>>]>> {
        let archives: Option<Retained<NSArray<ProtocolObject<dyn MTL4Archive>>>> =
            unsafe { msg_send![self, lookupArchives] };
        archives.map(|archives| archives.to_vec().into_boxed_slice())
    }

    /// Sets the archive instances with copy semantics.
    pub fn set_lookup_archives(
        &self,
        lookup_archives: Option<&[&ProtocolObject<dyn MTL4Archive>]>,
    ) {
        let lookup_archives = lookup_archives.map(NSArray::from_slice);
        unsafe {
            let _: () = msg_send![self, setLookupArchives: lookup_archives.as_deref()];
        }
    }
}

/// Methods declared on superclass `NSObject`.
impl MTL4CompilerTaskOptions {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub fn new() -> Retained<Self>;
    );
}

/// Sendable callback invoked when compilation of a binary function completes.
///
/// Availability: macOS 26.0+, iOS 26.0+
pub struct MTL4NewBinaryFunctionCompletionHandler(
    RcBlock<dyn Fn(*mut ProtocolObject<dyn MTL4BinaryFunction>, *mut NSError)>,
);

impl MTL4NewBinaryFunctionCompletionHandler {
    /// Creates a completion handler whose captures are safe to invoke from Metal's worker threads.
    /// Metal's borrowed callback objects are retained before invoking `handler`.
    ///
    /// [Apple's callback declaration](https://developer.apple.com/documentation/metal/mtl4newbinaryfunctioncompletionhandler) includes:
    ///
    /// > `@Sendable ((any MTL4BinaryFunction)?, (any Error)?) -> Void`
    pub fn new<F>(handler: F) -> Self
    where
        F: Fn(Option<Retained<ProtocolObject<dyn MTL4BinaryFunction>>>, Option<MetalError>) + Send + Sync + 'static,
    {
        Self(RcBlock::new(move |binary_fn_ptr: *mut ProtocolObject<dyn MTL4BinaryFunction>, error: *mut NSError| {
            let binary_fn = unsafe { Retained::retain(binary_fn_ptr) };
            let error = unsafe { MetalError::from_unretained(error) };
            handler(binary_fn, error);
        }))
    }
}

/// Sendable callback invoked when compilation of a machine-learning pipeline state completes.
///
/// Availability: macOS 26.0+, iOS 26.0+
pub struct MTL4NewMachineLearningPipelineStateCompletionHandler(
    RcBlock<dyn Fn(*mut ProtocolObject<dyn MTL4MachineLearningPipelineState>, *mut NSError)>,
);

impl MTL4NewMachineLearningPipelineStateCompletionHandler {
    /// Creates a completion handler whose captures are safe to invoke from Metal's worker threads.
    /// Metal's borrowed callback objects are retained before invoking `handler`.
    ///
    /// [Apple's callback declaration](https://developer.apple.com/documentation/metal/mtl4newmachinelearningpipelinestatecompletionhandler) includes:
    ///
    /// > `@Sendable ((any MTL4MachineLearningPipelineState)?, (any Error)?) -> Void`
    pub fn new<F>(handler: F) -> Self
    where
        F: Fn(Option<Retained<ProtocolObject<dyn MTL4MachineLearningPipelineState>>>, Option<MetalError>)
            + Send
            + Sync
            + 'static,
    {
        Self(RcBlock::new(
            move |pipeline_ptr: *mut ProtocolObject<dyn MTL4MachineLearningPipelineState>, error: *mut NSError| {
                let pipeline = unsafe { Retained::retain(pipeline_ptr) };
                let error = unsafe { MetalError::from_unretained(error) };
                handler(pipeline, error);
            },
        ))
    }
}

extern_protocol!(
    /// A abstraction for a pipeline state and shader function compiler.
    ///
    /// Availability: macOS 26.0+, iOS 26.0+
    ///
    /// See also [Apple's documentation](https://developer.apple.com/documentation/metal/mtl4compiler?language=objc)
    ///
    /// # Safety
    ///
    /// Implementors must be valid Objective-C objects that conform to the
    /// `MTL4Compiler` protocol. Metal declares this protocol sendable.
    #[expect(
        clippy::missing_safety_doc,
        reason = "extern_protocol does not attach this safety section to its generated unsafe trait"
    )]
    ///
    /// # Thread safety
    ///
    /// [Apple's declaration](https://developer.apple.com/documentation/metal/mtl4compiler):
    ///
    /// > `protocol MTL4Compiler : NSObjectProtocol, Sendable`
    ///
    /// The `Send` and `Sync` bounds rely on this guarantee.
    pub unsafe trait MTL4Compiler: NSObjectProtocol + Send + Sync {
        /// Returns the device that this compiler belongs to.
        #[unsafe(method(device))]
        #[unsafe(method_family = none)]
        fn device(&self) -> Retained<ProtocolObject<dyn MTLDevice>>;

        /// Returns the pipeline data set serializer into which this compiler stores data for all pipelines it creates.
        #[unsafe(method(pipelineDataSetSerializer))]
        #[unsafe(method_family = none)]
        fn pipeline_data_set_serializer(&self) -> Option<Retained<ProtocolObject<dyn MTL4PipelineDataSetSerializer>>>;
    }
);

pub trait MTL4CompilerExt: MTL4Compiler + Message {
    /// Returns the optional label specified at compiler creation time.
    fn label(&self) -> Option<String>
    where
        Self: Sized,
    {
        let label: Option<Retained<NSString>> = unsafe { msg_send![self, label] };
        label.map(|label| label.to_string())
    }

    /// Creates a Metal library synchronously.
    fn new_library_with_descriptor_error(
        &self,
        descriptor: &MTL4LibraryDescriptor,
    ) -> Result<Retained<ProtocolObject<dyn MTLLibrary>>, MetalError>
    where
        Self: Sized,
    {
        unsafe { msg_send![self, newLibraryWithDescriptor: descriptor, error: _] }.map_err(MetalError::from_nserror)
    }

    /// Creates a dynamic library from a Metal IR library synchronously.
    fn new_dynamic_library_error(
        &self,
        library: &ProtocolObject<dyn MTLLibrary>,
    ) -> Result<Retained<ProtocolObject<dyn MTLDynamicLibrary>>, MetalError>
    where
        Self: Sized,
    {
        unsafe { msg_send![self, newDynamicLibrary: library, error: _] }.map_err(MetalError::from_nserror)
    }

    /// Creates a compute pipeline state synchronously.
    fn new_compute_pipeline_state_with_descriptor_compiler_task_options_error(
        &self,
        descriptor: &MTL4ComputePipelineDescriptor,
        compiler_task_options: Option<&MTL4CompilerTaskOptions>,
    ) -> Result<Retained<ProtocolObject<dyn MTLComputePipelineState>>, MetalError>
    where
        Self: Sized,
    {
        unsafe {
            msg_send![
                self,
                newComputePipelineStateWithDescriptor: descriptor,
                compilerTaskOptions: compiler_task_options,
                error: _
            ]
        }
        .map_err(MetalError::from_nserror)
    }

    /// Creates a compute pipeline state with dynamic-linking configuration synchronously.
    fn new_compute_pipeline_state_with_descriptor_dynamic_linking_descriptor_compiler_task_options_error(
        &self,
        descriptor: &MTL4ComputePipelineDescriptor,
        dynamic_linking_descriptor: Option<&MTL4PipelineStageDynamicLinkingDescriptor>,
        compiler_task_options: Option<&MTL4CompilerTaskOptions>,
    ) -> Result<Retained<ProtocolObject<dyn MTLComputePipelineState>>, MetalError>
    where
        Self: Sized,
    {
        unsafe {
            msg_send![
                self,
                newComputePipelineStateWithDescriptor: descriptor,
                dynamicLinkingDescriptor: dynamic_linking_descriptor,
                compilerTaskOptions: compiler_task_options,
                error: _
            ]
        }
        .map_err(MetalError::from_nserror)
    }

    /// Creates a render pipeline state synchronously.
    fn new_render_pipeline_state_with_descriptor_compiler_task_options_error(
        &self,
        descriptor: &MTL4PipelineDescriptor,
        compiler_task_options: Option<&MTL4CompilerTaskOptions>,
    ) -> Result<Retained<ProtocolObject<dyn MTLRenderPipelineState>>, MetalError>
    where
        Self: Sized,
    {
        unsafe {
            msg_send![
                self,
                newRenderPipelineStateWithDescriptor: descriptor,
                compilerTaskOptions: compiler_task_options,
                error: _
            ]
        }
        .map_err(MetalError::from_nserror)
    }

    /// Creates a render pipeline state with dynamic-linking configuration synchronously.
    fn new_render_pipeline_state_with_descriptor_dynamic_linking_descriptor_compiler_task_options_error(
        &self,
        descriptor: &MTL4PipelineDescriptor,
        dynamic_linking_descriptor: Option<&MTL4RenderPipelineDynamicLinkingDescriptor>,
        compiler_task_options: Option<&MTL4CompilerTaskOptions>,
    ) -> Result<Retained<ProtocolObject<dyn MTLRenderPipelineState>>, MetalError>
    where
        Self: Sized,
    {
        unsafe {
            msg_send![
                self,
                newRenderPipelineStateWithDescriptor: descriptor,
                dynamicLinkingDescriptor: dynamic_linking_descriptor,
                compilerTaskOptions: compiler_task_options,
                error: _
            ]
        }
        .map_err(MetalError::from_nserror)
    }

    /// Specializes an existing render pipeline state synchronously.
    fn new_render_pipeline_state_by_specialization_with_descriptor_pipeline_error(
        &self,
        descriptor: &MTL4PipelineDescriptor,
        pipeline: &ProtocolObject<dyn MTLRenderPipelineState>,
    ) -> Result<Retained<ProtocolObject<dyn MTLRenderPipelineState>>, MetalError>
    where
        Self: Sized,
    {
        unsafe {
            msg_send![
                self,
                newRenderPipelineStateBySpecializationWithDescriptor: descriptor,
                pipeline: pipeline,
                error: _
            ]
        }
        .map_err(MetalError::from_nserror)
    }

    /// Creates a binary function synchronously.
    fn new_binary_function_with_descriptor_compiler_task_options_error(
        &self,
        descriptor: &MTL4BinaryFunctionDescriptor,
        compiler_task_options: Option<&MTL4CompilerTaskOptions>,
    ) -> Result<Retained<ProtocolObject<dyn MTL4BinaryFunction>>, MetalError>
    where
        Self: Sized,
    {
        unsafe {
            msg_send![
                self,
                newBinaryFunctionWithDescriptor: descriptor,
                compilerTaskOptions: compiler_task_options,
                error: _
            ]
        }
        .map_err(MetalError::from_nserror)
    }

    /// Creates a machine-learning pipeline state synchronously.
    fn new_machine_learning_pipeline_state_with_descriptor_error(
        &self,
        descriptor: &MTL4MachineLearningPipelineDescriptor,
    ) -> Result<Retained<ProtocolObject<dyn MTL4MachineLearningPipelineState>>, MetalError>
    where
        Self: Sized,
    {
        unsafe { msg_send![self, newMachineLearningPipelineStateWithDescriptor: descriptor, error: _] }
            .map_err(MetalError::from_nserror)
    }

    /// Creates a dynamic library synchronously from a file path.
    fn new_dynamic_library_with_path(
        &self,
        path: &Path,
    ) -> Result<Retained<ProtocolObject<dyn MTLDynamicLibrary>>, MetalError>
    where
        Self: Sized,
    {
        let url = crate::util::file_url(path, "newDynamicLibraryWithURL:error:")?;
        unsafe { msg_send![self, newDynamicLibraryWithURL: &*url, error: _] }.map_err(MetalError::from_nserror)
    }

    fn new_library_with_descriptor_completion_handler(
        &self,
        descriptor: &MTL4LibraryDescriptor,
        completion_handler: MTLNewLibraryCompletionHandler,
    ) -> Retained<ProtocolObject<dyn MTL4CompilerTask>>
    where
        Self: Sized,
    {
        unsafe {
            msg_send![
                self,
                newLibraryWithDescriptor: descriptor,
                completionHandler: &*completion_handler.0
            ]
        }
    }

    fn new_dynamic_library_completion_handler(
        &self,
        library: &ProtocolObject<dyn MTLLibrary>,
        completion_handler: MTLNewDynamicLibraryCompletionHandler,
    ) -> Retained<ProtocolObject<dyn MTL4CompilerTask>>
    where
        Self: Sized,
    {
        unsafe {
            msg_send![
                self,
                newDynamicLibrary: library,
                completionHandler: &*completion_handler.0
            ]
        }
    }

    fn new_dynamic_library_with_path_completion_handler(
        &self,
        path: &Path,
        completion_handler: MTLNewDynamicLibraryCompletionHandler,
    ) -> Result<Retained<ProtocolObject<dyn MTL4CompilerTask>>, MetalError>
    where
        Self: Sized,
    {
        let url = crate::util::file_url(path, "newDynamicLibraryWithURL:completionHandler:")?;
        Ok(unsafe {
            msg_send![
                self,
                newDynamicLibraryWithURL: &*url,
                completionHandler: &*completion_handler.0
            ]
        })
    }

    fn new_compute_pipeline_state_with_descriptor_compiler_task_options_completion_handler(
        &self,
        descriptor: &MTL4ComputePipelineDescriptor,
        compiler_task_options: Option<&MTL4CompilerTaskOptions>,
        completion_handler: MTLNewComputePipelineStateCompletionHandler,
    ) -> Retained<ProtocolObject<dyn MTL4CompilerTask>>
    where
        Self: Sized,
    {
        unsafe {
            msg_send![
                self,
                newComputePipelineStateWithDescriptor: descriptor,
                compilerTaskOptions: compiler_task_options,
                completionHandler: &*completion_handler.0
            ]
        }
    }

    fn new_compute_pipeline_state_with_descriptor_dynamic_linking_descriptor_compiler_task_options_completion_handler(
        &self,
        descriptor: &MTL4ComputePipelineDescriptor,
        dynamic_linking_descriptor: Option<&MTL4PipelineStageDynamicLinkingDescriptor>,
        compiler_task_options: Option<&MTL4CompilerTaskOptions>,
        completion_handler: MTLNewComputePipelineStateCompletionHandler,
    ) -> Retained<ProtocolObject<dyn MTL4CompilerTask>>
    where
        Self: Sized,
    {
        unsafe {
            msg_send![
                self,
                newComputePipelineStateWithDescriptor: descriptor,
                dynamicLinkingDescriptor: dynamic_linking_descriptor,
                compilerTaskOptions: compiler_task_options,
                completionHandler: &*completion_handler.0
            ]
        }
    }

    fn new_render_pipeline_state_with_descriptor_compiler_task_options_completion_handler(
        &self,
        descriptor: &MTL4PipelineDescriptor,
        compiler_task_options: Option<&MTL4CompilerTaskOptions>,
        completion_handler: MTLNewRenderPipelineStateCompletionHandler,
    ) -> Retained<ProtocolObject<dyn MTL4CompilerTask>>
    where
        Self: Sized,
    {
        unsafe {
            msg_send![
                self,
                newRenderPipelineStateWithDescriptor: descriptor,
                compilerTaskOptions: compiler_task_options,
                completionHandler: completion_handler.as_block()
            ]
        }
    }

    fn new_render_pipeline_state_with_descriptor_dynamic_linking_descriptor_compiler_task_options_completion_handler(
        &self,
        descriptor: &MTL4PipelineDescriptor,
        dynamic_linking_descriptor: Option<&MTL4RenderPipelineDynamicLinkingDescriptor>,
        compiler_task_options: Option<&MTL4CompilerTaskOptions>,
        completion_handler: MTLNewRenderPipelineStateCompletionHandler,
    ) -> Retained<ProtocolObject<dyn MTL4CompilerTask>>
    where
        Self: Sized,
    {
        unsafe {
            msg_send![
                self,
                newRenderPipelineStateWithDescriptor: descriptor,
                dynamicLinkingDescriptor: dynamic_linking_descriptor,
                compilerTaskOptions: compiler_task_options,
                completionHandler: completion_handler.as_block()
            ]
        }
    }

    fn new_render_pipeline_state_by_specialization_with_descriptor_pipeline_completion_handler(
        &self,
        descriptor: &MTL4PipelineDescriptor,
        pipeline: &ProtocolObject<dyn MTLRenderPipelineState>,
        completion_handler: MTLNewRenderPipelineStateCompletionHandler,
    ) -> Retained<ProtocolObject<dyn MTL4CompilerTask>>
    where
        Self: Sized,
    {
        unsafe {
            msg_send![
                self,
                newRenderPipelineStateBySpecializationWithDescriptor: descriptor,
                pipeline: pipeline,
                completionHandler: completion_handler.as_block()
            ]
        }
    }

    fn new_binary_function_with_descriptor_compiler_task_options_completion_handler(
        &self,
        descriptor: &MTL4BinaryFunctionDescriptor,
        compiler_task_options: Option<&MTL4CompilerTaskOptions>,
        completion_handler: MTL4NewBinaryFunctionCompletionHandler,
    ) -> Retained<ProtocolObject<dyn MTL4CompilerTask>>
    where
        Self: Sized,
    {
        unsafe {
            msg_send![
                self,
                newBinaryFunctionWithDescriptor: descriptor,
                compilerTaskOptions: compiler_task_options,
                completionHandler: &*completion_handler.0
            ]
        }
    }

    fn new_machine_learning_pipeline_state_with_descriptor_completion_handler(
        &self,
        descriptor: &MTL4MachineLearningPipelineDescriptor,
        completion_handler: MTL4NewMachineLearningPipelineStateCompletionHandler,
    ) -> Retained<ProtocolObject<dyn MTL4CompilerTask>>
    where
        Self: Sized,
    {
        unsafe {
            msg_send![
                self,
                newMachineLearningPipelineStateWithDescriptor: descriptor,
                completionHandler: &*completion_handler.0
            ]
        }
    }
}

impl<T: MTL4Compiler + Message> MTL4CompilerExt for T {}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use objc2::runtime::ProtocolObject;
    use objc2_foundation::{NSError, NSString};

    use super::{MTL4NewBinaryFunctionCompletionHandler, MTL4NewMachineLearningPipelineStateCompletionHandler};
    use crate::MTL4BinaryFunction;

    #[test]
    fn compilation_callbacks_forward_null_results() {
        let calls = Arc::new(Mutex::new(0));
        let binary_calls = Arc::clone(&calls);
        let binary_handler = MTL4NewBinaryFunctionCompletionHandler::new(move |function, error| {
            assert!(function.is_none());
            assert!(error.is_none());
            *binary_calls.lock().unwrap() += 1;
        });
        let machine_learning_calls = Arc::clone(&calls);
        let machine_learning_handler =
            MTL4NewMachineLearningPipelineStateCompletionHandler::new(move |pipeline, error| {
                assert!(pipeline.is_none());
                assert!(error.is_none());
                *machine_learning_calls.lock().unwrap() += 1;
            });

        binary_handler.0.call((core::ptr::null_mut(), core::ptr::null_mut()));
        machine_learning_handler.0.call((core::ptr::null_mut(), core::ptr::null_mut()));

        assert_eq!(*calls.lock().unwrap(), 2);
    }

    #[test]
    fn borrowed_callback_error_is_retained_for_the_safe_handler() {
        let received = Arc::new(Mutex::new(None));
        let received_by_handler = Arc::clone(&received);
        let handler = MTL4NewBinaryFunctionCompletionHandler::new(move |function, error| {
            assert!(function.is_none());
            *received_by_handler.lock().unwrap() = error;
        });
        let domain = NSString::from_str("mtl-rs.MTL4CompilerTests");
        let error = unsafe { NSError::errorWithDomain_code_userInfo(&domain, 27, None) };

        handler.0.call((
            core::ptr::null_mut::<ProtocolObject<dyn MTL4BinaryFunction>>(),
            (&*error as *const NSError).cast_mut(),
        ));

        let received = received.lock().unwrap().take().unwrap();
        assert_eq!(received.code(), 27);
        drop(received);
        assert_eq!(error.code(), 27);
    }
}
