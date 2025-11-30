use crate::algorithm::problem::Problem;
use crate::world::types::Ray;
use macroquad::{prelude::*, rand};

pub struct PsoStrategy {
    pub init_random_offset: f32,
    pub swarms_count: usize,
    pub inertia_weight: f64,
    pub local_factor: f64,
    pub global_factor: f64,
    pub iterate_count: usize,
    pub max_velocity: f32,
}

impl PsoStrategy {
    pub fn upgrade_path(&self, problem: &Problem, init_path: &[Vec2]) -> Vec<Vec2> {
        let mut swarms: Vec<Vec<Vec2>> = Vec::new();
        swarms.resize(self.swarms_count, Vec::new());
        swarms.iter_mut().for_each(|x| *x = self.gen_init_particle(&init_path));

        let mut swarms_velocity: Vec<Vec<Vec2>> = Vec::new();
        swarms_velocity.resize(self.swarms_count, Vec::new());
        swarms_velocity.iter_mut().for_each(|x| {
            x.resize(
                init_path.len(),
                Vec2::new(rand::gen_range(0.0, 50.0), rand::gen_range(0.0, 50.0)),
            )
        });

        let mut best_particle_sol: Vec<Vec<Vec2>> = swarms.clone();
        let mut best_particle_fitness: Vec<f64> = Vec::new();
        best_particle_fitness.reserve(best_particle_sol.len());
        for particle in best_particle_sol.iter() {
            best_particle_fitness.push(self.cal_fitness(problem, particle));
        }

        let global_best_idx = best_particle_fitness
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .map(|(idx, _)| idx)
            .unwrap();
        let mut global_best = best_particle_sol[global_best_idx].clone();
        let mut global_best_fitness = best_particle_fitness[global_best_idx];

        for _ in 0..self.iterate_count {
            for index in 0..swarms.len() {
                let particle = swarms.get_mut(index).unwrap();
                let pre_velocity = swarms_velocity.get_mut(index).unwrap();
                let local_best = &best_particle_sol[index];

                let new_velocity = self.cal_velocity(particle, pre_velocity, local_best, &global_best);

                *pre_velocity = new_velocity;

                for i in 1..particle.len() - 1 {
                    particle[i] = particle[i] + pre_velocity[i];
                }

                let fitness = self.cal_fitness(problem, particle);

                if fitness < best_particle_fitness[index] {
                    best_particle_fitness[index] = fitness;
                    best_particle_sol[index] = particle.clone();

                    if fitness < global_best_fitness {
                        global_best_fitness = fitness;
                        global_best = particle.clone();
                    }
                }
            }
        }

        global_best
    }

    fn gen_init_particle(&self, ref_path: &[Vec2]) -> Vec<Vec2> {
        let mut particle = Vec::new();

        if ref_path.is_empty() {
            return particle;
        }

        particle.push(ref_path[0]);

        for point in ref_path.iter().skip(1).take(ref_path.len() - 2) {
            let random_offset_x = (rand::gen_range(0.0, 1.0) - 0.5) * 2.0 * self.init_random_offset;
            let random_offset_y = (rand::gen_range(0.0, 1.0) - 0.5) * 2.0 * self.init_random_offset;

            let new_point = Vec2::new(point.x + random_offset_x, point.y + random_offset_y);

            particle.push(new_point);
        }

        if ref_path.len() > 1 {
            particle.push(ref_path[ref_path.len() - 1]);
        }

        particle
    }

    fn cal_fitness(&self, problem: &Problem, particle: &[Vec2]) -> f64 {
        if particle.len() < 2 {
            return 1e10;
        }

        let mut total_length = 0.0;

        for i in 0..particle.len() - 1 {
            let start = particle[i];
            let end = particle[i + 1];

            let ray = Ray {
                root: start,
                dir: end - start,
            };

            if let Some(_hit) = problem.grid_map.raycast(ray) {
                return 99999999.0;
            }

            total_length += (end - start).length() as f64;
        }

        total_length
    }

    fn cal_velocity(&self, particle: &[Vec2], pre_velocity: &[Vec2], local_best: &[Vec2], global_best: &[Vec2]) -> Vec<Vec2> {
        let mut res = Vec::new();
        res.reserve(pre_velocity.len());

        for index in 0..pre_velocity.len() {
            if index == 0 || index == pre_velocity.len() - 1 {
                res.push(Vec2::ZERO);
                continue;
            }

            let particle_i = particle[index];
            let pre_velocity_i = pre_velocity[index];
            let local_best_i = local_best[index];
            let global_best_i = global_best[index];

            let r1 = rand::gen_range(0.0, 1.0);
            let r2 = rand::gen_range(0.0, 1.0);

            let mut new_velocity = pre_velocity_i * self.inertia_weight as f32
                + (local_best_i - particle_i) * (self.local_factor as f32 * r1)
                + (global_best_i - particle_i) * (self.global_factor as f32 * r2);

            let velocity_magnitude = new_velocity.length();
            if velocity_magnitude > self.max_velocity {
                new_velocity = new_velocity.normalize() * self.max_velocity;
            }

            res.push(new_velocity);
        }

        res
    }
}
