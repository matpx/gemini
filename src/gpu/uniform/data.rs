use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct CameraUniformData {
    pub view_proj: glam::Mat4,
}

unsafe impl Pod for CameraUniformData {}
unsafe impl Zeroable for CameraUniformData {}

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct TransformUniformData {
    pub model: glam::Mat4,
}

unsafe impl Pod for TransformUniformData {}
unsafe impl Zeroable for TransformUniformData {}
