use std::f32::consts::PI;

use legion::Entity;

#[derive(Debug, Clone, Copy)]
pub struct TransformComponent {
    pub translation: glam::Vec3,
    pub scale: glam::Vec3,
    pub rotation: glam::Quat,
    pub local: glam::Mat4,
    pub world: glam::Mat4,
    pub parent: Option<Entity>,
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

#[derive(Debug, Clone, Copy)]
pub struct MeshComponent {
    pub mesh_id: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct MaterialComponent {
    pub material_id: usize,
}

#[derive(Debug, Clone, Copy)]
pub struct CameraComponent {
    pub proj: glam::Mat4,
}

impl Default for CameraComponent {
    fn default() -> Self {
        Self {
            proj: glam::Mat4::perspective_lh(PI/4.0, 1.0, 0.1, 100.0),
        }
    }
}
