use objc2::{extern_class, extern_methods, runtime::NSObject};

use crate::MTLDataType;

extern_class!(
    /// Base class for reflection types used by argument/pipeline reflection.
    ///
    /// Availability: macOS 10.13+, iOS 11.0+
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLType;
);

impl MTLType {
    extern_methods!(
        /// The data type for this reflection type.
        #[unsafe(method(dataType))]
        #[unsafe(method_family = none)]
        pub fn data_type(&self) -> MTLDataType;
    );
}
