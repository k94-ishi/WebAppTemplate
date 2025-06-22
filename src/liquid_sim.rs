use rand::Rng;
use wasm_bindgen::prelude::*;
use js_sys::Array;

#[wasm_bindgen]
pub struct Particle {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
}

#[wasm_bindgen]
pub struct Obstacle {
    x: f32,
    y: f32,
    radius: f32,
}

#[wasm_bindgen]
pub struct LiquidSim {
    particles: Vec<Particle>,
    obstacles: Vec<Obstacle>,
}

#[wasm_bindgen]
impl LiquidSim {
    #[wasm_bindgen(constructor)]
    pub fn new() -> LiquidSim {
        let mut particles = vec![];
        let mut rng = rand::rng();
        for _ in 0..500 {
            let dx: f32 = rng.random::<f32>();
            let dy: f32 = rng.random::<f32>();
            let dvx: f32 = rng.random::<f32>();
            let dvy: f32 = rng.random::<f32>();
            particles.push(Particle {
                x: 1000.0 + dx * 100.0,
                y: 0.0 + dy * 100.0,
                vx: 0.0 + dvx * 100.0,
                vy: 0.0 + dvy * 100.0,
            });
        }

        let obstacles = vec![
            Obstacle { x: 500.0, y: 200.0, radius: 500.0 },
            Obstacle { x: 0.0, y: 400.0, radius: 500.0 },
        ];

        LiquidSim { particles, obstacles }
    }

    pub fn step(&mut self, dt: f32) {
        let gravity = 100.0;
        let spring_k = 10.0;
        let rest_length = 5.0;

        // バネ的な力 (粒子ペアに対して適用)
        for i in 0..self.particles.len() {
            for j in (i+1)..self.particles.len() {
                let (pi, pj) = {
                    let (head, tail) = self.particles.split_at_mut(j);
                    (&mut head[i], &mut tail[0])
                };

                let dx = pj.x - pi.x;
                let dy = pj.y - pi.y;
                let dist = (dx * dx + dy * dy).sqrt();
                if dist > 0.0 {
                    let force = spring_k * (dist - rest_length);
                    let fx = force * dx / dist;
                    let fy = force * dy / dist;

                    pi.vx += fx * dt;
                    pi.vy += fy * dt;
                    pj.vx -= fx * dt;
                    pj.vy -= fy * dt;
                }
            }
        }

        // 粒子の移動と重力
        for p in &mut self.particles {
            p.vy += gravity * dt;
            p.x += p.vx * dt;
            p.y += p.vy * dt;
        }

        // 障害物との衝突
        for p in &mut self.particles {
            for obs in &self.obstacles {
                let dx = p.x - obs.x;
                let dy = p.y - obs.y;
                let dist = (dx * dx + dy * dy).sqrt();

                if dist < obs.radius {
                    let overlap = obs.radius - dist;
                    let nx = dx / dist;
                    let ny = dy / dist;

                    p.x += nx * overlap;
                    p.y += ny * overlap;

                    // 簡単な反射
                    let dot = p.vx * nx + p.vy * ny;
                    p.vx -= 2.0 * dot * nx;
                    p.vy -= 2.0 * dot * ny;
                }
            }
        }
    }

    pub fn get_positions(&self) -> Array {
        let positions = Array::new();
        for p in &self.particles {
            let pair = Array::of2(&p.x.into(), &p.y.into());
            positions.push(&pair);
        }
        positions
    }

    pub fn get_obstacles(&self) -> Array {
        let obstacles = Array::new();
        for o in &self.obstacles {
            let arr = Array::of3(&o.x.into(), &o.y.into(), &o.radius.into());
            obstacles.push(&arr);
        }
        obstacles
    }
}
