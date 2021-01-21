use crate::components::TransformComponent;
use legion::*;

pub fn transform_system(world: &mut World) {
    unsafe {
        for mut transform in <&mut TransformComponent>::query().iter_unchecked(world) {
            if let Some(parent) = transform.parent {
                let parent_model = world
                    .entry_ref(parent)
                    .unwrap()
                    .get_component::<TransformComponent>()
                    .unwrap()
                    .model;

                transform.model =
                    parent_model.mul_mat4(&glam::Mat4::from_scale_rotation_translation(
                        transform.scale,
                        transform.rotation,
                        transform.translation,
                    ));
            } else {
                transform.model = glam::Mat4::from_scale_rotation_translation(
                    transform.scale,
                    transform.rotation,
                    transform.translation,
                );
            }
        }
    }
}
