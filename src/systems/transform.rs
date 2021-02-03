use crate::resources::scene::Scene;
pub struct TransformSystem;

impl TransformSystem {
    pub fn update(scene: &mut Scene) {
        let transforms = &mut scene.components.transforms;

        scene.components.transforms_sorted.retain(|&key| {
            let parent_world;
            let parent_key;

            if let Some(transform) = transforms.get_mut(key) {
                transform.local = glam::Mat4::from_scale_rotation_translation(
                    transform.scale,
                    transform.rotation,
                    transform.translation,
                );

                if let Some(p) = transform.parent {
                    parent_key = p;
                } else {
                    transform.world = transform.local;

                    return true;
                }
            } else {
                return false;
            }

            if let Some(parent) = transforms.get(parent_key) {
                parent_world = parent.world;
            } else {
                let transform = transforms.get_mut(key).unwrap();

                transform.parent = None;

                transform.world = transform.local;

                return true;
            }

            let transform = transforms.get_mut(key).unwrap();

            transform.world = parent_world.mul_mat4(&transform.local);

            true
        });
    }
}
