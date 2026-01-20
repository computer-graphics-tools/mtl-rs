mod origin;
mod region;
mod resource_id;
mod sample_position;
mod size;

pub use origin::MTLOrigin;
pub use region::MTLRegion;
pub use resource_id::MTLResourceID;
pub use sample_position::MTLSamplePosition;
pub use size::MTLSize;

/// A floating point coordinate in an abstract 2D space.
pub type Coordinate2D = MTLSamplePosition;
