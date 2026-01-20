use objc2::{
    extern_class, extern_conformance, extern_methods, msg_send,
    rc::{Allocated, Retained},
    runtime::NSObject,
};
use objc2_foundation::{CopyingHelper, NSCopying, NSDictionary, NSObjectProtocol, NSString};

use super::{
    MLTLanguageVersion, MTLCompileSymbolVisibility, MTLLibraryOptimizationLevel, MTLLibraryType,
    MTLMathFloatingPointFunctions, MTLMathMode,
};
use crate::types::MTLSize;

extern_class!(
    /// Options for compiling Metal libraries.
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLCompileOptions;
);

extern_conformance!(
    unsafe impl NSCopying for MTLCompileOptions {}
);

unsafe impl CopyingHelper for MTLCompileOptions {
    type Result = Self;
}

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLCompileOptions {}
);

impl MTLCompileOptions {
    extern_methods!(
        #[unsafe(method(preprocessorMacros))]
        #[unsafe(method_family = none)]
        pub fn preprocessor_macros(&self) -> Option<Retained<NSDictionary<NSString, NSObject>>>;

        #[unsafe(method(setPreprocessorMacros:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_preprocessor_macros(
            &self,
            macros: Option<&NSDictionary<NSString, NSObject>>,
        );

        #[unsafe(method(mathMode))]
        #[unsafe(method_family = none)]
        pub unsafe fn math_mode(&self) -> MTLMathMode;

        #[unsafe(method(setMathMode:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_math_mode(&self, math_mode: MTLMathMode);

        #[unsafe(method(mathFloatingPointFunctions))]
        #[unsafe(method_family = none)]
        pub unsafe fn math_floating_point_functions(&self) -> MTLMathFloatingPointFunctions;

        #[unsafe(method(setMathFloatingPointFunctions:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_math_floating_point_functions(&self, val: MTLMathFloatingPointFunctions);

        #[unsafe(method(languageVersion))]
        #[unsafe(method_family = none)]
        pub fn language_version(&self) -> MLTLanguageVersion;

        #[unsafe(method(setLanguageVersion:))]
        #[unsafe(method_family = none)]
        pub fn set_language_version(&self, v: MLTLanguageVersion);

        #[unsafe(method(libraryType))]
        #[unsafe(method_family = none)]
        pub fn library_type(&self) -> MTLLibraryType;

        #[unsafe(method(setLibraryType:))]
        #[unsafe(method_family = none)]
        pub fn set_library_type(&self, v: MTLLibraryType);

        #[unsafe(method(preserveInvariance))]
        #[unsafe(method_family = none)]
        pub fn preserve_invariance(&self) -> bool;

        #[unsafe(method(setPreserveInvariance:))]
        #[unsafe(method_family = none)]
        pub fn set_preserve_invariance(&self, v: bool);

        #[unsafe(method(optimizationLevel))]
        #[unsafe(method_family = none)]
        pub unsafe fn optimization_level(&self) -> MTLLibraryOptimizationLevel;

        #[unsafe(method(setOptimizationLevel:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_optimization_level(&self, v: MTLLibraryOptimizationLevel);

        #[unsafe(method(compileSymbolVisibility))]
        #[unsafe(method_family = none)]
        pub unsafe fn compile_symbol_visibility(&self) -> MTLCompileSymbolVisibility;

        #[unsafe(method(setCompileSymbolVisibility:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_compile_symbol_visibility(&self, v: MTLCompileSymbolVisibility);

        #[unsafe(method(allowReferencingUndefinedSymbols))]
        #[unsafe(method_family = none)]
        pub unsafe fn allow_referencing_undefined_symbols(&self) -> bool;

        #[unsafe(method(setAllowReferencingUndefinedSymbols:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_allow_referencing_undefined_symbols(&self, v: bool);

        #[unsafe(method(maxTotalThreadsPerThreadgroup))]
        #[unsafe(method_family = none)]
        pub unsafe fn max_total_threads_per_threadgroup(&self) -> usize;

        #[unsafe(method(setMaxTotalThreadsPerThreadgroup:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_max_total_threads_per_threadgroup(&self, v: usize);

        #[unsafe(method(requiredThreadsPerThreadgroup))]
        #[unsafe(method_family = none)]
        pub unsafe fn required_threads_per_threadgroup(&self) -> MTLSize;

        #[unsafe(method(setRequiredThreadsPerThreadgroup:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_required_threads_per_threadgroup(&self, v: MTLSize);

        #[unsafe(method(enableLogging))]
        #[unsafe(method_family = none)]
        pub unsafe fn enable_logging(&self) -> bool;

        #[unsafe(method(setEnableLogging:))]
        #[unsafe(method_family = none)]
        pub unsafe fn set_enable_logging(&self, v: bool);
    );
}

impl MTLCompileOptions {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub fn new() -> Retained<Self>;
    );
}

#[allow(unused)]
impl MTLCompileOptions {
    fn install_name(&self) -> Option<String> {
        let s: Option<Retained<NSString>> = unsafe { msg_send![self, installName] };
        s.map(|s| s.to_string())
    }

    fn set_install_name(&self, name: Option<&str>) {
        unsafe {
            let _: () = msg_send![self, setInstallName: name.map(NSString::from_str).as_deref()];
        }
    }
}
