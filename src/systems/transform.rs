use crate::components::TransformComponent;
use core::panic;
use legion::*;

pub struct TransformSystem;

impl TransformSystem {
    unsafe fn update_parent(world: &World, entity: Entity, depth: u32) -> glam::Mat4 {
        if depth > 64 {
            panic!("Max transform depth has been exceeded! This might be caused by a cycle in the transform hierarchy.");
        }

        let entity_ref = world.entry_ref(entity).unwrap();

        let mut transform = entity_ref
            .get_component_unchecked::<TransformComponent>()
            .unwrap();

        if transform.needs_update {
            if let Some(parent) = transform.parent {
                let parent_world = Self::update_parent(world, parent, depth + 1);

                transform.world = parent_world.mul_mat4(&transform.local);
            } else {
                transform.world = transform.local;
            }
        }

        transform.needs_update = false;

        transform.world
    }

    pub fn update(world: &mut World) {
        for mut transform in <&mut TransformComponent>::query().iter_mut(world) {
            transform.local = glam::Mat4::from_scale_rotation_translation(
                transform.scale,
                transform.rotation,
                transform.translation,
            );

            transform.needs_update = true;
        }

        unsafe {
            for mut transform in <&mut TransformComponent>::query().iter_unchecked(world) {
                if let Some(parent) = transform.parent {
                    let parent_world = Self::update_parent(world, parent, 0);

                    transform.world = parent_world.mul_mat4(&transform.local);
                } else {
                    transform.world = transform.local;
                }

                transform.needs_update = false;
            }
        }
    }
}
