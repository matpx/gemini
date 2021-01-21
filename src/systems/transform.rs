use crate::components::TransformComponent;
use legion::*;

pub fn transform_system(world: &mut World) {
    for mut transform in <&mut TransformComponent>::query().iter_mut(world) {
        transform.model = glam::Mat4::from_scale_rotation_translation(
            transform.scale,
            transform.rotation,
            transform.position,
        );
    }
}
