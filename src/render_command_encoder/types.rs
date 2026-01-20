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
    const ENCODING: Encoding = Encoding::Struct(
        "{MTLScissorRect=QQQQ}",
        &[
            usize::ENCODING,
            usize::ENCODING,
            usize::ENCODING,
            usize::ENCODING,
        ],
    );
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
        "{MTLViewport=dddddd}",
        &[
            f64::ENCODING,
            f64::ENCODING,
            f64::ENCODING,
            f64::ENCODING,
            f64::ENCODING,
            f64::ENCODING,
        ],
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

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MTLVertexAmplificationViewMapping {
    pub viewport_array_index_offset: u32,
    pub render_target_array_index_offset: u32,
}

unsafe impl Encode for MTLVertexAmplificationViewMapping {
    const ENCODING: Encoding = Encoding::Struct(
        "{MTLVertexAmplificationViewMapping=II}",
        &[u32::ENCODING, u32::ENCODING],
    );
}

unsafe impl RefEncode for MTLVertexAmplificationViewMapping {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
