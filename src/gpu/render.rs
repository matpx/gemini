use super::uniform::{
    CameraUniformData, PrimitiveUniformData, TransformUniformData, UniformContext,
    BUFFER_ENTITIES_NUM,
};
use crate::resources::{manager::ResourceManager, scene::Scene};
use slotmap::DefaultKey;
use wgpu::{Device, Queue, SwapChain};

pub fn render(
    device: &Device,
    queue: &Queue,
    resource_manager: &ResourceManager,
    swap_chain: &mut SwapChain,
    uniforms: &UniformContext,
    scene: &Scene,
    camera: DefaultKey,
) {
    let view_proj = {
        let camera_transform = scene.transforms.get(camera).unwrap();
        let camera_comp = scene.cameras.get(camera).unwrap();

        let proj = camera_comp.proj;
        let view = camera_transform.world.inverse();

        proj.mul_mat4(&view)
    };

    queue.write_buffer(
        &uniforms.camera_uniform_buffer,
        0,
        bytemuck::bytes_of(&CameraUniformData { view_proj }),
    );

    let frame = swap_chain
        .get_current_frame()
        .expect("Failed to acquire next swap chain texture")
        .output;
    let mut encoder =
        device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    {
        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
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

        let mut transform_counter: u32 = 0;
        let mut primitive_counter: u32 = 0;

        for (entitiy_id, mesh) in scene.meshes.iter() {
            if let Some(transform) = scene.transforms.get(entitiy_id) {
                assert!(transform_counter < BUFFER_ENTITIES_NUM as wgpu::DynamicOffset);

                let transform_offset: wgpu::DynamicOffset =
                    transform_counter * wgpu::BIND_BUFFER_ALIGNMENT as wgpu::DynamicOffset;

                queue.write_buffer(
                    &uniforms.transform_uniform_buffer,
                    transform_offset as wgpu::BufferAddress,
                    bytemuck::bytes_of(&TransformUniformData {
                        model: transform.world,
                    }),
                );

                for primitive in &mesh.primitives {
                    assert!(primitive_counter < BUFFER_ENTITIES_NUM as wgpu::DynamicOffset);

                    let primitive_offset: wgpu::DynamicOffset =
                        primitive_counter * wgpu::BIND_BUFFER_ALIGNMENT as wgpu::DynamicOffset;

                    let geometry = resource_manager
                        .geometries
                        .get(primitive.geometry_id)
                        .unwrap();
                    let pipeline = resource_manager
                        .pipelines
                        .get(primitive.pipeline_id)
                        .unwrap();
                    if let Some(texture_key) = primitive.material.color_texture {
                        let color_texture = resource_manager.texture.get(texture_key).unwrap();

                        rpass.set_bind_group(3, &color_texture.bind_group, &[]);
                    } else {
                        rpass.set_bind_group(3, &uniforms.dummy_texture.bind_group, &[]);
                    }

                    queue.write_buffer(
                        &uniforms.primitive_uniform_buffer,
                        primitive_offset as wgpu::BufferAddress,
                        bytemuck::bytes_of(&PrimitiveUniformData {
                            color: primitive.material.color,
                        }),
                    );

                    rpass.set_pipeline(&pipeline.pipeline);
                    rpass.set_bind_group(0, &uniforms.camera_bind_group, &[]);
                    rpass.set_bind_group(1, &uniforms.transform_bind_group, &[transform_offset]);
                    rpass.set_bind_group(2, &uniforms.primitive_bind_group, &[primitive_offset]);
                    rpass.set_index_buffer(geometry.index_buffer.slice(..), super::INDEX_FORMAT);
                    rpass.set_vertex_buffer(0, geometry.vertex_buffer.slice(..));
                    rpass.draw_indexed(0..geometry.index_count, 0, 0..1);

                    primitive_counter += 1;
                }

                transform_counter += 1;
            }
        }
    }

    queue.submit(Some(encoder.finish()));
}
