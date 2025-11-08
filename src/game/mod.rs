pub mod game_manager;
pub mod grid_render;
pub mod ui_manager;

use macroquad::prelude::*;

#[derive(Debug, Clone)]
pub struct RenderConfig {
    pub background_color: Color,
    pub obstacle_color: Color,
}
