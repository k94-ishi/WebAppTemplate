use wasm_bindgen::prelude::*;
use js_sys::Array;
use rapier2d::prelude::*;
use rapier2d::geometry::BroadPhaseMultiSap;

#[wasm_bindgen]
pub struct LiquidSim {
    pipeline: PhysicsPipeline,
    gravity: Vector<f32>,
    integration_parameters: IntegrationParameters,
    bodies: RigidBodySet,
    colliders: ColliderSet,
    island_manager: IslandManager,
    broad_phase: BroadPhaseMultiSap,
    narrow_phase: NarrowPhase,
    ccd_solver: CCDSolver,
    impulse_joints: ImpulseJointSet,
    multibody_joints: MultibodyJointSet,
    query_pipeline: QueryPipeline,
    body_handles: Vec<RigidBodyHandle>,
}

#[wasm_bindgen]
impl LiquidSim {
    #[wasm_bindgen(constructor)]
    pub fn new() -> LiquidSim {
        let mut bodies = RigidBodySet::new();
        let mut colliders = ColliderSet::new();
        let impulse_joints = ImpulseJointSet::new();

        let mut body_handles = vec![];

        let cols = 25;
        let rows = 40;
        let spacing = 5.0;

        // 粒子作成
        for y in 0..rows {
            for x in 0..cols {
                let px = 100.0 + x as f32 * spacing;
                let py = 100.0 + y as f32 * spacing;

                let body = RigidBodyBuilder::dynamic()
                    .translation(vector![px, py])
                    .linear_damping(1.0)
                    .build();
                let handle = bodies.insert(body);
                let collider = ColliderBuilder::ball(0.1)
                    .density(100.0)
                    .friction(0.1)
                    .build();
                colliders.insert_with_parent(collider, handle, &mut bodies);

                body_handles.push(handle);
            }
        }

        // 障害物: 地面
        let ground = RigidBodyBuilder::fixed()
            .translation(vector![100.0, 300.0])
            .build();
        let ground_handle = bodies.insert(ground);
        let ground_collider = ColliderBuilder::cuboid(100.0, 200.0).build();
        colliders.insert_with_parent(ground_collider, ground_handle, &mut bodies);

        // 障害物: 左壁
        let left_wall = RigidBodyBuilder::fixed()
            .translation(vector![10., 200.0])
            .build();
        let left_wall_handle = bodies.insert(left_wall);
        let left_collider = ColliderBuilder::cuboid(10.0, 300.0).build();
        colliders.insert_with_parent(left_collider, left_wall_handle, &mut bodies);

        // 障害物: 円形の障害物
        let circle_obs = RigidBodyBuilder::fixed()
            .translation(vector![250.0, 500.0])
            .build();
        let circle_handle = bodies.insert(circle_obs);
        let circle_collider = ColliderBuilder::ball(40.0).build();
        colliders.insert_with_parent(circle_collider, circle_handle, &mut bodies);

        LiquidSim {
            pipeline: PhysicsPipeline::new(),
            gravity: vector![0.0, 98.0],
            integration_parameters: IntegrationParameters::default(),
            bodies,
            colliders,
            island_manager: IslandManager::new(),
            broad_phase: BroadPhaseMultiSap::new(),
            narrow_phase: NarrowPhase::new(),
            ccd_solver: CCDSolver::new(),
            impulse_joints,
            multibody_joints: MultibodyJointSet::new(),
            query_pipeline: QueryPipeline::new(),
            body_handles,
        }
    }

    // バネの補助メソッド
    fn body_pairs(&self) -> Vec<(RigidBodyHandle, RigidBodyHandle)> {
        let mut pairs = vec![];
        let cols = 25;

        for (i, &handle_a) in self.body_handles.iter().enumerate() {
            if i % cols != cols - 1 {
                pairs.push((handle_a, self.body_handles[i + 1]));
            }
            if i + cols < self.body_handles.len() {
                pairs.push((handle_a, self.body_handles[i + cols]));
            }
        }
        pairs
    }

    pub fn step(&mut self) {
        let stiffness = 10.0;
        let damping = 1.0;
        let rest_length = 5.;

        for pair in self.body_pairs() {
            let (h1, h2) = pair;
            if h1 != h2 {
                let (first_handle, second_handle) = if h1.into_raw_parts().0 < h2.into_raw_parts().0 {
                    (h1, h2)
                } else {
                    (h2, h1)
                };
            
                let (body1, body2) = {
                    let (first, rest) = self.bodies.get_pair_mut(first_handle, second_handle);
                    (first.unwrap(), rest.unwrap())
                };
            
        
                let p1 = body1.translation();
                let p2 = body2.translation();
        
                let delta = p2 - p1;
                let dist = delta.norm();
                let max_interactive_distance: f32 = 10.0;
                if dist > 0.0 && dist < max_interactive_distance {
                    let dir = delta / dist;
                    let force_mag = stiffness * (dist - rest_length);
                    let vel_rel = body2.linvel() - body1.linvel();
                    let damp = damping * vel_rel.dot(&dir);
        
                    let force = (force_mag + damp) * dir;
        
                    body1.add_force(-force, true);
                    body2.add_force(force, true);
                } 
            }
        }

        self.pipeline.step(
            &self.gravity,
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

    pub fn get_positions(&self) -> Array {
        let positions = Array::new();
        for handle in &self.body_handles {
            if let Some(body) = self.bodies.get(*handle) {
                let pos = body.translation();
                positions.push(&Array::of2(&pos.x.into(), &pos.y.into()));
            }
        }
        positions
    }

    pub fn get_colliders(&self) -> Array {
        let colliders = Array::new();
        for (_, col) in self.colliders.iter() {
            let pos = col.position().translation.vector;
            if let Some(ball) = col.shape().as_ball() {
                let arr = Array::of4(&pos.x.into(), &pos.y.into(), &ball.radius.into(), &"ball".into());
                colliders.push(&arr);
            } else if let Some(cuboid) = col.shape().as_cuboid() {
                let hx = cuboid.half_extents.x;
                let hy = cuboid.half_extents.y;
                let arr = Array::of5(&pos.x.into(), &pos.y.into(), &hx.into(), &hy.into(), &"cuboid".into());
                colliders.push(&arr);
            }
        }
        colliders
    }
}
