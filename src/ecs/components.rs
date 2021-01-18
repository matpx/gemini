pub struct TransformComponent {
    position: glam::Vec3,
    scale: glam::Vec3,
    rotation: glam::Quat,
}

pub struct MeshComponent {
    mesh_id: usize,
}

pub struct MaterialComponent {
    pipeline_id: usize,
}
