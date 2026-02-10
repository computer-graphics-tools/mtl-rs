use objc2::{
    encode::Encoding,
    extern_class, extern_conformance, extern_methods, msg_send,
    rc::{Allocated, Retained},
    runtime::NSObject,
};
use objc2_foundation::{
    CopyingHelper, NSCopying, NSDictionary, NSNumber, NSObjectProtocol, NSString,
};

use super::{
    MLTLanguageVersion, MTLCompileSymbolVisibility, MTLLibraryOptimizationLevel, MTLLibraryType,
    MTLMathFloatingPointFunctions, MTLMathMode,
};
use crate::types::MTLSize;

#[derive(Debug, Clone, PartialEq)]
pub enum MTLPreprocessorMacroValue {
    String(String),
    I64(i64),
    U64(u64),
    F64(f64),
}

impl MTLPreprocessorMacroValue {
    fn from_ns_object(value: Retained<NSObject>) -> Self {
        let value = match value.downcast::<NSString>() {
            Ok(value) => return Self::String(value.to_string()),
            Err(value) => value,
        };

        let value = match value.downcast::<NSNumber>() {
            Ok(value) => {
                return match value.encoding() {
                    Encoding::Char
                    | Encoding::Short
                    | Encoding::Int
                    | Encoding::Long
                    | Encoding::LongLong => Self::I64(value.as_i64()),
                    Encoding::UChar
                    | Encoding::UShort
                    | Encoding::UInt
                    | Encoding::ULong
                    | Encoding::ULongLong => Self::U64(value.as_u64()),
                    Encoding::Float | Encoding::Double => Self::F64(value.as_f64()),
                    _ => unreachable!("unexpected NSNumber encoding"),
                };
            }
            Err(value) => value,
        };

        // Fallback to object description so we never leak NSObject in the public API.
        let description: Retained<NSString> = unsafe { msg_send![&*value, description] };
        Self::String(description.to_string())
    }

    fn to_ns_object(&self) -> Retained<NSObject> {
        match self {
            Self::String(value) => NSString::from_str(value)
                .downcast::<NSObject>()
                .expect("NSString must be an NSObject"),
            Self::I64(value) => NSNumber::new_i64(*value)
                .downcast::<NSObject>()
                .expect("NSNumber must be an NSObject"),
            Self::U64(value) => NSNumber::new_u64(*value)
                .downcast::<NSObject>()
                .expect("NSNumber must be an NSObject"),
            Self::F64(value) => NSNumber::new_f64(*value)
                .downcast::<NSObject>()
                .expect("NSNumber must be an NSObject"),
        }
    }
}

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
        #[unsafe(method(mathMode))]
        #[unsafe(method_family = none)]
        pub fn math_mode(&self) -> MTLMathMode;

        #[unsafe(method(setMathMode:))]
        #[unsafe(method_family = none)]
        pub fn set_math_mode(&self, math_mode: MTLMathMode);

        #[unsafe(method(mathFloatingPointFunctions))]
        #[unsafe(method_family = none)]
        pub fn math_floating_point_functions(&self) -> MTLMathFloatingPointFunctions;

        #[unsafe(method(setMathFloatingPointFunctions:))]
        #[unsafe(method_family = none)]
        pub fn set_math_floating_point_functions(&self, val: MTLMathFloatingPointFunctions);

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
        pub fn optimization_level(&self) -> MTLLibraryOptimizationLevel;

        #[unsafe(method(setOptimizationLevel:))]
        #[unsafe(method_family = none)]
        pub fn set_optimization_level(&self, v: MTLLibraryOptimizationLevel);

        #[unsafe(method(compileSymbolVisibility))]
        #[unsafe(method_family = none)]
        pub fn compile_symbol_visibility(&self) -> MTLCompileSymbolVisibility;

        #[unsafe(method(setCompileSymbolVisibility:))]
        #[unsafe(method_family = none)]
        pub fn set_compile_symbol_visibility(&self, v: MTLCompileSymbolVisibility);

        #[unsafe(method(allowReferencingUndefinedSymbols))]
        #[unsafe(method_family = none)]
        pub fn allow_referencing_undefined_symbols(&self) -> bool;

        #[unsafe(method(setAllowReferencingUndefinedSymbols:))]
        #[unsafe(method_family = none)]
        pub fn set_allow_referencing_undefined_symbols(&self, v: bool);

        #[unsafe(method(maxTotalThreadsPerThreadgroup))]
        #[unsafe(method_family = none)]
        pub fn max_total_threads_per_threadgroup(&self) -> usize;

        #[unsafe(method(setMaxTotalThreadsPerThreadgroup:))]
        #[unsafe(method_family = none)]
        pub fn set_max_total_threads_per_threadgroup(&self, v: usize);

        #[unsafe(method(requiredThreadsPerThreadgroup))]
        #[unsafe(method_family = none)]
        pub fn required_threads_per_threadgroup(&self) -> MTLSize;

        #[unsafe(method(setRequiredThreadsPerThreadgroup:))]
        #[unsafe(method_family = none)]
        pub fn set_required_threads_per_threadgroup(&self, v: MTLSize);

        #[unsafe(method(enableLogging))]
        #[unsafe(method_family = none)]
        pub fn enable_logging(&self) -> bool;

        #[unsafe(method(setEnableLogging:))]
        #[unsafe(method_family = none)]
        pub fn set_enable_logging(&self, v: bool);
    );

    pub fn preprocessor_macros(&self) -> Option<Box<[(String, MTLPreprocessorMacroValue)]>> {
        let macros: Option<Retained<NSDictionary<NSString, NSObject>>> =
            unsafe { msg_send![self, preprocessorMacros] };
        macros.map(|macros| {
            let (keys, values) = macros.to_vecs();
            keys.into_iter()
                .zip(values)
                .map(|(key, value)| {
                    (
                        key.to_string(),
                        MTLPreprocessorMacroValue::from_ns_object(value),
                    )
                })
                .collect::<Vec<_>>()
                .into_boxed_slice()
        })
    }

    pub fn set_preprocessor_macros(&self, macros: Option<&[(&str, MTLPreprocessorMacroValue)]>) {
        let macros = macros.map(|macros| {
            let keys: Vec<Retained<NSString>> = macros
                .iter()
                .map(|(key, _)| NSString::from_str(key))
                .collect();
            let key_refs: Vec<&NSString> = keys.iter().map(|key| &**key).collect();
            let values: Vec<Retained<NSObject>> = macros
                .iter()
                .map(|(_, value)| value.to_ns_object())
                .collect();
            let value_refs: Vec<&NSObject> = values.iter().map(|value| &**value).collect();
            NSDictionary::from_slices(&key_refs, &value_refs)
        });
        unsafe {
            let _: () = msg_send![self, setPreprocessorMacros: macros.as_deref()];
        }
    }
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
