#![allow(deprecated)]

use core::ptr::NonNull;

use block2::{Block, RcBlock};
use objc2::{rc::Retained, runtime::ProtocolObject};
use objc2_foundation::{NSArray, NSObjectProtocol, NSString};

use super::MTLDevice;

type RawDeviceNotificationName = NSString;

unsafe extern "C" {
    /// Posted when a Metal device is added to the system.
    #[deprecated(note = "Metal device notifications do not apply to Apple silicon")]
    static MTLDeviceWasAddedNotification: &'static RawDeviceNotificationName;

    /// Posted when the user requests that an application stop using a device.
    #[deprecated(note = "Metal device notifications do not apply to Apple silicon")]
    static MTLDeviceRemovalRequestedNotification: &'static RawDeviceNotificationName;

    /// Posted when a Metal device is removed from the system.
    #[deprecated(note = "Metal device notifications do not apply to Apple silicon")]
    static MTLDeviceWasRemovedNotification: &'static RawDeviceNotificationName;
}

/// The name posted when a Metal device is added to the system.
#[deprecated(note = "Metal device notifications do not apply to Apple silicon")]
pub fn device_was_added_notification() -> String {
    unsafe { MTLDeviceWasAddedNotification }.to_string()
}

/// The name posted when the user requests that an application stop using a device.
#[deprecated(note = "Metal device notifications do not apply to Apple silicon")]
pub fn device_removal_requested_notification() -> String {
    unsafe { MTLDeviceRemovalRequestedNotification }.to_string()
}

/// The name posted when a Metal device is removed from the system.
#[deprecated(note = "Metal device notifications do not apply to Apple silicon")]
pub fn device_was_removed_notification() -> String {
    unsafe { MTLDeviceWasRemovedNotification }.to_string()
}

type DeviceNotificationBlock = dyn Fn(NonNull<ProtocolObject<dyn MTLDevice>>, NonNull<RawDeviceNotificationName>);

/// A sendable handler used by the legacy Metal device-notification APIs.
#[deprecated(note = "Metal device notifications do not apply to Apple silicon")]
pub struct MTLDeviceNotificationHandler(RcBlock<DeviceNotificationBlock>);

impl MTLDeviceNotificationHandler {
    /// Creates a handler whose captured state can safely be sent to and shared
    /// with Metal's callback thread.
    ///
    /// [Apple's declaration](https://developer.apple.com/documentation/metal/mtldevicenotificationhandler):
    ///
    /// > `typealias MTLDeviceNotificationHandler = @Sendable (any MTLDevice, MTLDeviceNotificationName) -> Void`
    pub fn new<F>(handler: F) -> Self
    where
        F: Fn(&ProtocolObject<dyn MTLDevice>, &str) + Send + Sync + 'static,
    {
        Self(RcBlock::new(
            move |device: NonNull<ProtocolObject<dyn MTLDevice>>, name: NonNull<RawDeviceNotificationName>| {
                let name = unsafe { name.as_ref() }.to_string();
                handler(unsafe { device.as_ref() }, &name);
            },
        ))
    }
}

/// An opaque token that owns a legacy Metal device-change observer.
#[deprecated(note = "Metal device notifications do not apply to Apple silicon")]
pub struct MTLDeviceObserver(Retained<ProtocolObject<dyn NSObjectProtocol>>);

/// Returns all devices and installs a legacy device-change observer.
///
/// The returned observer owns the +1 retain count documented by Metal. Pass
/// it to [`remove_device_observer`] to stop notifications.
#[deprecated(note = "Metal device notifications do not apply to Apple silicon")]
pub fn copy_all_devices_with_observer(
    handler: &MTLDeviceNotificationHandler
) -> (Box<[Retained<ProtocolObject<dyn MTLDevice>>]>, Option<MTLDeviceObserver>) {
    unsafe extern "C" {
        fn MTLCopyAllDevicesWithObserver(
            observer: *mut *mut ProtocolObject<dyn NSObjectProtocol>,
            handler: &Block<DeviceNotificationBlock>,
        ) -> *mut NSArray<ProtocolObject<dyn MTLDevice>>;
    }

    let mut observer = core::ptr::null_mut();
    let devices = unsafe { MTLCopyAllDevicesWithObserver(&mut observer, &handler.0) };
    let devices = unsafe { Retained::from_raw(devices).expect("MTLCopyAllDevicesWithObserver returned a null array") };
    let observer = unsafe { Retained::from_raw(observer) }.map(MTLDeviceObserver);
    (devices.to_vec().into_boxed_slice(), observer)
}

/// Removes a legacy Metal device-change observer.
///
/// `observer` should be the object returned by
/// [`copy_all_devices_with_observer`].
#[deprecated(note = "Metal device notifications do not apply to Apple silicon")]
pub fn remove_device_observer(observer: MTLDeviceObserver) {
    unsafe extern "C" {
        fn MTLRemoveDeviceObserver(observer: &ProtocolObject<dyn NSObjectProtocol>);
    }

    unsafe { MTLRemoveDeviceObserver(&observer.0) };
}
