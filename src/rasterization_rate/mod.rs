mod layer_array;
mod layer_descriptor;
mod map;
mod map_descriptor;
mod sample_array;

pub use layer_array::MTLRasterizationRateLayerArray;
pub use layer_descriptor::MTLRasterizationRateLayerDescriptor;
pub use map::{MTLRasterizationRateMap, MTLRasterizationRateMapExt};
pub use map_descriptor::MTLRasterizationRateMapDescriptor;
pub use sample_array::MTLRasterizationRateSampleArray;

#[cfg(test)]
mod tests {
    use super::MTLRasterizationRateLayerDescriptor;
    use crate::MTLSize;

    #[test]
    #[should_panic(expected = "horizontal sample count does not match sample_count.width")]
    fn layer_descriptor_rejects_a_short_horizontal_slice() {
        let _ = MTLRasterizationRateLayerDescriptor::new_with_sample_count_horizontal_vertical(
            MTLSize::new(2, 1, 0),
            &[1.0],
            &[1.0],
        );
    }
}
