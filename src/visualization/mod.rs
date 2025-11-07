//! Visualization module for path-finding algorithms
//!
//! This module provides interactive visualization using Macroquad for rendering
//! grids, paths, and algorithm execution in real-time.

pub mod grid;
pub mod path;
pub mod renderer;
pub mod ui;

// Re-export main visualization types
pub use grid::GridRenderer;
pub use path::PathRenderer;
pub use renderer::VisualizationRenderer;
pub use ui::{UIState, UIManager};

use macroquad::prelude::*;

/// Configuration for visualization settings
#[derive(Debug, Clone)]
pub struct VisualizationConfig {
    pub grid_size: (usize, usize),
    pub cell_size: f32,
    pub grid_color: Color,
    pub wall_color: Color,
    pub start_color: Color,
    pub end_color: Color,
    pub path_color: Color,
    pub visited_color: Color,
    pub exploring_color: Color,
    pub background_color: Color,
    pub show_grid_lines: bool,
    pub animation_speed: f32,
}

impl Default for VisualizationConfig {
    fn default() -> Self {
        Self {
            grid_size: (20, 20),
            cell_size: 30.0,
            grid_color: BLACK,
            wall_color: DARKGRAY,
            start_color: GREEN,
            end_color: RED,
            path_color: YELLOW,
            visited_color: LIGHTGRAY,
            exploring_color: BLUE,
            background_color: WHITE,
            show_grid_lines: true,
            animation_speed: 1.0,
        }
    }
}

/// Main visualization application
pub struct PathFindingVisualization {
    renderer: VisualizationRenderer,
    ui_manager: UIManager,
    config: VisualizationConfig,
}

impl PathFindingVisualization {
    pub fn new(config: VisualizationConfig) -> Self {
        Self {
            renderer: VisualizationRenderer::new(config.clone()),
            ui_manager: UIManager::new(),
            config,
        }
    }

    pub async fn run(&mut self) {
        loop {
            clear_background(self.config.background_color);

            // Handle input
            self.ui_manager.handle_input();

            // Update visualization
            self.renderer.update();

            // Render everything
            self.renderer.render();
            self.ui_manager.render();

            // Check for exit
            if is_key_pressed(KeyCode::Escape) {
                break;
            }

            next_frame().await;
        }
    }
}