use crate::world::grid::*;
use macroquad::prelude::*;
use std::sync::Arc;

pub struct Problem {
    pub grid_map: Arc<GridMap>,
    pub start: Vec2,
    pub goal: Vec2,
}

impl Problem {
    pub fn new(grid_map: Arc<GridMap>, start: Vec2, end: Vec2) -> Self {
        Self {
            grid_map,
            start,
            goal: end,
        }
    }

    pub fn grid_map(&self) -> &GridMap {
        &self.grid_map
    }

    pub fn start(&self) -> Vec2 {
        self.start
    }

    pub fn goal(&self) -> Vec2 {
        self.goal
    }
}
