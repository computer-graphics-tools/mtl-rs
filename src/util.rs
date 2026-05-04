//! Helpers for reinterpreting Rust slices of object references as the raw
//! pointer arrays that Objective-C methods expect.
//!
//! The casts here rely on documented layout equivalences:
//! - `&T` has the same memory layout as `NonNull<T>` (both are non-null,
//!   properly aligned pointers).
//! - `Option<&T>` has the same memory layout as `*const T` (the `None`
//!   niche occupies the null-pointer value).

use core::ptr::NonNull;

/// Reinterprets `&[&T]` as `*mut NonNull<T>` — the layout an Objective-C
/// method expects for `id<T> _Nonnull * objects` paired with a count.
/// Combine with `slice.len()` at the call site.
pub(crate) fn ref_slice_as_ptr<T: ?Sized>(slice: &[&T]) -> *mut NonNull<T> {
    (slice.as_ptr() as *mut &T).cast()
}

/// Reinterprets `&[Option<&T>]` as `*mut *const T` — the layout an
/// Objective-C method expects for `id<T> _Nullable * objects` paired with
/// a count. Combine with `slice.len()` at the call site.
pub(crate) fn opt_ref_slice_as_ptr<T: ?Sized>(slice: &[Option<&T>]) -> *mut *const T {
    (slice.as_ptr() as *mut Option<&T>).cast()
}
