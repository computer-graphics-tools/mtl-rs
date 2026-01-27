use objc2_foundation::NSErrorDomain;

unsafe extern "C" {
    /// Error domain constant for `MTLBinaryArchive` operations.
    ///
    /// Apple's documentation: `https://developer.apple.com/documentation/metal/mtlbinaryarchivedomain`.
    pub static MTLBinaryArchiveDomain: &'static NSErrorDomain;
}

#[allow(unused)]
/// Bridged error domain symbol for `MTLBinaryArchive`.
pub fn binary_archive_domain() -> &'static NSErrorDomain {
    unsafe { MTLBinaryArchiveDomain }
}
