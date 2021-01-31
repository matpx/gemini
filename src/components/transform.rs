use slotmap::DefaultKey;

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
