use wasm_bindgen::prelude::*;
use js_sys::Array;
use rapier2d::prelude::*;
use crate::PhysicsWorld;

#[wasm_bindgen]
pub struct LiquidSim {
    gravity: Vector<f32>,
    world: PhysicsWorld,
    body_handles: Vec<RigidBodyHandle>,
    spring_pairs: Vec<(RigidBodyHandle, RigidBodyHandle)>,
}

#[wasm_bindgen]
impl LiquidSim {
    #[wasm_bindgen(constructor)]
    pub fn new() -> LiquidSim {
        let mut world = PhysicsWorld::new();
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
                let handle = world.bodies.insert(body);
                let collider = ColliderBuilder::ball(0.1)
                    .density(100.0)
                    .friction(0.1)
                    .build();
                world.colliders.insert_with_parent(collider, handle, &mut world.bodies);

                body_handles.push(handle);
            }
        }

        // 障害物: 地面
        let ground = RigidBodyBuilder::fixed()
            .translation(vector![100.0, 300.0])
            .build();
        let ground_handle = world.bodies.insert(ground);
        let ground_collider = ColliderBuilder::cuboid(100.0, 200.0).build();
        world.colliders.insert_with_parent(ground_collider, ground_handle, &mut world.bodies);

        // 障害物: 左壁
        let left_wall = RigidBodyBuilder::fixed()
            .translation(vector![10., 200.0])
            .build();
        let left_wall_handle = world.bodies.insert(left_wall);
        let left_collider = ColliderBuilder::cuboid(10.0, 300.0).build();
        world.colliders.insert_with_parent(left_collider, left_wall_handle, &mut world.bodies);

        // 障害物: 円形の障害物
        let circle_obs = RigidBodyBuilder::fixed()
            .translation(vector![250.0, 500.0])
            .build();
        let circle_handle = world.bodies.insert(circle_obs);
        let circle_collider = ColliderBuilder::ball(40.0).build();
        world.colliders.insert_with_parent(circle_collider, circle_handle, &mut world.bodies);

        // スプリング接続を事前計算
        let spring_pairs = Self::calculate_spring_pairs(&body_handles, cols);

        LiquidSim {
            gravity: vector![0.0, 98.0],
            world,
            body_handles,
            spring_pairs,
        }
    }

    // スプリング接続を事前計算（最適化）
    fn calculate_spring_pairs(body_handles: &[RigidBodyHandle], cols: usize) -> Vec<(RigidBodyHandle, RigidBodyHandle)> {
        let mut pairs = Vec::with_capacity(body_handles.len() * 2);

        for (i, &handle_a) in body_handles.iter().enumerate() {
            if i % cols != cols - 1 {
                pairs.push((handle_a, body_handles[i + 1]));
            }
            if i + cols < body_handles.len() {
                pairs.push((handle_a, body_handles[i + cols]));
            }
        }
        pairs
    }

    pub fn step(&mut self) {
        const STIFFNESS: f32 = 10.0;
        const DAMPING: f32 = 1.0;
        const REST_LENGTH: f32 = 5.0;
        const MAX_INTERACTIVE_DISTANCE: f32 = 10.0;

        // スプリング力を適用（事前計算されたペアを使用）
        for &(h1, h2) in &self.spring_pairs {
            let (first_handle, second_handle) = if h1.into_raw_parts().0 < h2.into_raw_parts().0 {
                (h1, h2)
            } else {
                (h2, h1)
            };
        
            if let (Some(body1), Some(body2)) = self.world.bodies.get_pair_mut(first_handle, second_handle) {
                let p1 = *body1.translation();
                let p2 = *body2.translation();
        
                let delta = p2 - p1;
                let dist = delta.norm();
                
                if dist > 0.0 && dist < MAX_INTERACTIVE_DISTANCE {
                    let dir = delta / dist;
                    let force_mag = STIFFNESS * (dist - REST_LENGTH);
                    let vel_rel = *body2.linvel() - *body1.linvel();
                    let damp = DAMPING * vel_rel.dot(&dir);
        
                    let force = (force_mag + damp) * dir;
        
                    body1.add_force(-force, true);
                    body2.add_force(force, true);
                }
            }
        }

        self.world.step(&self.gravity);
    }

    pub fn get_positions(&self) -> Array {
        let positions = Array::new();
        for handle in &self.body_handles {
            if let Some(body) = self.world.bodies.get(*handle) {
                let pos = body.translation();
                positions.push(&Array::of2(&pos.x.into(), &pos.y.into()));
            }
        }
        positions
    }

    pub fn get_colliders(&self) -> Array {
        let colliders = Array::new();
        for (_, col) in self.world.colliders.iter() {
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
