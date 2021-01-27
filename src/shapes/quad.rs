use crate::gpu::{Geometry, Vertex};
use wgpu::{BindGroupLayout, Device};

pub fn _generate_quad(device: &Device, local_bind_group_layout: &BindGroupLayout) -> Geometry {
    let vertex_data = [
        Vertex {
            position: [-1.0, 1.0, 0.0],
            uv: [-1.0, 1.0],
            normal: [0.0, 0.0, 1.0],
        },
        Vertex {
            position: [1.0, 1.0, 0.0],
            uv: [1.0, 1.0],
            normal: [0.0, 0.0, 1.0],
        },
        Vertex {
            position: [1.0, -1.0, 0.0],
            uv: [1.0, -1.0],
            normal: [0.0, 0.0, 1.0],
        },
        Vertex {
            position: [-1.0, -1.0, 0.0],
            uv: [-1.0, -1.0],
            normal: [0.0, 0.0, 1.0],
        },
    ];

    let index_data: &[u32] = &[0, 1, 2, 2, 3, 0];

    Geometry::new(device, local_bind_group_layout, &vertex_data, index_data)
}
