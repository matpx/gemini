#[derive(Debug, Clone, Copy, Default)]
pub struct TransformComponent {
    pub position: glam::Vec3,
    pub scale: glam::Vec3,
    pub rotation: glam::Quat,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct MeshComponent {
    pub mesh_id: usize,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct MaterialComponent {
    pub material_id: usize,
}
