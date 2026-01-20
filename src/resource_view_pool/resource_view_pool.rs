use objc2::{extern_protocol, rc::Retained, runtime::ProtocolObject};
use objc2_foundation::{NSObjectProtocol, NSRange, NSString};

use crate::{MTLDevice, MTLResourceID};

extern_protocol!(
    /// Contains views over resources of a specific type, and allows you to manage those views.
    pub unsafe trait MTLResourceViewPool: NSObjectProtocol {
        /// Obtains the resource ID corresponding to the resource view at index 0 in this resource view pool.
        #[unsafe(method(baseResourceID))]
        #[unsafe(method_family = none)]
        unsafe fn base_resource_id(&self) -> MTLResourceID;

        /// Queries the number of resource views that this pool contains.
        #[unsafe(method(resourceViewCount))]
        #[unsafe(method_family = none)]
        unsafe fn resource_view_count(&self) -> usize;

        /// Obtains a reference to the GPU device this pool belongs to.
        #[unsafe(method(device))]
        #[unsafe(method_family = none)]
        unsafe fn device(&self) -> Retained<ProtocolObject<dyn MTLDevice>>;

        /// Queries the optional debug label of this resource view pool.
        #[unsafe(method(label))]
        #[unsafe(method_family = none)]
        unsafe fn label(&self) -> Option<Retained<NSString>>;

        /// Copies a range of resource views from a source view pool to a destination location in this view pool.
        #[unsafe(method(copyResourceViewsFromPool:sourceRange:destinationIndex:))]
        #[unsafe(method_family = none)]
        unsafe fn copy_resource_views_from_pool(
            &self,
            source_pool: &ProtocolObject<dyn MTLResourceViewPool>,
            source_range: NSRange,
            destination_index: usize,
        ) -> MTLResourceID;
    }
);
