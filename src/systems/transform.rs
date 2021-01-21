use crate::components::TransformComponent;
use legion::*;

pub fn transform_system(world: &mut World) {
    unsafe {
        for mut transform in <&mut TransformComponent>::query().iter_unchecked(world) {
            transform.local = glam::Mat4::from_scale_rotation_translation(
                transform.scale,
                transform.rotation,
                transform.translation,
            );

            if let Some(parent) = transform.parent {
                let parent_world = world
                    .entry_ref(parent)
                    .unwrap()
                    .get_component::<TransformComponent>()
                    .unwrap()
                    .world;

                transform.world = parent_world.mul_mat4(&transform.local);
            } else {
                transform.world = transform.local;
            }
        }
    }
}
