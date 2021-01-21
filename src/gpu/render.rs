use crate::components::*;
use crate::scene::Scene;
use legion::*;
use wgpu::{BindGroup, Device, Queue, SwapChain};

pub fn render(
    device: &Device,
    swap_chain: &mut SwapChain,
    queue: &Queue,
    global_bind_group: &BindGroup,
    scene: &Scene,
) {
    let frame = swap_chain
        .get_current_frame()
        .expect("Failed to acquire next swap chain texture")
        .output;
    let mut encoder =
        device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    {
        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                attachment: &frame.view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: true,
                },
            }],
            depth_stencil_attachment: None,
        });

        for (mesh, material) in <(&MeshComponent, &MaterialComponent)>::query().iter(&scene.world) {
            let gpu_mesh = scene.meshes.get(mesh.mesh_id).unwrap();
            let gpu_material = scene.materials.get(material.material_id).unwrap();

            rpass.set_pipeline(&gpu_material.pipeline);
            rpass.set_bind_group(0, &global_bind_group, &[]);
            rpass.set_bind_group(1, &gpu_mesh.local_bind_group, &[]);
            rpass.set_index_buffer(gpu_mesh.index_buffer.slice(..));
            rpass.set_vertex_buffer(0, gpu_mesh.vertex_buffer.slice(..));
            rpass.draw_indexed(0..gpu_mesh.index_count, 0, 0..1);
        }
    }

    queue.submit(Some(encoder.finish()));
}
