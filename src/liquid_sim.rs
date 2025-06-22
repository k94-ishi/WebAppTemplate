use wasm_bindgen::prelude::*;
use rapier2d::geometry::BroadPhaseMultiSap;
use rapier2d::prelude::*;
use js_sys::Array;

#[wasm_bindgen]
pub struct Sim {
    pipeline: PhysicsPipeline,
    gravity: Vector<f32>,
    integration_parameters: IntegrationParameters,
    island_manager: IslandManager,
    broad_phase: BroadPhaseMultiSap,
    narrow_phase: NarrowPhase,
    bodies: RigidBodySet,
    colliders: ColliderSet,
    query_pipeline: QueryPipeline,
}

#[wasm_bindgen]
impl Sim {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Sim {
        let mut bodies = RigidBodySet::new();
        let mut colliders = ColliderSet::new();

        // 粒子の生成
        for i in 0..100 {
            let body = RigidBodyBuilder::dynamic()
                .translation(vector![i as f32 * 0.05, 5.0])
                .build();
            let handle = bodies.insert(body);
            let collider = ColliderBuilder::ball(0.02).build();
            colliders.insert_with_parent(collider, handle, &mut bodies);
        }

        // 障害物の生成
        let ground = RigidBodyBuilder::fixed().translation(vector![0.0, -1.0]).build();
        let ground_handle = bodies.insert(ground);
        let ground_collider = ColliderBuilder::cuboid(3.0, 0.1).build();
        colliders.insert_with_parent(ground_collider, ground_handle, &mut bodies);

        let wall = RigidBodyBuilder::fixed().translation(vector![0.5, 1.0]).rotation(0.3).build();
        let wall_handle = bodies.insert(wall);
        let wall_collider = ColliderBuilder::cuboid(1.0, 0.05).build();
        colliders.insert_with_parent(wall_collider, wall_handle, &mut bodies);

        Sim {
            pipeline: PhysicsPipeline::new(),
            gravity: vector![0.0, -9.81],
            integration_parameters: IntegrationParameters::default(),
            island_manager: IslandManager::new(),
            broad_phase: BroadPhaseMultiSap::new(),
            narrow_phase: NarrowPhase::new(),
            bodies,
            colliders,
            query_pipeline: QueryPipeline::new(),
        }
    }

    pub fn step(&mut self) {
        self.pipeline.step(
            &self.gravity,
            &self.integration_parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.bodies,
            &mut self.colliders,
            &mut ImpulseJointSet::new(),
            &mut MultibodyJointSet::new(),
            &mut CCDSolver::new(),
            Some(&mut self.query_pipeline),
            &(),
            &(),
        );
    }

    pub fn get_positions(&self) -> Array {
        let positions = Array::new();
        for (_, body) in self.bodies.iter() {
            let pos = body.translation();
            let arr = js_sys::Array::of2(&pos.x.into(), &pos.y.into());
            positions.push(&arr);
        }
        positions
    }
}
