use super::components::*;

pub struct Entity {
    pub transform: TransformComponent,
    pub mesh: MeshComponent,
    pub material: MaterialComponent,
    pub local_bind_group: wgpu::BindGroup,
}
