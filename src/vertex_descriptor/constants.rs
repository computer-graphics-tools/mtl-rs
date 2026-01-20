use std::ffi::c_ulong;

/// A constant indicating a dynamic stride for vertex buffer layouts.
///
/// Available since macOS 14.0, iOS 17.0.
pub const BUFFER_LAYOUT_STRIDE_DYNAMIC: c_ulong = c_ulong::MAX;
