use core::ops::Range;

use objc2::{Message, extern_protocol, msg_send, rc::Retained, runtime::ProtocolObject};
use objc2_foundation::{NSObjectProtocol, NSRange, NSString};

use crate::{MTLDevice, MTLResourceID};

extern_protocol!(
    /// Contains views over resources of a specific type, and allows you to manage those views.
    pub unsafe trait MTLResourceViewPool: NSObjectProtocol {
        /// Obtains the resource ID corresponding to the resource view at index 0 in this resource view pool.
        #[unsafe(method(baseResourceID))]
        #[unsafe(method_family = none)]
        fn base_resource_id(&self) -> MTLResourceID;

        /// Queries the number of resource views that this pool contains.
        #[unsafe(method(resourceViewCount))]
        #[unsafe(method_family = none)]
        fn resource_view_count(&self) -> usize;

        /// Obtains a reference to the GPU device this pool belongs to.
        #[unsafe(method(device))]
        #[unsafe(method_family = none)]
        fn device(&self) -> Retained<ProtocolObject<dyn MTLDevice>>;

        /// Queries the optional debug label of this resource view pool.
        #[unsafe(method(label))]
        #[unsafe(method_family = none)]
        fn label(&self) -> Option<Retained<NSString>>;
    }
);

#[allow(unused)]
pub trait MTLResourceViewPoolExt: MTLResourceViewPool + Message {
    /// Copies a range of resource views from a source view pool to a destination location in this view pool.
    fn copy_resource_views_from_pool(
        &self,
        source_pool: &ProtocolObject<dyn MTLResourceViewPool>,
        source_range: Range<usize>,
        destination_index: usize,
    ) -> MTLResourceID
    where
        Self: Sized,
    {
        unsafe {
            msg_send![
                self,
                copyResourceViewsFromPool: source_pool,
                sourceRange: NSRange::from(source_range),
                destinationIndex: destination_index
            ]
        }
    }
}

impl<T: MTLResourceViewPool + Message> MTLResourceViewPoolExt for T {}
