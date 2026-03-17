use core::ptr::NonNull;

pub(crate) fn ref_ptr_cast_const<T: ?Sized>(objects: *const &T) -> *mut NonNull<T> {
    (objects as *mut &T).cast()
}

pub(crate) fn option_ref_ptr_cast_const<T: ?Sized>(objects: *const Option<&T>) -> *mut *const T {
    (objects as *mut Option<&T>).cast()
}
