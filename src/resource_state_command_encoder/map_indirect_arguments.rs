use objc2::{Encode, Encoding, RefEncode};

/// Structure describing an indirect mapping region (from `MTLMapIndirectArguments`).
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MTLMapIndirectArguments {
    pub region_origin_x: u32,
    pub region_origin_y: u32,
    pub region_origin_z: u32,
    pub region_size_width: u32,
    pub region_size_height: u32,
    pub region_size_depth: u32,
    pub mip_map_level: u32,
    pub slice_id: u32,
}

unsafe impl Encode for MTLMapIndirectArguments {
    const ENCODING: Encoding = Encoding::Struct(
        "{MTLMapIndirectArguments=IIIIIIII}",
        &[
            u32::ENCODING,
            u32::ENCODING,
            u32::ENCODING,
            u32::ENCODING,
            u32::ENCODING,
            u32::ENCODING,
            u32::ENCODING,
            u32::ENCODING,
        ],
    );
}

unsafe impl RefEncode for MTLMapIndirectArguments {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}
