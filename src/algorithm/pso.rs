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
}

impl PsoStrategy {
    pub fn upgrade_path(&self, problem: &Problem, init_path: &[Vec2]) -> Vec<Vec2> {
        let mut swarms: Vec<Vec<Vec2>> = Vec::new();
        swarms.resize(self.swarms_count, Vec::new());
        swarms.iter_mut().for_each(|x| *x = self.gen_init_particle(&init_path));

        let mut swarms_velocity: Vec<Vec<Vec2>> = Vec::new();
        swarms_velocity.resize(self.swarms_count, Vec::new());
        swarms_velocity.iter_mut().for_each(|x| x.resize(init_path.len(), Vec2::ZERO));

        let mut best_particle_sol: Vec<Vec<Vec2>> = swarms.clone();
        let mut best_particle_fitness: Vec<f64> = Vec::new();
        best_particle_fitness.reserve(best_particle_sol.len());
        for particle in best_particle_sol.iter() {
            best_particle_fitness.push(self.cal_fitness(problem, particle));
        }

        for _ in 0..self.iterate_count {
            for index in 0..swarms.len() {
                let particle = swarms.get_mut(index).unwrap();
                let pre_velocity = swarms_velocity.get_mut(index).unwrap();
                let global_best = self.cal_global_best(problem, &best_particle_sol);
                let local_best = best_particle_sol.get_mut(index).unwrap();

                let velocity = self.cal_velocity(particle, pre_velocity, local_best, &global_best);

                particle
                    .iter_mut()
                    .enumerate()
                    .for_each(|(i, x)| *x = *x + *velocity.get(i).unwrap());
                let fitness = self.cal_fitness(problem, particle);

                *pre_velocity = velocity;
                if fitness < *best_particle_fitness.get(index).unwrap() {
                    *best_particle_fitness.get_mut(index).unwrap() = fitness;
                    *local_best = particle.clone();
                }
            }
        }

        self.cal_global_best(problem, &best_particle_sol)
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
            return f64::INFINITY;
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
                return f64::INFINITY;
            }

            total_length += (end - start).length() as f64;
        }

        total_length
    }

    fn cal_global_best(&self, problem: &Problem, best_particle_sol: &[Vec<Vec2>]) -> Vec<Vec2> {
        let mut global_best_particle = best_particle_sol.first().unwrap();
        let mut best_fitness = f64::INFINITY;
        for particle in best_particle_sol.iter() {
            let fitness = self.cal_fitness(problem, particle);
            if fitness < best_fitness {
                best_fitness = fitness;
                global_best_particle = particle;
            }
        }
        global_best_particle.clone()
    }

    fn cal_velocity(&self, particle: &[Vec2], pre_velocity: &[Vec2], local_best: &[Vec2], global_best: &[Vec2]) -> Vec<Vec2> {
        let mut res = Vec::new();
        res.reserve(pre_velocity.len());

        for index in 0..pre_velocity.len() {
            let particle_i = particle.get(index).unwrap().clone();
            let pre_velocity_i = pre_velocity.get(index).unwrap().clone();
            let local_best_i = local_best.get(index).unwrap().clone();
            let global_best_i = global_best.get(index).unwrap().clone();
            res.push({
                pre_velocity_i * self.inertia_weight as f32
                    + (local_best_i - particle_i) * (self.local_factor as f32 * rand::gen_range(0.0, 1.0))
                    + (global_best_i - particle_i) * (self.global_factor as f32 * rand::gen_range(0.0, 1.0))
            });
        }

        res
    }
}
