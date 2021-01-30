use crate::gpu::{Geometry, Vertex};
use wgpu::Device;

pub fn _generate_quad(device: &Device) -> Geometry {
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

    Geometry::new(device, &vertex_data, index_data)
}
