use crate::components::*;
use crate::scene::Scene;
use legion::*;
use wgpu::{BindGroup, Buffer, Device, Queue, SwapChain, TextureView};

use super::{EntityUniform, GlobalUniform};

pub fn render(
    device: &Device,
    swap_chain: &mut SwapChain,
    queue: &Queue,
    global_bind_group: &BindGroup,
    global_uniform_buffer: &Buffer,
    depth_view: &TextureView,
    scene: &Scene,
    camera: Entity,
) {
    let view_proj = {
        let camera_entry = scene.world.entry_ref(camera).unwrap();

        let camera_transform = camera_entry.get_component::<TransformComponent>().unwrap();
        let camera_comp = camera_entry.get_component::<CameraComponent>().unwrap();

        let proj = camera_comp.proj;
        let view = camera_transform.world.inverse();

        proj.mul_mat4(&view)
    };

    queue.write_buffer(
        global_uniform_buffer,
        0,
        bytemuck::bytes_of(&GlobalUniform { view_proj }),
    );

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
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachmentDescriptor {
                attachment: &depth_view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: false,
                }),
                stencil_ops: None,
            }),
        });

        for (transform, mesh) in <(&TransformComponent, &MeshComponent)>::query().iter(&scene.world)
        {
            let gpu_mesh = scene.meshes.get(mesh.mesh_id).unwrap();
            let gpu_material = scene.pipelines.get(mesh.material_id).unwrap();

            queue.write_buffer(
                &gpu_mesh.local_buffer,
                0,
                bytemuck::bytes_of(&EntityUniform {
                    model: transform.world,
                }),
            );

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
