use crate::game::game::RenderConfig;
use macroquad::prelude::*;

pub struct PathRenderer {
    config: RenderConfig,
    path_points: Option<Vec<Vec2>>,
}

impl PathRenderer {
    pub fn new(config: RenderConfig) -> Self {
        Self {
            config,
            path_points: None,
        }
    }

    pub fn set_path(&mut self, path_points: Vec<Vec2>) {
        self.path_points = Some(path_points);
    }

    pub fn unset_path(&mut self) {
        self.path_points = None;
    }

    pub fn draw(&self) {
        if let Some(path) = &self.path_points {
            if path.len() < 2 {
                return;
            }

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
}
