use rapier2d::geometry::BroadPhaseMultiSap;
use rapier2d::prelude::*;
use wasm_bindgen::prelude::*;

mod liquid_sim;

struct PhysicsWorld {
    pipeline: PhysicsPipeline,
    bodies: RigidBodySet,
    colliders: ColliderSet,
    integration_parameters: IntegrationParameters,
    broad_phase: BroadPhaseMultiSap,
    narrow_phase: NarrowPhase,
    island_manager: IslandManager,
    ccd_solver: CCDSolver,
    impulse_joints: ImpulseJointSet,
    multibody_joints: MultibodyJointSet,
    query_pipeline: QueryPipeline,
}

impl PhysicsWorld {
    fn new() -> Self {
        Self {
            pipeline: PhysicsPipeline::new(),
            bodies: RigidBodySet::new(),
            colliders: ColliderSet::new(),
            integration_parameters: IntegrationParameters::default(),
            broad_phase: BroadPhaseMultiSap::new(),
            narrow_phase: NarrowPhase::new(),
            island_manager: IslandManager::new(),
            ccd_solver: CCDSolver::new(),
            impulse_joints: ImpulseJointSet::new(),
            multibody_joints: MultibodyJointSet::new(),
            query_pipeline: QueryPipeline::new(),
        }
    }

    fn step(&mut self, gravity: &Vector<f32>) {
        self.pipeline.step(
            gravity,
            &self.integration_parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.bodies,
            &mut self.colliders,
            &mut self.impulse_joints,
            &mut self.multibody_joints,
            &mut self.ccd_solver,
            Some(&mut self.query_pipeline),
            &(),
            &(),
        );
    }
}


#[wasm_bindgen]
pub fn calc(a: f64, b: f64, op: &str) -> f64 {
    match op {
        "+" => a + b,
        "-" => a - b,
        "*" => a * b,
        "/" => {
            if b == 0.0 {
                f64::NAN
            } else {
                a / b
            }
        }
        _ => f64::NAN,
    }
}

#[wasm_bindgen]
pub struct PhysicsSim {
    gravity: Vector<f32>,
    world: PhysicsWorld,
}

#[wasm_bindgen]
impl PhysicsSim {
    #[wasm_bindgen(constructor)]
    pub fn new() -> PhysicsSim {
        PhysicsSim {
            gravity: vector![0.0, -9.81],
            world: PhysicsWorld::new(),
        }
    }

    pub fn step(&mut self) {
        self.world.step(&self.gravity);
    }

    pub fn add_ball(&mut self, x: f32, y: f32) {
        let rigid_body = RigidBodyBuilder::dynamic()
            .translation(vector![x, y])
            .build();
        let handle = self.world.bodies.insert(rigid_body);
        let collider = ColliderBuilder::ball(0.5).build();
        self.world.colliders
            .insert_with_parent(collider, handle, &mut self.world.bodies);
    }

    pub fn get_positions(&self) -> js_sys::Array {
        let positions = js_sys::Array::new();
        for (_, body) in self.world.bodies.iter() {
            let pos = body.translation();
            let arr = js_sys::Array::of2(&pos.x.into(), &pos.y.into());
            positions.push(&arr);
        }
        positions
    }
}
