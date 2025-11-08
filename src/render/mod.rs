pub mod grid_render;

use macroquad::prelude::*;

#[derive(Debug, Clone)]
pub struct RenderConfig {
    pub background_color: Color,
    pub obstacle_color: Color,
}
