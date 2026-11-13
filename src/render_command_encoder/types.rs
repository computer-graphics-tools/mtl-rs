use objc2::{Encode, Encoding, RefEncode};

#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MTLPrimitiveType {
    Point = 0,
    Line = 1,
    LineStrip = 2,
    Triangle = 3,
    TriangleStrip = 4,
}

unsafe impl Encode for MTLPrimitiveType {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for MTLPrimitiveType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MTLVisibilityResultMode {
    Disabled = 0,
    Boolean = 1,
    Counting = 2,
}

unsafe impl Encode for MTLVisibilityResultMode {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for MTLVisibilityResultMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MTLScissorRect {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

unsafe impl Encode for MTLScissorRect {
    const ENCODING: Encoding =
        Encoding::Struct("?", &[usize::ENCODING, usize::ENCODING, usize::ENCODING, usize::ENCODING]);
}

unsafe impl RefEncode for MTLScissorRect {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MTLViewport {
    pub origin_x: f64,
    pub origin_y: f64,
    pub width: f64,
    pub height: f64,
    pub znear: f64,
    pub zfar: f64,
}

unsafe impl Encode for MTLViewport {
    const ENCODING: Encoding = Encoding::Struct(
        "?",
        &[f64::ENCODING, f64::ENCODING, f64::ENCODING, f64::ENCODING, f64::ENCODING, f64::ENCODING],
    );
}

unsafe impl RefEncode for MTLViewport {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MTLCullMode {
    None = 0,
    Front = 1,
    Back = 2,
}

unsafe impl Encode for MTLCullMode {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for MTLCullMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MTLWinding {
    Clockwise = 0,
    CounterClockwise = 1,
}

unsafe impl Encode for MTLWinding {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for MTLWinding {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MTLDepthClipMode {
    Clip = 0,
    Clamp = 1,
}

unsafe impl Encode for MTLDepthClipMode {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for MTLDepthClipMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[repr(u64)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MTLTriangleFillMode {
    Fill = 0,
    Lines = 1,
}

unsafe impl Encode for MTLTriangleFillMode {
    const ENCODING: Encoding = u64::ENCODING;
}

unsafe impl RefEncode for MTLTriangleFillMode {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// Render-pass stages used by render encoders and render pipeline state.
///
/// Availability: macOS 10.13+, iOS 10.0+
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MTLRenderStages(pub usize);

bitflags::bitflags! {
    impl MTLRenderStages: usize {
        const Vertex = 1 << 0;
        const Fragment = 1 << 1;
        const Tile = 1 << 2;
        const Object = 1 << 3;
        const Mesh = 1 << 4;
    }
}

unsafe impl Encode for MTLRenderStages {
    const ENCODING: Encoding = usize::ENCODING;
}

unsafe impl RefEncode for MTLRenderStages {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// Arguments for an indirect non-indexed draw command.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct MTLDrawPrimitivesIndirectArguments {
    pub vertex_count: u32,
    pub instance_count: u32,
    pub vertex_start: u32,
    pub base_instance: u32,
}

unsafe impl Encode for MTLDrawPrimitivesIndirectArguments {
    const ENCODING: Encoding = Encoding::Struct("?", &[u32::ENCODING, u32::ENCODING, u32::ENCODING, u32::ENCODING]);
}

unsafe impl RefEncode for MTLDrawPrimitivesIndirectArguments {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// Arguments for an indirect indexed draw command.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct MTLDrawIndexedPrimitivesIndirectArguments {
    pub index_count: u32,
    pub instance_count: u32,
    pub index_start: u32,
    pub base_vertex: i32,
    pub base_instance: u32,
}

unsafe impl Encode for MTLDrawIndexedPrimitivesIndirectArguments {
    const ENCODING: Encoding =
        Encoding::Struct("?", &[u32::ENCODING, u32::ENCODING, u32::ENCODING, i32::ENCODING, u32::ENCODING]);
}

unsafe impl RefEncode for MTLDrawIndexedPrimitivesIndirectArguments {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MTLVertexAmplificationViewMapping {
    pub viewport_array_index_offset: u32,
    pub render_target_array_index_offset: u32,
}

unsafe impl Encode for MTLVertexAmplificationViewMapping {
    const ENCODING: Encoding = Encoding::Struct("?", &[u32::ENCODING, u32::ENCODING]);
}

unsafe impl RefEncode for MTLVertexAmplificationViewMapping {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// Arguments for an indirect tessellation patch draw command.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct MTLDrawPatchIndirectArguments {
    pub patch_count: u32,
    pub instance_count: u32,
    pub patch_start: u32,
    pub base_instance: u32,
}

unsafe impl Encode for MTLDrawPatchIndirectArguments {
    const ENCODING: Encoding = Encoding::Struct("?", &[u32::ENCODING, u32::ENCODING, u32::ENCODING, u32::ENCODING]);
}

unsafe impl RefEncode for MTLDrawPatchIndirectArguments {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// Half-precision tessellation factors for a quad patch.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct MTLQuadTessellationFactorsHalf {
    pub edge_tessellation_factor: [u16; 4],
    pub inside_tessellation_factor: [u16; 2],
}

unsafe impl Encode for MTLQuadTessellationFactorsHalf {
    const ENCODING: Encoding = Encoding::Struct("?", &[<[u16; 4]>::ENCODING, <[u16; 2]>::ENCODING]);
}

unsafe impl RefEncode for MTLQuadTessellationFactorsHalf {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

/// Half-precision tessellation factors for a triangle patch.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct MTLTriangleTessellationFactorsHalf {
    pub edge_tessellation_factor: [u16; 3],
    pub inside_tessellation_factor: u16,
}

unsafe impl Encode for MTLTriangleTessellationFactorsHalf {
    const ENCODING: Encoding = Encoding::Struct("?", &[<[u16; 3]>::ENCODING, u16::ENCODING]);
}

unsafe impl RefEncode for MTLTriangleTessellationFactorsHalf {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

#[cfg(test)]
mod tests {
    use core::mem::{align_of, offset_of, size_of};

    use super::*;

    #[test]
    fn draw_primitives_indirect_arguments_match_metal_abi() {
        assert_eq!(
            (
                size_of::<MTLDrawPrimitivesIndirectArguments>(),
                align_of::<MTLDrawPrimitivesIndirectArguments>(),
                offset_of!(MTLDrawPrimitivesIndirectArguments, vertex_count),
                offset_of!(MTLDrawPrimitivesIndirectArguments, instance_count),
                offset_of!(MTLDrawPrimitivesIndirectArguments, vertex_start),
                offset_of!(MTLDrawPrimitivesIndirectArguments, base_instance),
            ),
            (16, 4, 0, 4, 8, 12),
        );
    }

    #[test]
    fn draw_indexed_primitives_indirect_arguments_match_metal_abi() {
        assert_eq!(
            (
                size_of::<MTLDrawIndexedPrimitivesIndirectArguments>(),
                align_of::<MTLDrawIndexedPrimitivesIndirectArguments>(),
                offset_of!(MTLDrawIndexedPrimitivesIndirectArguments, index_count),
                offset_of!(MTLDrawIndexedPrimitivesIndirectArguments, instance_count),
                offset_of!(MTLDrawIndexedPrimitivesIndirectArguments, index_start),
                offset_of!(MTLDrawIndexedPrimitivesIndirectArguments, base_vertex),
                offset_of!(MTLDrawIndexedPrimitivesIndirectArguments, base_instance),
            ),
            (20, 4, 0, 4, 8, 12, 16),
        );
    }

    #[test]
    fn draw_patch_indirect_arguments_match_metal_abi() {
        assert_eq!(
            (
                size_of::<MTLDrawPatchIndirectArguments>(),
                align_of::<MTLDrawPatchIndirectArguments>(),
                offset_of!(MTLDrawPatchIndirectArguments, patch_count),
                offset_of!(MTLDrawPatchIndirectArguments, instance_count),
                offset_of!(MTLDrawPatchIndirectArguments, patch_start),
                offset_of!(MTLDrawPatchIndirectArguments, base_instance),
            ),
            (16, 4, 0, 4, 8, 12),
        );
    }

    #[test]
    fn quad_tessellation_factors_match_metal_abi() {
        assert_eq!(
            (
                size_of::<MTLQuadTessellationFactorsHalf>(),
                align_of::<MTLQuadTessellationFactorsHalf>(),
                offset_of!(MTLQuadTessellationFactorsHalf, edge_tessellation_factor),
                offset_of!(MTLQuadTessellationFactorsHalf, inside_tessellation_factor),
            ),
            (12, 2, 0, 8),
        );
    }

    #[test]
    fn triangle_tessellation_factors_match_metal_abi() {
        assert_eq!(
            (
                size_of::<MTLTriangleTessellationFactorsHalf>(),
                align_of::<MTLTriangleTessellationFactorsHalf>(),
                offset_of!(MTLTriangleTessellationFactorsHalf, edge_tessellation_factor),
                offset_of!(MTLTriangleTessellationFactorsHalf, inside_tessellation_factor),
            ),
            (8, 2, 0, 6),
        );
    }

    #[test]
    fn vertex_amplification_mapping_matches_metal_abi() {
        assert_eq!(
            (
                size_of::<MTLVertexAmplificationViewMapping>(),
                align_of::<MTLVertexAmplificationViewMapping>(),
                offset_of!(MTLVertexAmplificationViewMapping, viewport_array_index_offset),
                offset_of!(MTLVertexAmplificationViewMapping, render_target_array_index_offset),
            ),
            (8, 4, 0, 4),
        );
    }

    #[test]
    fn scissor_rect_matches_platform_nsuinteger_abi() {
        let word = size_of::<usize>();
        assert_eq!(
            (
                size_of::<MTLScissorRect>(),
                align_of::<MTLScissorRect>(),
                offset_of!(MTLScissorRect, x),
                offset_of!(MTLScissorRect, y),
                offset_of!(MTLScissorRect, width),
                offset_of!(MTLScissorRect, height),
            ),
            (4 * word, align_of::<usize>(), 0, word, 2 * word, 3 * word),
        );
    }

    #[test]
    fn viewport_matches_metal_abi() {
        assert_eq!(
            (
                size_of::<MTLViewport>(),
                align_of::<MTLViewport>(),
                offset_of!(MTLViewport, origin_x),
                offset_of!(MTLViewport, origin_y),
                offset_of!(MTLViewport, width),
                offset_of!(MTLViewport, height),
                offset_of!(MTLViewport, znear),
                offset_of!(MTLViewport, zfar),
            ),
            (48, align_of::<f64>(), 0, 8, 16, 24, 32, 40),
        );
    }
}
