use bytemuck::{Pod, Zeroable};
use wgpu::{BindGroup, BindGroupLayout, Buffer, TextureView};

pub struct UniformContext {
    pub local_bind_group_layout: BindGroupLayout,
    pub global_bind_group_layout: BindGroupLayout,
    pub global_bind_group: BindGroup,
    pub global_uniform_buffer: Buffer,
    pub depth_view: TextureView,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct GlobalUniform {
    pub view_proj: glam::Mat4,
}

unsafe impl Pod for GlobalUniform {}
unsafe impl Zeroable for GlobalUniform {}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct EntityUniform {
    pub model: glam::Mat4,
}

unsafe impl Pod for EntityUniform {}
unsafe impl Zeroable for EntityUniform {}
