use bytemuck::{Pod, Zeroable};

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
