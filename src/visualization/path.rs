//! Path rendering utilities

use crate::data_structures::{Grid, Path, Position};
use crate::visualization::VisualizationConfig;
use macroquad::prelude::*;

/// Renderer for path visualization
pub struct PathRenderer {
    config: VisualizationConfig,
}

impl PathRenderer {
    pub fn new(config: VisualizationConfig) -> Self {
        Self { config }
    }

    /// Render a complete path
    pub fn render_path(&self, grid: &Grid, path: &Path) {
        if path.is_empty() {
            return;
        }

        // Draw path cells
        for position in &path.positions {
            if grid.is_walkable(position) {
                self.render_path_cell(position);
            }
        }

        // Draw path connections
        self.render_path_connections(&path.positions);
    }

    /// Render a single path cell
    fn render_path_cell(&self, position: &Position) {
        let screen_pos = Vec2::new(
            position.x as f32 * self.config.cell_size,
            position.y as f32 * self.config.cell_size,
        );

        // Draw filled rectangle with slight transparency
        let path_color = Color::new(
            self.config.path_color.r,
            self.config.path_color.g,
            self.config.path_color.b,
            0.7,
        );

        draw_rectangle(
            screen_pos.x + 2.0,
            screen_pos.y + 2.0,
            self.config.cell_size - 4.0,
            self.config.cell_size - 4.0,
            path_color,
        );
    }

    /// Render connections between path nodes
    fn render_path_connections(&self, positions: &[Position]) {
        if positions.len() < 2 {
            return;
        }

        for window in positions.windows(2) {
            let pos1 = &window[0];
            let pos2 = &window[1];

            let screen_pos1 = Vec2::new(
                pos1.x as f32 * self.config.cell_size + self.config.cell_size / 2.0,
                pos1.y as f32 * self.config.cell_size + self.config.cell_size / 2.0,
            );

            let screen_pos2 = Vec2::new(
                pos2.x as f32 * self.config.cell_size + self.config.cell_size / 2.0,
                pos2.y as f32 * self.config.cell_size + self.config.cell_size / 2.0,
            );

            draw_line(
                screen_pos1,
                screen_pos2,
                3.0,
                self.config.path_color,
            );
        }
    }

    /// Render an animated path (for visualization during algorithm execution)
    pub fn render_animated_path(&self, positions: &[Position], progress: f32) {
        if positions.is_empty() {
            return;
        }

        let num_cells = (positions.len() as f32 * progress).ceil() as usize;
        let animated_positions = &positions[..num_cells.min(positions.len())];

        for (i, position) in animated_positions.iter().enumerate() {
            let intensity = 1.0 - (i as f32 / positions.len() as f32) * 0.5;
            let color = Color::new(
                self.config.path_color.r,
                self.config.path_color.g,
                self.config.path_color.b,
                self.config.path_color.a * intensity,
            );

            let screen_pos = Vec2::new(
                position.x as f32 * self.config.cell_size,
                position.y as f32 * self.config.cell_size,
            );

            draw_rectangle(
                screen_pos.x + 4.0,
                screen_pos.y + 4.0,
                self.config.cell_size - 8.0,
                self.config.cell_size - 8.0,
                color,
            );
        }
    }

    /// Render path with directional arrows
    pub fn render_path_with_directions(&self, positions: &[Position]) {
        if positions.len() < 2 {
            return;
        }

        // Render path cells
        for position in positions {
            self.render_path_cell(position);
        }

        // Render directional arrows
        for window in positions.windows(2) {
            let from = &window[0];
            let to = &window[1];
            self.render_direction_arrow(from, to);
        }
    }

    /// Render a directional arrow between two positions
    fn render_direction_arrow(&self, from: &Position, to: &Position) {
        let from_screen = Vec2::new(
            from.x as f32 * self.config.cell_size + self.config.cell_size / 2.0,
            from.y as f32 * self.config.cell_size + self.config.cell_size / 2.0,
        );

        let to_screen = Vec2::new(
            to.x as f32 * self.config.cell_size + self.config.cell_size / 2.0,
            to.y as f32 * self.config.cell_size + self.config.cell_size / 2.0,
        );

        // Calculate arrow direction
        let direction = (to_screen - from_screen).normalize();
        let arrow_length = self.config.cell_size * 0.3;
        let arrow_head_length = self.config.cell_size * 0.15;

        // Draw arrow line
        let arrow_end = from_screen + direction * arrow_length;
        draw_line(from_screen, arrow_end, 2.0, self.config.path_color);

        // Draw arrow head
        let perpendicular = Vec2::new(-direction.y, direction.x);
        let arrow_head_base = arrow_end - direction * arrow_head_length;
        let arrow_head_left = arrow_head_base + perpendicular * arrow_head_length * 0.5;
        let arrow_head_right = arrow_head_base - perpendicular * arrow_head_length * 0.5;

        draw_triangle(
            arrow_end,
            arrow_head_left,
            arrow_head_right,
            self.config.path_color,
        );
    }

    /// Render path cost indicators
    pub fn render_cost_indicators(&self, grid: &Grid, path: &Path) {
        if path.is_empty() {
            return;
        }

        for position in &path.positions {
            if grid.is_walkable(position) {
                // For now, we'll just show the position index
                // In a real implementation, you might want to show actual costs
                let screen_pos = Vec2::new(
                    position.x as f32 * self.config.cell_size,
                    position.y as f32 * self.config.cell_size,
                );

                // You could add cost text here if needed
                // draw_text(&format!("{:.1}", cost), screen_pos.x, screen_pos.y, 10.0, BLACK);
            }
        }
    }
}