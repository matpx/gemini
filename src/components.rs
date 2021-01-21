#[derive(Debug, Clone, Copy)]
pub struct TransformComponent {
    pub position: glam::Vec3,
    pub scale: glam::Vec3,
    pub rotation: glam::Quat,
    pub model: glam::Mat4,
}

impl Default for TransformComponent {
    fn default() -> Self {
        Self {
            position: glam::Vec3::zero(),
            scale: glam::Vec3::one(),
            rotation: glam::Quat::identity(),
            model: glam::Mat4::identity(),
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
            proj: glam::Mat4::identity(),
        }
    }
}
