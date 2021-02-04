use crate::{
    components::{PlayerComponent, TransformComponent},
    input::InputManager,
    resources::scene::Scene,
};
use slotmap::DefaultKey;

pub struct PlayerSystem;

impl PlayerSystem {
    pub fn setup(scene: &mut Scene) -> DefaultKey {
        let player_entity = scene.create_entity(TransformComponent::default());

        scene
            .players
            .insert(player_entity, PlayerComponent::default());

        player_entity
    }

    pub fn update(scene: &mut Scene, input_manager: &InputManager, player_entity: DefaultKey) {
        let transform_component = scene.transforms.get_mut(player_entity).unwrap();
        let player_component = scene.players.get_mut(player_entity).unwrap();

        let speed_multiplier = 0.01;
        let rotation_multiplier = 0.01;

        player_component.x += input_manager.axis_b.x * rotation_multiplier;
        player_component.y += input_manager.axis_b.y * rotation_multiplier;

        let mut new_rotation =
            glam::Quat::from_axis_angle(glam::vec3(0.0, 1.0, 0.0), player_component.x);

        new_rotation *= glam::Quat::from_axis_angle(glam::vec3(1.0, 0.0, 0.0), player_component.y);

        transform_component.rotation = new_rotation;

        let mut add_translation = glam::vec3(
            input_manager.axis_a.x * speed_multiplier,
            0.0,
            input_manager.axis_a.y * speed_multiplier,
        );

        add_translation = transform_component.rotation * add_translation;

        transform_component.translation += add_translation;
    }
}
