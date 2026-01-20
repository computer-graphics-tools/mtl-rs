mod descriptor;
mod manager;
mod capture_destination;
mod capture_error;

pub use descriptor::MTLCaptureDescriptor;
pub use manager::MTLCaptureManager;
pub use capture_destination::MTLCaptureDestination;
pub use capture_error::{capture_error_domain, MTLCaptureError};
