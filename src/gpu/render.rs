use super::{EntityUniform, GlobalUniform, UniformContext};
use crate::scene::Scene;
use slotmap::DefaultKey;
use wgpu::{Device, Queue, SwapChain};

pub fn render(
    device: &Device,
    queue: &Queue,
    swap_chain: &mut SwapChain,
    uniforms: &UniformContext,
    scene: &Scene,
    camera: DefaultKey,
) {
    let view_proj = {
        let camera_transform = scene.components.transforms.get(camera).unwrap();
        let camera_comp = scene.components.cameras.get(camera).unwrap();

        let proj = camera_comp.proj;
        let view = camera_transform.world.inverse();

        proj.mul_mat4(&view)
    };

    queue.write_buffer(
        &uniforms.global_uniform_buffer,
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
                attachment: &uniforms.depth_view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: false,
                }),
                stencil_ops: None,
            }),
        });

        for (entitiy_id, mesh) in scene.components.meshes.iter() {
            if let Some(transform) = scene.components.transforms.get(entitiy_id) {
                let geometry = scene.geometries.get(mesh.geometry_id).unwrap();
                let pipeline = scene.pipelines.get(mesh.pipeline_id).unwrap();

                queue.write_buffer(
                    &mesh.local_buffer,
                    0,
                    bytemuck::bytes_of(&EntityUniform {
                        model: transform.world,
                    }),
                );

                rpass.set_pipeline(&pipeline.pipeline);
                rpass.set_bind_group(0, &uniforms.global_bind_group, &[]);
                rpass.set_bind_group(1, &mesh.local_bind_group, &[]);
                rpass.set_index_buffer(geometry.index_buffer.slice(..));
                rpass.set_vertex_buffer(0, geometry.vertex_buffer.slice(..));
                rpass.draw_indexed(0..geometry.index_count, 0, 0..1);
            }
        }
    }

    queue.submit(Some(encoder.finish()));
}
