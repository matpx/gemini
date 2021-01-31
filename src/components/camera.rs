#[derive(Default, Debug, Clone, Copy)]
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
