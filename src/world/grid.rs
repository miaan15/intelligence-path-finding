use crate::world::types::{quad, Quad, Ray, RayHitInfo};
use crate::world::WorldConfig;
use macroquad::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GridNodeValue {
    Air,
    Obstacle,
}

#[derive(Debug)]
pub struct Grid {
    pub(crate) width: usize,
    pub(crate) height: usize,
    data: Vec<GridNodeValue>,
}

pub struct GridMap {
    grid: Box<Grid>,
    pub(crate) config: WorldConfig,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width: width,
            height: height,
            data: vec![GridNodeValue::Air; width * height],
        }
    }

    pub fn set(&mut self, x: usize, y: usize, val: GridNodeValue) -> Option<()> {
        let pos = self.get_grid_pos(x, y)?;
        self.data[pos] = val;
        Some(())
    }

    pub fn get(&self, x: usize, y: usize) -> Option<GridNodeValue> {
        let pos = self.get_grid_pos(x, y)?;
        Some(self.data[pos])
    }

    pub fn is_air(&self, x: usize, y: usize) -> Option<bool> {
        let pos = self.get_grid_pos(x, y)?;
        Some(self.data[pos] == GridNodeValue::Air)
    }

    fn get_grid_pos(&self, x: usize, y: usize) -> Option<usize> {
        if x >= self.width || y >= self.height {
            return None;
        }

        Some(y * self.width + x)
    }
}

impl GridMap {
    pub fn new(width: usize, height: usize, config: WorldConfig) -> Self {
        let grid = Grid::new(width, height);
        Self {
            grid: Box::new(grid),
            config: config,
        }
    }

    pub fn grid(&self) -> &Grid { self.grid.as_ref() }
    pub fn grid_mut(&mut self) -> &mut Grid { self.grid.as_mut() }

    pub fn width(&self) -> usize { self.grid.width }
    pub fn height(&self) -> usize { self.grid.height }
    pub fn cell_size(&self) -> f32 { self.config.cell_size }

    pub fn get_quad(&self, x: usize, y: usize) -> Option<Quad> {
        if x >= self.grid.width || y >= self.grid.height {
            return None;
        }

        Some(quad(
            x as f32 * self.config.cell_size,
            y as f32 * self.config.cell_size,
            self.config.cell_size,
            self.config.cell_size,
        ))
    }

    pub fn raycast(&self, ray: Ray) -> Option<RayHitInfo> {
        let dir_norm = ray.dir.normalize_or_zero();
        if dir_norm == Vec2::new(0.0, 0.0) {
            return None;
        }

        let mut t_min = 0.0;

        let grid_width = self.grid.width as f32 * self.config.cell_size;
        let grid_height = self.grid.height as f32 * self.config.cell_size;

        let bound_min = Vec2::new(0.0, 0.0);
        let bound_max = Vec2::new(grid_width, grid_height);

        if let Some(t_bounds) = self.box_intersection(ray.root, dir_norm, bound_min, bound_max) {
            if t_bounds < t_min {
                return None;
            }
        } else {
            return None;
        }

        let cell_size = self.config.cell_size;
        let mut grid_x = (ray.root.x / cell_size).floor() as isize;
        let mut grid_y = (ray.root.y / cell_size).floor() as isize;

        grid_x = grid_x.clamp(0, self.grid.width as isize - 1);
        grid_y = grid_y.clamp(0, self.grid.height as isize - 1);

        let step_x = if dir_norm.x > 0.0 {
            1
        } else if dir_norm.x < 0.0 {
            -1
        } else {
            0
        };
        let step_y = if dir_norm.y > 0.0 {
            1
        } else if dir_norm.y < 0.0 {
            -1
        } else {
            0
        };

        let t_delta_x = if dir_norm.x != 0.0 {
            cell_size / dir_norm.x.abs()
        } else {
            f32::MAX
        };

        let t_delta_y = if dir_norm.y != 0.0 {
            cell_size / dir_norm.y.abs()
        } else {
            f32::MAX
        };

        let mut t_max_x = if dir_norm.x > 0.0 {
            ((grid_x as f32 + 1.0) * cell_size - ray.root.x) / dir_norm.x
        } else if dir_norm.x < 0.0 {
            (grid_x as f32 * cell_size - ray.root.x) / dir_norm.x
        } else {
            f32::MAX
        };

        let mut t_max_y = if dir_norm.y > 0.0 {
            ((grid_y as f32 + 1.0) * cell_size - ray.root.y) / dir_norm.y
        } else if dir_norm.y < 0.0 {
            (grid_y as f32 * cell_size - ray.root.y) / dir_norm.y
        } else {
            f32::MAX
        };

        while grid_x >= 0 && grid_x < self.grid.width as isize && grid_y >= 0 && grid_y < self.grid.height as isize {
            if let Some(GridNodeValue::Obstacle) = self.grid.get(grid_x as usize, grid_y as usize) {
                let hit_pos = ray.root + dir_norm * t_min;

                let cell_center = Vec2::new(
                    grid_x as f32 * cell_size + cell_size * 0.5,
                    grid_y as f32 * cell_size + cell_size * 0.5,
                );
                let to_center = (cell_center - hit_pos).normalize_or_zero();

                let rel_pos = hit_pos - Vec2::new(grid_x as f32 * cell_size, grid_y as f32 * cell_size);
                let normal = if rel_pos.x < cell_size * 0.1 {
                    Vec2::new(-1.0, 0.0)
                } else if rel_pos.x > cell_size * 0.9 {
                    Vec2::new(1.0, 0.0)
                } else if rel_pos.y < cell_size * 0.1 {
                    Vec2::new(0.0, -1.0)
                } else if rel_pos.y > cell_size * 0.9 {
                    Vec2::new(0.0, 1.0)
                } else {
                    to_center
                };

                return Some(RayHitInfo {
                    pt: hit_pos,
                    nor: normal,
                    dist: t_min,
                });
            }

            if t_max_x < t_max_y {
                t_min = t_max_x;
                t_max_x += t_delta_x;
                grid_x += step_x;
            } else {
                t_min = t_max_y;
                t_max_y += t_delta_y;
                grid_y += step_y;
            }
        }

        None
    }

    fn box_intersection(&self, ray_root: Vec2, ray_dir: Vec2, box_min: Vec2, box_max: Vec2) -> Option<f32> {
        let mut t1 = (box_min.x - ray_root.x) / ray_dir.x;
        let mut t2 = (box_max.x - ray_root.x) / ray_dir.x;

        if t1 > t2 {
            std::mem::swap(&mut t1, &mut t2);
        }

        let mut t3 = (box_min.y - ray_root.y) / ray_dir.y;
        let mut t4 = (box_max.y - ray_root.y) / ray_dir.y;

        if t3 > t4 {
            std::mem::swap(&mut t3, &mut t4);
        }

        let t_min = t1.max(t3).max(0.0);
        let t_max = t2.min(t4);

        if t_min <= t_max {
            Some(t_min)
        } else {
            None
        }
    }
}
