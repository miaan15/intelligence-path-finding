//! Grid rendering utilities

use crate::data_structures::{Grid, Position};
use crate::visualization::VisualizationConfig;
use macroquad::prelude::*;

/// Renderer for grid visualization
pub struct GridRenderer {
    config: VisualizationConfig,
}

impl GridRenderer {
    pub fn new(config: VisualizationConfig) -> Self {
        Self { config }
    }

    /// Convert grid position to screen coordinates
    pub fn grid_to_screen(&self, position: &Position) -> Vec2 {
        Vec2::new(
            position.x as f32 * self.config.cell_size,
            position.y as f32 * self.config.cell_size,
        )
    }

    /// Convert screen coordinates to grid position
    pub fn screen_to_grid(&self, screen_pos: Vec2) -> Option<Position> {
        let grid_x = (screen_pos.x / self.config.cell_size) as usize;
        let grid_y = (screen_pos.y / self.config.cell_size) as usize;

        if grid_x < self.config.grid_size.0 && grid_y < self.config.grid_size.1 {
            Some(Position::new(grid_x, grid_y))
        } else {
            None
        }
    }

    /// Render the complete grid
    pub fn render_grid(&self, grid: &Grid) {
        let (width, height) = grid.dimensions();

        for y in 0..height {
            for x in 0..width {
                let position = Position::new(x, y);
                self.render_cell_base(grid, &position);
            }
        }

        // Draw grid lines if enabled
        if self.config.show_grid_lines {
            self.render_grid_lines();
        }
    }

    /// Render a single cell with its base color
    fn render_cell_base(&self, grid: &Grid, position: &Position) {
        let screen_pos = self.grid_to_screen(position);
        let color = if grid.is_walkable(position) {
            self.config.background_color
        } else {
            self.config.wall_color
        };

        draw_rectangle(
            screen_pos.x,
            screen_pos.y,
            self.config.cell_size,
            self.config.cell_size,
            color,
        );
    }

    /// Render a single cell with a specific color (overriding base color)
    pub fn render_cell(&self, grid: &Grid, position: &Position, color: Color) {
        if !grid.is_walkable(position) {
            return; // Don't render over walls
        }

        let screen_pos = self.grid_to_screen(position);
        draw_rectangle(
            screen_pos.x,
            screen_pos.y,
            self.config.cell_size,
            self.config.cell_size,
            color,
        );

        // Draw border for better visibility
        draw_rectangle_lines(
            screen_pos.x,
            screen_pos.y,
            self.config.cell_size,
            self.config.cell_size,
            1.0,
            self.config.grid_color,
        );
    }

    /// Render grid lines
    fn render_grid_lines(&self) {
        let grid_width = self.config.grid_size.0 as f32 * self.config.cell_size;
        let grid_height = self.config.grid_size.1 as f32 * self.config.cell_size;

        // Vertical lines
        for x in 0..=self.config.grid_size.0 {
            let screen_x = x as f32 * self.config.cell_size;
            draw_line(
                Vec2::new(screen_x, 0.0),
                Vec2::new(screen_x, grid_height),
                1.0,
                self.config.grid_color,
            );
        }

        // Horizontal lines
        for y in 0..=self.config.grid_size.1 {
            let screen_y = y as f32 * self.config.cell_size;
            draw_line(
                Vec2::new(0.0, screen_y),
                Vec2::new(grid_width, screen_y),
                1.0,
                self.config.grid_color,
            );
        }
    }

    /// Highlight a cell with animation effect
    pub fn highlight_cell(&self, position: &Position, color: Color, intensity: f32) {
        let screen_pos = self.grid_to_screen(position);
        let highlighted_color = Color::new(
            color.r,
            color.g,
            color.b,
            color.a * intensity,
        );

        draw_rectangle(
            screen_pos.x,
            screen_pos.y,
            self.config.cell_size,
            self.config.cell_size,
            highlighted_color,
        );
    }

    /// Render a circle in a cell (useful for start/end markers)
    pub fn render_circle_in_cell(&self, position: &Position, color: Color, scale: f32) {
        let screen_pos = self.grid_to_screen(position);
        let center = Vec2::new(
            screen_pos.x + self.config.cell_size / 2.0,
            screen_pos.y + self.config.cell_size / 2.0,
        );
        let radius = (self.config.cell_size / 2.0) * scale * 0.8;

        draw_circle(center.x, center.y, radius, color);
    }

    /// Render text in a cell
    pub fn render_text_in_cell(&self, position: &Position, text: &str, color: Color) {
        let screen_pos = self.grid_to_screen(position);
        let text_size = self.config.cell_size * 0.3;
        let text_x = screen_pos.x + self.config.cell_size / 2.0 - text_size * text.len() as f32 / 4.0;
        let text_y = screen_pos.y + self.config.cell_size / 2.0 + text_size / 3.0;

        draw_text(text, text_x, text_y, text_size, color);
    }

    /// Get the cell size
    pub fn cell_size(&self) -> f32 {
        self.config.cell_size
    }

    /// Get the grid dimensions
    pub fn grid_dimensions(&self) -> (usize, usize) {
        self.config.grid_size
    }
}