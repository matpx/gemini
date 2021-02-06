#[derive(Debug, Default, Clone, Copy)]
pub struct PbrMaterial {
    pub color: glam::Vec4,
    pub color_texture: Option<usize>,
}
