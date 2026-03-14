use std::f32::consts::PI;

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct VertexData {
    pub position: [f32; 2],
    pub _padding: [f32; 2],
    pub color: [f32; 4],
}

pub fn write_rotating_triangle_vertices(
    rotation_degrees: f32,
    destination: *mut VertexData,
) {
    let radius: f32 = 350.0;
    let base_angle = rotation_degrees * PI / 180.0;

    let vertices = [
        VertexData {
            position: [radius * base_angle.cos(), radius * base_angle.sin()],
            _padding: [0.0; 2],
            color: [1.0, 0.0, 0.0, 1.0],
        },
        VertexData {
            position: [radius * (base_angle + 2.0 * PI / 3.0).cos(), radius * (base_angle + 2.0 * PI / 3.0).sin()],
            _padding: [0.0; 2],
            color: [0.0, 1.0, 0.0, 1.0],
        },
        VertexData {
            position: [radius * (base_angle + 4.0 * PI / 3.0).cos(), radius * (base_angle + 4.0 * PI / 3.0).sin()],
            _padding: [0.0; 2],
            color: [0.0, 0.0, 1.0, 1.0],
        },
    ];

    unsafe {
        std::ptr::copy_nonoverlapping(vertices.as_ptr(), destination, vertices.len());
    }
}
