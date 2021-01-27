use crate::{components::TransformComponent, scene::Scene};

pub struct TransformSystem;

impl TransformSystem {
    pub fn update(scene: &mut Scene) {
        for transform_entry in &scene.components.transforms {
            let transform =
                (transform_entry.1 as *const TransformComponent) as *mut TransformComponent;

            unsafe {
                (*transform).local = glam::Mat4::from_scale_rotation_translation(
                    (*transform).scale,
                    (*transform).rotation,
                    (*transform).translation,
                );

                if let Some(parent_id) = (*transform).parent {
                    if let Some(parent_transform) = scene.components.transforms.get(&parent_id) {
                        (*transform).world = parent_transform.world.mul_mat4(&(*transform).local);
                    } else {
                        (*transform).parent = None;

                        (*transform).world = (*transform).local;
                    }
                } else {
                    (*transform).world = (*transform).local;
                }
            }
        }
    }
}
