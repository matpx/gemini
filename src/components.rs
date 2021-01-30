use slotmap::DefaultKey;
use wgpu::{util::DeviceExt, BindGroup, BindGroupLayout, Buffer, Device};

use crate::gpu::EntityUniform;

#[derive(Debug, Clone, Copy)]
pub struct TransformComponent {
    pub translation: glam::Vec3,
    pub scale: glam::Vec3,
    pub rotation: glam::Quat,
    pub local: glam::Mat4,
    pub world: glam::Mat4,
    pub parent: Option<DefaultKey>,
}

impl Default for TransformComponent {
    fn default() -> Self {
        Self {
            translation: glam::Vec3::zero(),
            scale: glam::Vec3::one(),
            rotation: glam::Quat::identity(),
            local: glam::Mat4::identity(),
            world: glam::Mat4::identity(),
            parent: None,
        }
    }
}

#[derive(Debug)]
pub struct MeshComponent {
    pub geometry_id: usize,
    pub pipeline_id: usize,
    pub local_buffer: Buffer,
    pub local_bind_group: BindGroup,
}

impl MeshComponent {
    pub fn new(
        device: &Device,
        local_bind_group_layout: &BindGroupLayout,
        geometry_id: usize,
        pipeline_id: usize,
    ) -> Self {
        let local_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::bytes_of(&EntityUniform {
                model: glam::Mat4::identity(),
            }),
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
        });

        let local_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &local_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(local_buffer.slice(..)),
            }],
            label: None,
        });

        MeshComponent {
            geometry_id,
            pipeline_id,
            local_buffer,
            local_bind_group,
        }
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct CameraComponent {
    pub fov: f32,
    pub aspect: f32,
    pub near: f32,
    pub far: f32,
    pub proj: glam::Mat4,
}

impl CameraComponent {
    pub fn new(fov: f32, aspect: f32, near: f32, far: f32) -> Self {
        Self {
            fov,
            aspect,
            near,
            far,
            proj: glam::Mat4::perspective_lh(fov, aspect, near, far),
        }
    }

    pub fn update_projection_matrix(&mut self) {
        self.proj = glam::Mat4::perspective_lh(self.fov, self.aspect, self.near, self.far);
    }
}
