use rapier2d::{
    dynamics::{IntegrationParameters, JointSet, RigidBodySet},
    geometry::{BroadPhase, ColliderSet, NarrowPhase},
    na::Vector2,
    pipeline::PhysicsPipeline,
};

pub struct PhysicsWorld {
    pipeline: PhysicsPipeline,
    gravity: Vector2<f32>,
    integration_parameters: IntegrationParameters,
    broad_phase: BroadPhase,
    narrow_phase: NarrowPhase,
    bodies: RigidBodySet,
    colliders: ColliderSet,
    joints: JointSet,
    event_handler: (),
    time_accumulator: f32,
}

impl PhysicsWorld {
    pub fn new() -> Self {
        let pipeline = PhysicsPipeline::new();
        let gravity = Vector2::new(0.0, -9.81);
        let integration_parameters = IntegrationParameters::default();
        let broad_phase = BroadPhase::new();
        let narrow_phase = NarrowPhase::new();
        let bodies = RigidBodySet::new();
        let colliders = ColliderSet::new();
        let joints = JointSet::new();
        let event_handler = ();

        Self {
            pipeline,
            gravity,
            integration_parameters,
            broad_phase,
            narrow_phase,
            bodies,
            colliders,
            joints,
            event_handler,
            time_accumulator: 0.0,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        self.time_accumulator += delta_time;

        while self.time_accumulator > self.integration_parameters.dt {
            self.pipeline.step(
                &self.gravity,
                &self.integration_parameters,
                &mut self.broad_phase,
                &mut self.narrow_phase,
                &mut self.bodies,
                &mut self.colliders,
                &mut self.joints,
                None,
                None,
                &self.event_handler,
            );

            self.time_accumulator -= self.integration_parameters.dt;
        }
    }
}
