use objc2::{ClassType, Encode, Encoding, RefEncode, extern_methods};
use objc2_foundation::NSNotificationName;

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
    static NSProcessInfoPerformanceProfileDidChangeNotification:
        Option<&'static NSNotificationName>;
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
pub fn process_info_performance_profile_did_change_notification()
-> Option<&'static NSNotificationName> {
    unsafe { NSProcessInfoPerformanceProfileDidChangeNotification }
}

mod private_nsprocessinfo_device_certification {
    pub trait Sealed {}
}

/// Category "NSDeviceCertification" on `NSProcessInfo`.
pub unsafe trait NSProcessInfoDeviceCertification:
    ClassType + Sized + private_nsprocessinfo_device_certification::Sealed
{
    extern_methods!(
        #[unsafe(method(isDeviceCertifiedFor:))]
        #[unsafe(method_family = none)]
        fn is_device_certified_for(&self, performance_tier: DeviceCertification) -> bool;

        #[unsafe(method(hasPerformanceProfile:))]
        #[unsafe(method_family = none)]
        fn has_performance_profile(
            &self,
            performance_profile: ProcessPerformanceProfile,
        ) -> bool;
    );
}

impl private_nsprocessinfo_device_certification::Sealed for objc2_foundation::NSProcessInfo {}
unsafe impl NSProcessInfoDeviceCertification for objc2_foundation::NSProcessInfo {}
