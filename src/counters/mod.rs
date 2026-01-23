mod common;
mod counter;
mod counter_set;
mod descriptor;
mod error;
mod results;
mod sample_buffer;
mod types;

pub use common::{MTLCommonCounter, MTLCommonCounterSet};
pub use counter::{MTLCounter, MTLCounterExt};
pub use counter_set::{MTLCounterSet, MTLCounterSetExt};
pub use descriptor::MTLCounterSampleBufferDescriptor;
pub use error::{
    MTL_COUNTER_DONT_SAMPLE, MTL_COUNTER_ERROR_VALUE, MTLCounterSampleBufferError,
    counter_error_domain,
};
pub use results::{
    MTLCounterResultStageUtilization, MTLCounterResultStatistic, MTLCounterResultTimestamp,
};
pub use sample_buffer::MTLCounterSampleBuffer;
