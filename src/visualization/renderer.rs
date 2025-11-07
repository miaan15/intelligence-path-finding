//! Main visualization renderer

use crate::data_structures::{Grid, Position, Path};
use crate::visualization::{grid::GridRenderer, path::PathRenderer, VisualizationConfig};
use macroquad::prelude::*;

/// Main renderer for path-finding visualization
pub struct VisualizationRenderer {
    grid_renderer: GridRenderer,
    path_renderer: PathRenderer,
    config: VisualizationConfig,
    grid: Option<Grid>,
    path: Option<Path>,
    start_pos: Option<Position>,
    end_pos: Option<Position>,
    visited_cells: Vec<Position>,
    exploring_cells: Vec<Position>,
}

impl VisualizationRenderer {
    pub fn new(config: VisualizationConfig) -> Self {
        Self {
            grid_renderer: GridRenderer::new(config.clone()),
            path_renderer: PathRenderer::new(config.clone()),
            config,
            grid: None,
            path: None,
            start_pos: None,
            end_pos: None,
            visited_cells: Vec::new(),
            exploring_cells: Vec::new(),
        }
    }

    /// Set the grid to visualize
    pub fn set_grid(&mut self, grid: Grid) {
        self.grid = Some(grid);
    }

    /// Set the path to visualize
    pub fn set_path(&mut self, path: Path) {
        self.path = Some(path);
    }

    /// Set start and end positions
    pub fn set_start_end(&mut self, start: Position, end: Position) {
        self.start_pos = Some(start);
        self.end_pos = Some(end);
    }

    /// Add a visited cell for animation
    pub fn add_visited_cell(&mut self, position: Position) {
        if !self.visited_cells.contains(&position) {
            self.visited_cells.push(position);
        }
    }

    /// Add cells currently being explored
    pub fn set_exploring_cells(&mut self, positions: Vec<Position>) {
        self.exploring_cells = positions;
    }

    /// Clear all visualization data
    pub fn clear(&mut self) {
        self.path = None;
        self.visited_cells.clear();
        self.exploring_cells.clear();
    }

    /// Update animation and state
    pub fn update(&mut self) {
        // Update animations here if needed
    }

    /// Render the complete visualization
    pub fn render(&self) {
        if let Some(grid) = &self.grid {
            // Draw grid
            self.grid_renderer.render_grid(grid);

            // Draw visited cells
            for position in &self.visited_cells {
                self.grid_renderer.render_cell(
                    grid,
                    position,
                    self.config.visited_color,
                );
            }

            // Draw currently exploring cells
            for position in &self.exploring_cells {
                self.grid_renderer.render_cell(
                    grid,
                    position,
                    self.config.exploring_color,
                );
            }

            // Draw start position
            if let Some(start_pos) = &self.start_pos {
                self.grid_renderer.render_cell(
                    grid,
                    start_pos,
                    self.config.start_color,
                );
            }

            // Draw end position
            if let Some(end_pos) = &self.end_pos {
                self.grid_renderer.render_cell(
                    grid,
                    end_pos,
                    self.config.end_color,
                );
            }

            // Draw path
            if let Some(path) = &self.path {
                self.path_renderer.render_path(grid, path);
            }
        }

        // Draw UI elements
        self.render_info();
    }

    /// Render information panel
    fn render_info(&self) {
        let info_x = self.config.grid_size.0 as f32 * self.config.cell_size + 20.0;
        let mut y = 20.0;

        draw_text("Path Finding Visualization", info_x, y, 20.0, BLACK);
        y += 30.0;

        if let Some(path) = &self.path {
            draw_text(&format!("Path Length: {}", path.len()), info_x, y, 16.0, BLACK);
            y += 20.0;
            draw_text(&format!("Path Cost: {:.2}", path.cost), info_x, y, 16.0, BLACK);
            y += 20.0;
        }

        draw_text(&format!("Visited Cells: {}", self.visited_cells.len()), info_x, y, 16.0, BLACK);
        y += 20.0;
        draw_text(&format!("Exploring: {}", self.exploring_cells.len()), info_x, y, 16.0, BLACK);
        y += 30.0;

        // Draw legend
        draw_text("Legend:", info_x, y, 16.0, BLACK);
        y += 20.0;

        self.draw_legend_item("Start", self.config.start_color, info_x, y);
        y += 20.0;
        self.draw_legend_item("End", self.config.end_color, info_x, y);
        y += 20.0;
        self.draw_legend_item("Path", self.config.path_color, info_x, y);
        y += 20.0;
        self.draw_legend_item("Visited", self.config.visited_color, info_x, y);
        y += 20.0;
        self.draw_legend_item("Exploring", self.config.exploring_color, info_x, y);
        y += 20.0;
        self.draw_legend_item("Wall", self.config.wall_color, info_x, y);

        // Instructions
        y += 30.0;
        draw_text("Controls:", info_x, y, 16.0, BLACK);
        y += 20.0;
        draw_text("Left Click: Set start", info_x, y, 14.0, DARKGRAY);
        y += 18.0;
        draw_text("Right Click: Set end", info_x, y, 14.0, DARKGRAY);
        y += 18.0;
        draw_text("Middle Click: Toggle wall", info_x, y, 14.0, DARKGRAY);
        y += 18.0;
        draw_text("Space: Run algorithm", info_x, y, 14.0, DARKGRAY);
        y += 18.0;
        draw_text("C: Clear path", info_x, y, 14.0, DARKGRAY);
        y += 18.0;
        draw_text("R: Reset all", info_x, y, 14.0, DARKGRAY);
        y += 18.0;
        draw_text("ESC: Exit", info_x, y, 14.0, DARKGRAY);
    }

    fn draw_legend_item(&self, label: &str, color: Color, x: f32, y: f32) {
        draw_rectangle(x, y - 10.0, 15.0, 15.0, color);
        draw_text(label, x + 20.0, y, 14.0, BLACK);
    }
}