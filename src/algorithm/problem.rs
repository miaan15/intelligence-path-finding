use crate::world::grid::*;
use macroquad::prelude::*;
use std::sync::Arc;

pub struct Problem {
    pub grid_map: Arc<GridMap>,
    pub start: Vec2,
    pub end: Vec2,
}

impl Problem {
    pub fn grid_map(&self) -> &GridMap { &self.grid_map }

    pub fn start(&self) -> Vec2 { self.start }

    pub fn end(&self) -> Vec2 { self.end }
}
