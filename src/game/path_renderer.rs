use crate::game::game::RenderConfig;
use macroquad::prelude::*;

pub struct PathRenderer {
    config: RenderConfig,
    aco_path: Option<Vec<Vec2>>,
    pso_path: Option<Vec<Vec2>>,
}

impl PathRenderer {
    pub fn new(config: RenderConfig) -> Self {
        Self {
            config,
            aco_path: None,
            pso_path: None,
        }
    }

    pub fn set_aco_path(&mut self, path_points: Vec<Vec2>) {
        self.aco_path = Some(path_points);
    }

    pub fn set_pso_path(&mut self, path_points: Vec<Vec2>) {
        self.pso_path = Some(path_points);
    }

    pub fn unset_aco_path(&mut self) {
        self.aco_path = None;
    }

    pub fn unset_pso_path(&mut self) {
        self.pso_path = None;
    }

    pub fn unset_paths(&mut self) {
        self.aco_path = None;
        self.pso_path = None;
    }

    pub fn draw(&self) {
        // Draw ACO path in the original color
        if let Some(path) = &self.aco_path {
            if path.len() >= 2 {
                for i in 0..path.len() - 1 {
                    draw_line(
                        path[i].x,
                        path[i].y,
                        path[i + 1].x,
                        path[i + 1].y,
                        self.config.path_thickness,
                        self.config.path_color,
                    );
                }
            }
        }

        // Draw PSO path in a different color (blue)
        if let Some(path) = &self.pso_path {
            if path.len() >= 2 {
                for i in 0..path.len() - 1 {
                    draw_line(
                        path[i].x,
                        path[i].y,
                        path[i + 1].x,
                        path[i + 1].y,
                        self.config.path_thickness,
                        GREEN, // PSO path in blue
                    );
                }
            }
        }
    }
}
