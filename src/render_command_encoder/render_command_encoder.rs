use core::ptr::NonNull;
use objc2::{extern_protocol, runtime::ProtocolObject};

use super::{
    MTLCullMode, MTLDepthClipMode, MTLScissorRect, MTLTriangleFillMode, MTLViewport, MTLVisibilityResultMode,
};
use crate::render_pipeline::{MTLLogicalToPhysicalColorAttachmentMap, MTLRenderPipelineState};
use crate::MTLCommandEncoder;

extern_protocol!(
    /// Render command encoder interface.
    pub unsafe trait MTLRenderCommandEncoder: MTLCommandEncoder {
        #[unsafe(method(setRenderPipelineState:))]
        #[unsafe(method_family = none)]
        fn set_render_pipeline_state(
            &self,
            pipeline_state: &ProtocolObject<dyn MTLRenderPipelineState>,
        );

        #[unsafe(method(setViewport:))]
        #[unsafe(method_family = none)]
        fn set_viewport(&self, viewport: MTLViewport);

        #[unsafe(method(setViewports:count:))]
        #[unsafe(method_family = none)]
        unsafe fn set_viewports(&self, viewports: NonNull<MTLViewport>, count: usize);

        #[unsafe(method(setFrontFacingWinding:))]
        #[unsafe(method_family = none)]
        fn set_front_facing_winding(&self, winding: super::types::MTLWinding);

        #[unsafe(method(setCullMode:))]
        #[unsafe(method_family = none)]
        fn set_cull_mode(&self, cull_mode: MTLCullMode);

        #[unsafe(method(setDepthClipMode:))]
        #[unsafe(method_family = none)]
        fn set_depth_clip_mode(&self, mode: MTLDepthClipMode);

        #[unsafe(method(setTriangleFillMode:))]
        #[unsafe(method_family = none)]
        fn set_triangle_fill_mode(&self, mode: MTLTriangleFillMode);

        #[unsafe(method(setScissorRect:))]
        #[unsafe(method_family = none)]
        fn set_scissor_rect(&self, rect: MTLScissorRect);

        #[unsafe(method(setScissorRects:count:))]
        #[unsafe(method_family = none)]
        unsafe fn set_scissor_rects(&self, rects: NonNull<MTLScissorRect>, count: usize);

        #[unsafe(method(setBlendColorRed:green:blue:alpha:))]
        #[unsafe(method_family = none)]
        fn set_blend_color(&self, red: f32, green: f32, blue: f32, alpha: f32);

        #[unsafe(method(setStencilReferenceValue:))]
        #[unsafe(method_family = none)]
        fn set_stencil_reference_value(&self, reference_value: u32);

        #[unsafe(method(setStencilFrontReferenceValue:backReferenceValue:))]
        #[unsafe(method_family = none)]
        fn set_stencil_front_back_reference_value(&self, front: u32, back: u32);

        #[unsafe(method(setVisibilityResultMode:offset:))]
        #[unsafe(method_family = none)]
        fn set_visibility_result_mode(&self, mode: MTLVisibilityResultMode, offset: usize);

        #[unsafe(method(setColorAttachmentMap:))]
        #[unsafe(method_family = none)]
        unsafe fn set_color_attachment_map(
            &self,
            mapping: Option<&MTLLogicalToPhysicalColorAttachmentMap>,
        );
    }
);
