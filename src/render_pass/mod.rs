mod clear_color;
mod load_action;
mod render_pass_attachment_descriptor;
mod render_pass_color_attachment_descriptor;
mod render_pass_color_attachment_descriptor_array;
mod render_pass_depth_attachment_descriptor;
mod render_pass_descriptor;
mod render_pass_sample_buffer_attachment_descriptor;
mod render_pass_sample_buffer_attachment_descriptor_array;
mod render_pass_stencil_attachment_descriptor;
mod store_action;
mod store_action_options;
mod visibility_result_type;

pub use clear_color::MTLClearColor;
pub use load_action::MTLLoadAction;
pub use render_pass_attachment_descriptor::MTLRenderPassAttachmentDescriptor;
pub use render_pass_color_attachment_descriptor::MTLRenderPassColorAttachmentDescriptor;
pub use render_pass_color_attachment_descriptor_array::MTLRenderPassColorAttachmentDescriptorArray;
#[allow(deprecated)]
pub use render_pass_depth_attachment_descriptor::{
    MTLMultisampleDepthResolveFilter, MTLRenderPassDepthAttachmentDescriptor, MultisampleDepthResolveFilter,
};
pub use render_pass_descriptor::MTLRenderPassDescriptor;
pub use render_pass_sample_buffer_attachment_descriptor::MTLRenderPassSampleBufferAttachmentDescriptor;
pub use render_pass_sample_buffer_attachment_descriptor_array::MTLRenderPassSampleBufferAttachmentDescriptorArray;
pub use render_pass_stencil_attachment_descriptor::{
    MTLMultisampleStencilResolveFilter, MTLRenderPassStencilAttachmentDescriptor,
};
pub use store_action::MTLStoreAction;
pub use store_action_options::MTLStoreActionOptions;
pub use visibility_result_type::MTLVisibilityResultType;

#[cfg(test)]
mod tests {
    use core::mem::{align_of, size_of};

    use super::MTLClearColor;

    #[test]
    fn clear_color_make_equivalent_matches_the_c_layout() {
        let color = MTLClearColor::new(0.25, 0.5, 0.75, 1.0);

        assert_eq!(color.red, 0.25);
        assert_eq!(color.green, 0.5);
        assert_eq!(color.blue, 0.75);
        assert_eq!(color.alpha, 1.0);
        assert_eq!(size_of::<MTLClearColor>(), 4 * size_of::<f64>());
        assert_eq!(align_of::<MTLClearColor>(), align_of::<f64>());
    }
}
