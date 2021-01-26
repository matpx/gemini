use legion::Entity;

#[derive(Debug, Clone, Copy)]
pub struct TransformComponent {
    pub translation: glam::Vec3,
    pub scale: glam::Vec3,
    pub rotation: glam::Quat,
    pub local: glam::Mat4,
    pub world: glam::Mat4,
    pub parent: Option<Entity>,
    pub needs_update: bool,
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
            needs_update: true,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct MeshComponent {
    pub mesh_id: usize,
    pub material_id: usize,
}

#[derive(Debug, Clone, Copy)]
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
