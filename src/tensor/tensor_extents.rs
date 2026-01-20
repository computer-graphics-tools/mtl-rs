use objc2::__framework_prelude::*;
use objc2::{
    extern_class, extern_conformance, extern_methods, msg_send,
    rc::{Allocated, Retained},
    runtime::NSObject,
};
use objc2_foundation::NSObjectProtocol;

extern_class!(
    /// A container for tensor rank and extents per dimension.
    ///
    /// [Apple's documentation](https://developer.apple.com/documentation/metal/mtltensorextents?language=objc)
    #[unsafe(super(NSObject))]
    #[derive(Debug, PartialEq, Eq, Hash)]
    pub struct MTLTensorExtents;
);

extern_conformance!(
    unsafe impl NSObjectProtocol for MTLTensorExtents {}
);

impl MTLTensorExtents {
    extern_methods!(
        /// Obtains the rank of the tensor.
        ///
        /// The rank represents the number of dimensions.
        #[unsafe(method(rank))]
        #[unsafe(method_family = none)]
        pub fn rank(&self) -> usize;

        /// Returns the extent at an index.
        ///
        /// Parameter `dimensionIndex`: the index of the dimension. The first dimension is the innermost dimension.
        /// Returns: the extent at `dimensionIndex`. Returns -1 if `dimensionIndex` is greater than or equal to `rank`.
        #[unsafe(method(extentAtDimensionIndex:))]
        #[unsafe(method_family = none)]
        pub fn extent_at_dimension_index(&self, dimension_index: usize) -> isize;
    );
}

/// Methods declared on superclass `NSObject`.
impl MTLTensorExtents {
    extern_methods!(
        #[unsafe(method(init))]
        #[unsafe(method_family = init)]
        pub unsafe fn init(this: Allocated<Self>) -> Retained<Self>;

        #[unsafe(method(new))]
        #[unsafe(method_family = new)]
        pub unsafe fn new() -> Retained<Self>;
    );
}

impl MTLTensorExtents {
    /// Creates a new tensor extents with the rank and extent values you provide.
    ///
    /// Zero rank extents represent scalars. `values` can only be `nil`if `rank` is 0.
    /// - Parameters:
    ///   - rank: the number of dimensions.
    ///   - values: an array of length `rank` that specifies the size of each dimension. The first dimension is the innermost dimension.
    /// - Returns: Tensor extents with the rank and extent values you provide. Returns `nil` if `rank` exceeds 0 and `values` is nil or if `rank` exceeds ``MTL_TENSOR_MAX_RANK``.
    pub fn new_with_rank_values(rank: usize, values: Option<&[isize]>) -> Option<Retained<Self>> {
        let values_ptr: *const isize = match values {
            Some(v) => {
                debug_assert_eq!(v.len(), rank);
                v.as_ptr()
            }
            None => {
                debug_assert_eq!(rank, 0);
                core::ptr::null()
            }
        };
        unsafe {
            let this: Allocated<Self> = msg_send![Self::class(), alloc];
            msg_send![this, initWithRank: rank, values: values_ptr]
        }
    }
}
