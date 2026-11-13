use objc2::{Encode, Encoding, RefEncode, msg_send};
use objc2_foundation::{NSNotificationName, NSProcessInfo};

use crate::DeviceCertification;

/// Process performance profile.
///
/// Mirrors `NSProcessPerformanceProfile` (an `NSInteger`-backed typed enum).
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct ProcessPerformanceProfile(pub i64);

unsafe impl Encode for ProcessPerformanceProfile {
    const ENCODING: Encoding = i64::ENCODING;
}

unsafe impl RefEncode for ProcessPerformanceProfile {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

unsafe extern "C" {
    static NSProcessPerformanceProfileDefault: i64;
    static NSProcessPerformanceProfileSustained: i64;
    static NSProcessInfoPerformanceProfileDidChangeNotification: Option<&'static NSNotificationName>;
}

/// Default process profile value.
#[inline]
pub fn process_performance_profile_default() -> ProcessPerformanceProfile {
    unsafe { ProcessPerformanceProfile(NSProcessPerformanceProfileDefault) }
}

/// Sustained process profile value.
#[inline]
pub fn process_performance_profile_sustained() -> ProcessPerformanceProfile {
    unsafe { ProcessPerformanceProfile(NSProcessPerformanceProfileSustained) }
}

/// Notification sent when the process performance profile changes.
#[inline]
pub fn process_info_performance_profile_did_change_notification() -> Option<String> {
    unsafe { NSProcessInfoPerformanceProfileDidChangeNotification }.map(ToString::to_string)
}

/// Returns whether the current device is certified for `performance_tier`.
pub fn is_device_certified_for(performance_tier: DeviceCertification) -> bool {
    let process_info = NSProcessInfo::processInfo();
    unsafe { msg_send![&*process_info, isDeviceCertifiedFor: performance_tier] }
}

/// Returns whether the current process supports `performance_profile`.
pub fn has_performance_profile(performance_profile: ProcessPerformanceProfile) -> bool {
    let process_info = NSProcessInfo::processInfo();
    unsafe { msg_send![&*process_info, hasPerformanceProfile: performance_profile] }
}
