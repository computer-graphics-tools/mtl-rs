use core::{
    ffi::{c_char, c_void},
    ptr::NonNull,
};

use crate::{device::MTLIOCompressionMethod, io_compressor::MTLCompressionStatus};

/// Opaque handle to a Metal I/O compression context.
///
/// Availability: macOS 13.0+, iOS 16.0+
#[repr(transparent)]
pub struct MTLCompressionContext(*mut c_void);

unsafe extern "C-unwind" {
    fn MTLIOCompressionContextDefaultChunkSize() -> usize;
}

unsafe extern "C-unwind" {
    /// Safety: `path` must be a valid, null-terminated C string.
    fn MTLIOCreateCompressionContext(
        path: *const c_char,
        r#type: MTLIOCompressionMethod,
        chunk_size: usize,
    ) -> *mut c_void;
}

unsafe extern "C-unwind" {
    /// Safety: `context` and `data` must be valid pointers.
    fn MTLIOCompressionContextAppendData(context: *mut c_void, data: *const c_void, size: usize);
}

unsafe extern "C-unwind" {
    /// Safety: `context` must be a valid pointer.
    fn MTLIOFlushAndDestroyCompressionContext(context: *mut c_void) -> MTLCompressionStatus;
}

impl MTLCompressionContext {
    /// Returns Apple's default chunk size for the compression context.
    ///
    /// Availability: macOS 13.0+, iOS 16.0+
    pub fn default_chunk_size() -> usize {
        unsafe { MTLIOCompressionContextDefaultChunkSize() }
    }

    /// Create a new compression context that writes to the file at `path`.
    ///
    /// Safety: `path` must be a valid, null-terminated C string pointer.
    pub fn create(
        path: NonNull<c_char>,
        method: MTLIOCompressionMethod,
        chunk_size: usize,
    ) -> Option<Self> {
        let raw = unsafe { MTLIOCreateCompressionContext(path.as_ptr(), method, chunk_size) };
        if raw.is_null() { None } else { Some(Self(raw)) }
    }

    /// Append raw data to the compression stream.
    ///
    /// Availability: macOS 13.0+, iOS 16.0+
    ///
    /// Safety: `data` must be valid for reads of `size` bytes for the duration of the call.
    pub fn append_data(&mut self, data: NonNull<c_void>, size: usize) {
        unsafe { MTLIOCompressionContextAppendData(self.0, data.as_ptr(), size) };
    }

    /// Flush pending data and destroy the context, returning the final status.
    ///
    /// Availability: macOS 13.0+, iOS 16.0+
    /// The handle becomes invalid after this call.
    pub fn flush_and_destroy(self) -> MTLCompressionStatus {
        let status = unsafe { MTLIOFlushAndDestroyCompressionContext(self.0) };
        core::mem::forget(self);
        status
    }
}

impl Drop for MTLCompressionContext {
    fn drop(&mut self) {
        // Best-effort: avoid double-drop by consuming in `flush_and_destroy`.
        // If user forgot to call it, attempt to flush and destroy here.
        unsafe {
            let _ = MTLIOFlushAndDestroyCompressionContext(self.0);
        }
    }
}
